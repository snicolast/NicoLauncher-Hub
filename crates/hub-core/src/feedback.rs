//! Ideas, recomp proposals and messages for the author, sent from the
//! launcher to his private mailbox (crates/nico-inbox; protocol in
//! docs/feedback.md). Problems are not sent from here: they are issues on
//! GitHub, opened in the browser.
//!
//! The launcher is open source, so it carries no secret. Before each message
//! it works out a small sum the mailbox asks for (about a second), which keeps
//! sending thousands of messages slow. When the mailbox does not answer, the
//! message waits in `feedback.json`, next to the settings, and goes on a later
//! start or Refresh.

use crate::{agent, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Messages kept longer than this without reaching the mailbox are dropped.
const KEEP_SECS: u64 = 30 * 24 * 3600;
/// Messages kept at most, waiting.
const KEEP_MAX: usize = 20;
/// A harder sum than this is not worked out: it would hang the launcher.
const BITS_MAX: u32 = 26;

/// What the user wrote, and what goes with it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// idea, recomp or talk.
    pub kind: String,
    /// A project id, "launcher", or nothing (recomp proposals).
    #[serde(default)]
    pub about: Option<String>,
    pub title: String,
    #[serde(default)]
    pub system: Option<String>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub contact: Option<String>,
    /// [label, value] rows, in English: the versions, Windows, graphics card.
    #[serde(default)]
    pub details: Vec<(String, String)>,
    #[serde(default)]
    pub lang: Option<String>,
    /// When it was written, Unix seconds.
    #[serde(default)]
    pub written: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Sent,
    /// Kept, to send later.
    Queued,
    /// Too many messages from this PC for now: nothing kept, the form stays.
    Limited,
}

/// `feedback.json`: this install's random id, and what waits to be sent.
#[derive(Debug, Default, Serialize, Deserialize)]
struct File {
    #[serde(default)]
    sender: String,
    #[serde(default)]
    outbox: Vec<Message>,
}

enum Failure {
    /// No answer, or the mailbox could not take it now: keep it.
    Unreachable(String),
    Limited,
    /// The mailbox refused this message: keeping it would not help.
    Refused(String),
}

/// One send or flush at a time: both rewrite the file.
static BUSY: Mutex<()> = Mutex::new(());

pub fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// The file next to the settings file.
pub fn file_next_to(config: &Path) -> PathBuf {
    config.with_file_name("feedback.json")
}

fn read(path: &Path) -> File {
    std::fs::read(path).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
}

fn write(path: &Path, file: &File) {
    if let Ok(bytes) = serde_json::to_vec_pretty(file) {
        let tmp = path.with_extension("tmp");
        if std::fs::write(&tmp, bytes).is_ok() {
            let _ = std::fs::rename(tmp, path);
        }
    }
}

/// This install's id: random, made once. It lets the author tell messages
/// from one PC apart and block one that sends spam, without knowing who it is.
fn sender(path: &Path, file: &mut File) -> String {
    if file.sender.len() != 32 {
        let mut bytes = [0u8; 16];
        let _ = getrandom::getrandom(&mut bytes);
        file.sender = bytes.iter().map(|b| format!("{b:02x}")).collect();
        write(path, file);
    }
    file.sender.clone()
}

/// How many messages wait to be sent.
pub fn waiting(path: &Path) -> usize {
    read(path).outbox.len()
}

/// Leading zero bits of SHA-256("{challenge}:{nonce}"), as nico-inbox counts them.
fn zero_bits(challenge: &str, nonce: u64) -> u32 {
    let hash = Sha256::digest(format!("{challenge}:{nonce}").as_bytes());
    let mut bits = 0;
    for byte in hash {
        if byte == 0 {
            bits += 8;
        } else {
            bits += byte.leading_zeros();
            break;
        }
    }
    bits
}

fn solve(challenge: &str, bits: u32) -> u64 {
    (0..).find(|n| zero_bits(challenge, *n) >= bits).unwrap_or(0)
}

#[derive(Deserialize)]
struct Challenge {
    challenge: String,
    bits: u32,
}

#[derive(Deserialize)]
struct Answer {
    #[serde(default)]
    status: String,
    #[serde(default)]
    error: Option<String>,
}

