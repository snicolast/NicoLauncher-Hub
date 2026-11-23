//! The messages, one JSON file each in data/messages, and the blocked senders
//! in data/blocked.json. Plain files: they can be read, copied or deleted by
//! hand, and a few thousand messages are no work to list.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// What a launcher sends (docs/feedback.md).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incoming {
    /// The launcher's own random id, made once per install: groups and
    /// blocks one sender without knowing who they are.
    pub sender: String,
    pub challenge: String,
    pub nonce: u64,
    /// idea, recomp or talk.
    pub kind: String,
    /// A project id, "launcher", or nothing (recomp proposals).
    #[serde(default)]
    pub about: Option<String>,
    pub title: String,
    /// For a recomp proposal: the original system.
    #[serde(default)]
    pub system: Option<String>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub contact: Option<String>,
    /// [label, value] rows: launcher version, game version, Windows, graphics.
    #[serde(default)]
    pub details: Vec<(String, String)>,
    /// The launcher's language.
    #[serde(default)]
    pub lang: Option<String>,
    /// When it was written (Unix seconds): later than `received` when the
    /// launcher kept it while the mailbox did not answer.
    #[serde(default)]
    pub written: Option<u64>,
}

pub const KINDS: [&str; 3] = ["idea", "recomp", "talk"];

impl Incoming {
    /// Sizes the launcher's form allows, with room to spare.
    pub fn check(&self) -> Result<(), &'static str> {
        let short = |s: &str| s.chars().count() <= 200;
        if !KINDS.contains(&self.kind.as_str()) {
            return Err("unknown kind");
        }
        if self.title.trim().is_empty() {
            return Err("no title");
        }
        if !short(&self.title) || !short(&self.sender) || !short(&self.challenge) {
            return Err("too long");
        }
        if [&self.about, &self.system, &self.contact, &self.lang].iter().any(|o| o.as_deref().is_some_and(|s| !short(s))) {
            return Err("too long");
        }
        if self.text.chars().count() > 5000 {
            return Err("text too long");
        }
        if self.details.len() > 12 || self.details.iter().any(|(k, v)| !short(k) || !short(v)) {
            return Err("too many details");
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stored {
    pub id: String,
    /// Unix seconds.
    pub received: u64,
    /// The sender's address, as far as it is known.
    pub address: String,
    #[serde(flatten)]
    pub message: Incoming,
    /// Why it may need a second look: links, ROMs...
    #[serde(default)]
    pub flags: Vec<String>,
    #[serde(default)]
    pub read: bool,
    #[serde(default)]
    pub archived: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Blocked {
    /// Sender ids and addresses. Their messages are answered as sent, and dropped.
    pub keys: Vec<String>,
}

pub struct Store {
    dir: PathBuf,
}

impl Store {
    pub fn open(data: &Path) -> std::io::Result<Store> {
        std::fs::create_dir_all(data.join("messages"))?;
        Ok(Store { dir: data.to_path_buf() })
    }

    fn path(&self, id: &str) -> Option<PathBuf> {
        // Ids come back from the panel's forms: only the ones made here
        (!id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'))
            .then(|| self.dir.join("messages").join(format!("{id}.json")))
    }

    pub fn save(&self, m: &Stored) -> std::io::Result<()> {
        let path = self.path(&m.id).ok_or_else(|| std::io::Error::other("bad id"))?;
        let tmp = path.with_extension("tmp");
        std::fs::write(&tmp, serde_json::to_vec_pretty(m)?)?;
        std::fs::rename(tmp, path)
    }

    pub fn get(&self, id: &str) -> Option<Stored> {
        serde_json::from_slice(&std::fs::read(self.path(id)?).ok()?).ok()
    }

    pub fn delete(&self, id: &str) {
        if let Some(p) = self.path(id) {
            let _ = std::fs::remove_file(p);
        }
    }

    /// Newest first.
    pub fn all(&self) -> Vec<Stored> {
        let mut list: Vec<Stored> = std::fs::read_dir(self.dir.join("messages"))
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
            .filter_map(|e| serde_json::from_slice(&std::fs::read(e.path()).ok()?).ok())
            .collect();
        list.sort_by(|a: &Stored, b| b.received.cmp(&a.received).then_with(|| b.id.cmp(&a.id)));
        list
    }

    pub fn blocked(&self) -> Blocked {
        std::fs::read(self.dir.join("blocked.json")).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
    }

    pub fn set_blocked(&self, b: &Blocked) -> std::io::Result<()> {
        std::fs::write(self.dir.join("blocked.json"), serde_json::to_vec_pretty(b)?)
    }

    pub fn log(&self, line: &str) {
        use std::io::Write;
        let line = format!("{} {line}\n", crate::now());
        eprint!("{line}");
        if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(self.dir.join("inbox.log")) {
            let _ = f.write_all(line.as_bytes());
        }
    }
}

/// What may need a second look before trusting a message.
pub fn flags(m: &Incoming) -> Vec<String> {
    let all = format!("{} {} {} {}", m.title, m.text, m.system.as_deref().unwrap_or(""), m.contact.as_deref().unwrap_or("")).to_lowercase();
    let mut out = Vec::new();
    if ["http://", "https://", "www.", ".com/", "magnet:"].iter().any(|w| all.contains(w)) {
        out.push("link".to_string());
    }
    let words: Vec<&str> = all.split(|c: char| !c.is_alphanumeric()).collect();
    if words.iter().any(|w| ["rom", "roms", "romset", "chd", "mame", "torrent", "mega", "mediafire"].contains(w)) || all.contains(".zip") {
        out.push("roms?".to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(title: &str, text: &str) -> Incoming {
        Incoming {
            sender: "a".into(), challenge: "c".into(), nonce: 0, kind: "idea".into(), about: None,
            title: title.into(), system: None, text: text.into(), contact: None, details: vec![], lang: None, written: None,
        }
    }

    #[test]
    fn flags_links_and_roms() {
        assert!(flags(&msg("Nice camera", "Please add it")).is_empty());
        assert_eq!(flags(&msg("Get it", "https://example.com/x")), vec!["link"]);
        assert_eq!(flags(&msg("Where is the ROM", "romset.zip")), vec!["roms?"]);
        // A word that only contains "rom" is fine
        assert!(flags(&msg("From the menu", "Promote the brake")).is_empty());
    }

    #[test]
    fn sizes_are_checked() {
        assert!(msg("ok", "ok").check().is_ok());
        assert!(msg("", "ok").check().is_err());
        assert!(msg(&"x".repeat(300), "ok").check().is_err());
        let mut m = msg("ok", "ok");
        m.kind = "bug".into();
        assert!(m.check().is_err());
    }

    #[test]
    fn saves_lists_and_refuses_odd_ids() {
        let dir = std::env::temp_dir().join(format!("nico-inbox-test-{}", std::process::id()));
        let store = Store::open(&dir).unwrap();
        for (i, id) in ["100-a", "300-b", "200-c"].iter().enumerate() {
            let m = Stored { id: id.to_string(), received: [100, 300, 200][i], address: "x".into(), message: msg("t", "x"), flags: vec![], read: false, archived: false };
            store.save(&m).unwrap();
        }
        let ids: Vec<String> = store.all().into_iter().map(|m| m.id).collect();
        assert_eq!(ids, ["300-b", "200-c", "100-a"]);
        assert!(store.get("../config").is_none());
        store.delete("300-b");
        assert_eq!(store.all().len(), 2);
        let _ = std::fs::remove_dir_all(dir);
    }
}
