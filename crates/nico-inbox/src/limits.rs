//! How many messages each sender may send, and the challenges handed out. In
//! memory: a restart forgets them, which only ever lets a little more through.

use std::collections::HashMap;

const HOUR: u64 = 3600;
const DAY: u64 = 24 * HOUR;
/// A challenge must be answered within this time.
pub const CHALLENGE_TTL: u64 = 15 * 60;
/// Challenges one address may ask for in an hour, and kept for everyone at once.
const CHALLENGES_PER_HOUR: usize = 40;
const CHALLENGES_MAX: usize = 20_000;
/// Times one address may ask for its answers in an hour.
const LOOKUPS_PER_HOUR: usize = 60;
/// Addresses and senders remembered at once. Past this, the ones with
/// nothing recent are forgotten; if that is not enough, new ones wait.
const KEYS_MAX: usize = 20_000;

#[derive(Default)]
pub struct Limiter {
    /// When each sender (id or address) had a message kept.
    sent: HashMap<String, Vec<u64>>,
    all: Vec<u64>,
    /// Open challenges: when each was given, and to which address.
    challenges: HashMap<String, (u64, String)>,
    asked: HashMap<String, Vec<u64>>,
}

fn recent(list: &mut Vec<u64>, now: u64, window: u64) -> usize {
    list.retain(|t| now.saturating_sub(*t) < DAY.max(window));
    list.iter().filter(|t| now.saturating_sub(**t) < window).count()
}

impl Limiter {
    /// A new challenge for `address`, or None when it asked for too many.
    pub fn challenge(&mut self, address: &str, now: u64, token: String) -> Option<String> {
        if !self.room(address, now) {
            return None;
        }
        let asked = self.asked.entry(address.to_string()).or_default();
        if recent(asked, now, HOUR) >= CHALLENGES_PER_HOUR {
            return None;
        }
        asked.push(now);
        self.challenges.retain(|_, (t, _)| now.saturating_sub(*t) < CHALLENGE_TTL);
        if self.challenges.len() >= CHALLENGES_MAX {
            return None;
        }
        self.challenges.insert(token.clone(), (now, address.to_string()));
        Some(token)
    }

    /// Whether `address` may ask for its answers once more: a launcher asks
    /// when it starts and on Refresh, far less than this.
    pub fn lookup(&mut self, address: &str, now: u64) -> bool {
        let key = format!("replies {address}");
        if !self.room(&key, now) {
            return false;
        }
        let asked = self.asked.entry(key).or_default();
        if recent(asked, now, HOUR) >= LOOKUPS_PER_HOUR {
            return false;
        }
        asked.push(now);
        true
    }

    /// Whether `key` can be remembered: it already is, or there is room, after
    /// forgetting what is old if needed. Keeps memory bounded whatever comes in.
    fn room(&mut self, key: &str, now: u64) -> bool {
        if self.asked.contains_key(key) || self.asked.len() + self.sent.len() < KEYS_MAX {
            return true;
        }
        self.tidy(now);
        self.asked.len() + self.sent.len() < KEYS_MAX
    }

    fn tidy(&mut self, now: u64) {
        // Senders with nothing in the last day are forgotten
        self.sent.retain(|_, v| v.iter().any(|t| now.saturating_sub(*t) < DAY));
        self.asked.retain(|_, v| v.iter().any(|t| now.saturating_sub(*t) < HOUR));
    }

    /// Takes a challenge back: each one is good for one message.
    pub fn redeem(&mut self, token: &str, now: u64) -> bool {
        matches!(self.challenges.remove(token), Some((t, _)) if now.saturating_sub(t) < CHALLENGE_TTL)
    }

    /// Whether one more message from these senders fits, without counting it.
    pub fn allows(&mut self, keys: &[&str], now: u64, l: &crate::config::Limits) -> bool {
        if recent(&mut self.all, now, DAY) >= l.global_per_day as usize {
            return false;
        }
        keys.iter().all(|k| {
            let Some(list) = self.sent.get_mut(*k) else {
                return true;
            };
            let day = recent(list, now, DAY);
            let hour = list.iter().filter(|t| now.saturating_sub(**t) < HOUR).count();
            hour < l.per_hour as usize && day < l.per_day as usize
        })
    }

    pub fn count(&mut self, keys: &[&str], now: u64) {
        self.all.push(now);
        for k in keys {
            self.sent.entry(k.to_string()).or_default().push(now);
        }
        self.tidy(now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Limits;

    #[test]
    fn per_sender_and_global() {
        let l = Limits { per_hour: 2, per_day: 3, global_per_day: 5 };
        let mut lim = Limiter::default();
        let t = 1_000_000;
        assert!(lim.allows(&["a", "1.1.1.1"], t, &l));
        lim.count(&["a", "1.1.1.1"], t);
        lim.count(&["a", "1.1.1.1"], t + 1);
        assert!(!lim.allows(&["a", "1.1.1.1"], t + 2, &l), "two in an hour");
        assert!(!lim.allows(&["b", "1.1.1.1"], t + 2, &l), "same address, other launcher");
        assert!(lim.allows(&["a", "1.1.1.1"], t + HOUR + 1, &l), "next hour");
        lim.count(&["a", "1.1.1.1"], t + HOUR + 1);
        assert!(!lim.allows(&["a", "1.1.1.1"], t + 2 * HOUR + 2, &l), "three in a day");
        assert!(lim.allows(&["a", "1.1.1.1"], t + DAY + 2, &l), "next day");
        let mut lim = Limiter::default();
        for (i, k) in ["a", "b", "c", "d", "e"].iter().enumerate() {
            assert!(lim.allows(&[k], t + i as u64, &l));
            lim.count(&[k], t + i as u64);
        }
        assert!(!lim.allows(&["f"], t + 6, &l), "five from everyone");
    }

    #[test]
    fn challenges_are_single_use_and_expire() {
        let mut lim = Limiter::default();
        let c = lim.challenge("x", 100, "tok".into()).unwrap();
        assert!(lim.redeem(&c, 101));
        assert!(!lim.redeem(&c, 102));
        let c = lim.challenge("x", 100, "tok2".into()).unwrap();
        assert!(!lim.redeem(&c, 100 + CHALLENGE_TTL));
        for i in 0..40 {
            lim.challenge("y", 200, format!("t{i}"));
        }
        assert!(lim.challenge("y", 200, "more".into()).is_none());
        assert!(lim.challenge("z", 200, "other".into()).is_some());
    }

    #[test]
    fn memory_stays_bounded() {
        let mut lim = Limiter::default();
        for i in 0..KEYS_MAX + 500 {
            lim.lookup(&format!("10.0.{}.{}", i / 256, i % 256), 1000);
        }
        assert!(lim.asked.len() <= KEYS_MAX);
        assert!(!lim.lookup("new", 1001), "full of recent ones: new ones wait");
        assert!(lim.lookup("10.0.0.1", 1001), "known ones still count");
        assert!(lim.lookup("new", 1000 + HOUR + 1), "old ones are forgotten");
        assert_eq!(lim.asked.len(), 1);
    }
}