fn failure(url: &str, e: ureq::Error) -> Failure {
    match e {
        ureq::Error::Status(429, _) => Failure::Limited,
        ureq::Error::Status(403, r) => {
            let a: Option<Answer> = r.into_json().ok();
            match a {
                Some(a) if a.status == "retry" => Failure::Unreachable("challenge".into()),
                Some(a) => Failure::Refused(a.error.unwrap_or_else(|| "refused".into())),
                None => Failure::Refused("refused".into()),
            }
        }
        ureq::Error::Status(code @ 400..=499, r) => {
            let why = r.into_json::<Answer>().ok().and_then(|a| a.error).unwrap_or_else(|| format!("answered {code}"));
            Failure::Refused(why)
        }
        ureq::Error::Status(code, _) => Failure::Unreachable(format!("{url} answered {code}")),
        other => Failure::Unreachable(format!("could not reach the mailbox: {other}")),
    }
}

fn send_one(inbox: &str, sender: &str, m: &Message) -> std::result::Result<(), Failure> {
    let inbox = inbox.trim_end_matches('/');
    let a = agent(30);
    // A challenge that ran out (the mailbox restarted) is asked for again, once
    for attempt in 0..2 {
        let url = format!("{inbox}/v1/challenge");
        let c: Challenge = a.get(&url).call().map_err(|e| failure(&url, e))?
            .into_json().map_err(|e| Failure::Unreachable(e.to_string()))?;
        if c.bits > BITS_MAX {
            return Err(Failure::Unreachable("the mailbox asks for too much work".into()));
        }
        let nonce = solve(&c.challenge, c.bits);
        let mut body = serde_json::to_value(m).map_err(|e| Failure::Refused(e.to_string()))?;
        body["sender"] = sender.into();
        body["challenge"] = c.challenge.into();
        body["nonce"] = nonce.into();
        let url = format!("{inbox}/v1/message");
        match a.post(&url).send_json(body) {
            Ok(_) => return Ok(()),
            Err(e) => match failure(&url, e) {
                Failure::Unreachable(why) if why == "challenge" && attempt == 0 => continue,
                f => return Err(f),
            },
        }
    }
    Err(Failure::Unreachable("challenge".into()))
}

/// Sends `m` to the mailbox at `inbox`, or keeps it in `path` for later when
/// the mailbox does not answer.
pub fn send(inbox: &str, path: &Path, mut m: Message) -> Result<Outcome> {
    let _busy = BUSY.lock().unwrap_or_else(|e| e.into_inner());
    if m.title.trim().is_empty() {
        return Err("A title is needed.".into());
    }
    if m.written == 0 {
        m.written = now();
    }
    let mut file = read(path);
    let sender = sender(path, &mut file);
    match send_one(inbox, &sender, &m) {
        Ok(()) => Ok(Outcome::Sent),
        Err(Failure::Limited) => Ok(Outcome::Limited),
        Err(Failure::Refused(why)) => Err(why),
        Err(Failure::Unreachable(_)) => {
            let mut file = read(path);
            file.outbox.push(m);
            let extra = file.outbox.len().saturating_sub(KEEP_MAX);
            file.outbox.drain(..extra);
            write(path, &file);
            Ok(Outcome::Queued)
        }
    }
}

/// Sends what waits, oldest first, until the mailbox stops answering.
/// Returns how many went.
pub fn flush(inbox: &str, path: &Path) -> usize {
    let _busy = BUSY.lock().unwrap_or_else(|e| e.into_inner());
    let mut file = read(path);
    if file.outbox.is_empty() {
        return 0;
    }
    let sender = sender(path, &mut file);
    let t = now();
    let before = file.outbox.len();
    file.outbox.retain(|m| t.saturating_sub(m.written) < KEEP_SECS);
    let mut sent = 0;
    while let Some(m) = file.outbox.first() {
        match send_one(inbox, &sender, m) {
            // A refused one would be refused again: it goes too
            Ok(()) | Err(Failure::Refused(_)) => {
                file.outbox.remove(0);
                sent += 1;
            }
            Err(Failure::Limited) | Err(Failure::Unreachable(_)) => break,
        }
    }
    if sent > 0 || file.outbox.len() != before {
        write(path, &file);
    }
    sent
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::TcpListener;

    // nico-inbox's test checks the same numbers (crates/nico-inbox/src/pow.rs)
    #[test]
    fn the_shared_sum() {
        assert_eq!(solve("nico", 12), 5238);
        assert!(zero_bits("nico", 5238) >= 12);
    }

    fn message(title: &str) -> Message {
        Message {
            kind: "idea".into(), about: Some("launcher".into()), title: title.into(), system: None, text: "Text".into(),
            contact: None, details: vec![("Launcher".into(), "v0".into())], lang: Some("en".into()), written: 0,
        }
    }

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("hub-feedback-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir.join("feedback.json")
    }

    /// A mailbox on this machine that answers each request with the next of
    /// `answers` (status, body), and hands back what it was sent.
    fn mailbox(answers: Vec<(u16, &'static str)>) -> (String, std::thread::JoinHandle<Vec<String>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let handle = std::thread::spawn(move || {
            let mut seen = Vec::new();
            for (code, body) in answers {
                let (mut stream, _) = listener.accept().unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut first = String::new();
                reader.read_line(&mut first).unwrap();
                let mut len = 0;
                loop {
                    let mut line = String::new();
                    reader.read_line(&mut line).unwrap();
                    if line.trim().is_empty() {
                        break;
                    }
                    if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                        len = v.trim().parse().unwrap();
                    }
                }
                let mut payload = vec![0; len];
                reader.read_exact(&mut payload).unwrap();
                seen.push(format!("{}{}", first.trim(), String::from_utf8_lossy(&payload)));
                write!(stream, "HTTP/1.1 {code} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
            }
            seen
        });
        (url, handle)
    }

    const CHALLENGE: &str = r#"{"challenge":"abc","bits":8}"#;
    const SENT: &str = r#"{"status":"sent"}"#;

    #[test]
    fn sends_with_the_sum_worked_out() {
        let path = temp("sends");
        let (url, server) = mailbox(vec![(200, CHALLENGE), (200, SENT)]);
        assert_eq!(send(&url, &path, message("Hello")).unwrap(), Outcome::Sent);
        let seen = server.join().unwrap();
        assert!(seen[0].starts_with("GET /v1/challenge"));
        assert!(seen[1].starts_with("POST /v1/message"));
        let body: serde_json::Value = serde_json::from_str(&seen[1][seen[1].find('{').unwrap()..]).unwrap();
        let nonce = body["nonce"].as_u64().unwrap();
        assert!(zero_bits("abc", nonce) >= 8);
        assert_eq!(body["title"], "Hello");
        assert_eq!(body["sender"].as_str().unwrap().len(), 32);
        assert!(body["written"].as_u64().unwrap() > 0);
        assert_eq!(waiting(&path), 0);
    }

    #[test]
    fn keeps_it_when_the_mailbox_is_away_and_sends_it_later() {
        let path = temp("later");
        // Nothing listens there
        let away = {
            let l = TcpListener::bind("127.0.0.1:0").unwrap();
            format!("http://{}", l.local_addr().unwrap())
        };
        assert_eq!(send(&away, &path, message("One")).unwrap(), Outcome::Queued);
        assert_eq!(send(&away, &path, message("Two")).unwrap(), Outcome::Queued);
        assert_eq!(waiting(&path), 2);
        let (url, server) = mailbox(vec![(200, CHALLENGE), (200, SENT), (200, CHALLENGE), (503, r#"{"status":"error"}"#)]);
        assert_eq!(flush(&url, &path), 1);
        let seen = server.join().unwrap();
        assert!(seen[1].contains("\"One\""));
        assert_eq!(waiting(&path), 1, "the second waits for the next time");
    }

    #[test]
    fn limited_and_refused_are_not_kept() {
        let path = temp("limited");
        let (url, server) = mailbox(vec![(200, CHALLENGE), (429, r#"{"status":"limited"}"#)]);
        assert_eq!(send(&url, &path, message("x")).unwrap(), Outcome::Limited);
        server.join().unwrap();
        let (url, server) = mailbox(vec![(200, CHALLENGE), (400, r#"{"status":"error","error":"too long"}"#)]);
        assert_eq!(send(&url, &path, message("x")).unwrap_err(), "too long");
        server.join().unwrap();
        assert_eq!(waiting(&path), 0);
    }

    #[test]
    fn asks_again_for_a_challenge_that_ran_out() {
        let path = temp("retry");
        let (url, server) = mailbox(vec![(200, CHALLENGE), (403, r#"{"status":"retry"}"#), (200, CHALLENGE), (200, SENT)]);
        assert_eq!(send(&url, &path, message("x")).unwrap(), Outcome::Sent);
        assert_eq!(server.join().unwrap().len(), 4);
    }
}
