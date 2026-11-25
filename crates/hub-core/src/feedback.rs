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
//!
//! The same file keeps a copy of what was sent (title and text, not the
//! details), and the author's answers: the launcher asks the mailbox for them
//! with its random sender id, which only it and the mailbox know.

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
/// Copies of sent messages kept. Past it the oldest go, except those with an
/// answer not read yet.
const SENT_MAX: usize = 50;
/// Longest answer taken from the mailbox.
const REPLY_MAX: usize = 5000;

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
    /// This launcher's id for the message, random: the mailbox hands the
    /// author's answer back with it. Made when it is sent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_id: Option<String>,
}

/// A copy of a message sent from here, and the author's answer to it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sent {
    /// The message's `local_id`.
    pub id: String,
    /// The mailbox's id, once an answer came with it: a reply to the answer
    /// names it.
    #[serde(default)]
    pub inbox_id: Option<String>,
    pub kind: String,
    #[serde(default)]
    pub about: Option<String>,
    pub title: String,
    #[serde(default)]
    pub text: String,
    pub written: u64,
    /// sent, or queued while it waits for the mailbox.
    pub status: String,
    #[serde(default)]
    pub reply: Option<Answer>,
    /// The user opened the answer.
    #[serde(default)]
    pub reply_seen: bool,
    /// The emoji the user sent back, one of REACTIONS.
    #[serde(default)]
    pub reaction: Option<String>,
    /// The mailbox knows the answer was read, with this reaction. False
    /// while telling it waits for the mailbox.
    #[serde(default)]
    pub read_told: bool,
}

/// What a user may send back to an answer (nico-inbox takes only these).
pub const REACTIONS: [&str; 6] = ["👍", "❤️", "😄", "🙏", "🎉", "🤔"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Answer {
    pub text: String,
    /// When the author wrote it, Unix seconds.
    pub at: u64,
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
    #[serde(default)]
    sent: Vec<Sent>,
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

fn random_hex(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    let _ = getrandom::getrandom(&mut buf);
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

/// Keeps a copy of `m`, or updates the one there.
fn keep_copy(file: &mut File, m: &Message, status: &str) {
    let Some(id) = m.local_id.clone() else { return };
    if let Some(c) = file.sent.iter_mut().find(|c| c.id == id) {
        c.status = status.to_string();
        return;
    }
    file.sent.push(Sent {
        id,
        inbox_id: None,
        kind: m.kind.clone(),
        about: m.about.clone(),
        title: m.title.clone(),
        text: m.text.clone(),
        written: m.written,
        status: status.to_string(),
        reply: None,
        reply_seen: false,
        reaction: None,
        read_told: false,
    });
    trim_sent(&mut file.sent);
}

/// Oldest first out, but never an answer the user has not read.
fn trim_sent(list: &mut Vec<Sent>) {
    list.sort_by_key(|c| c.written);
    while list.len() > SENT_MAX {
        match list.iter().position(|c| c.reply.is_none() || c.reply_seen) {
            Some(i) => {
                list.remove(i);
            }
            None => break,
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
struct Status {
    #[serde(default)]
    status: String,
    #[serde(default)]
    error: Option<String>,
}

fn failure(url: &str, e: ureq::Error) -> Failure {
    match e {
        ureq::Error::Status(429, _) => Failure::Limited,
        ureq::Error::Status(403, r) => {
            let a: Option<Status> = r.into_json().ok();
            match a {
                Some(a) if a.status == "retry" => Failure::Unreachable("challenge".into()),
                Some(a) => Failure::Refused(a.error.unwrap_or_else(|| "refused".into())),
                None => Failure::Refused("refused".into()),
            }
        }
        ureq::Error::Status(code @ 400..=499, r) => {
            let why = r.into_json::<Status>().ok().and_then(|a| a.error).unwrap_or_else(|| format!("answered {code}"));
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
    if m.local_id.is_none() {
        m.local_id = Some(random_hex(8));
    }
    let mut file = read(path);
    let sender = sender(path, &mut file);
    match send_one(inbox, &sender, &m) {
        Ok(()) => {
            let mut file = read(path);
            keep_copy(&mut file, &m, "sent");
            write(path, &file);
            Ok(Outcome::Sent)
        }
        Err(Failure::Limited) => Ok(Outcome::Limited),
        Err(Failure::Refused(why)) => Err(why),
        Err(Failure::Unreachable(_)) => {
            let mut file = read(path);
            keep_copy(&mut file, &m, "queued");
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
                let m = file.outbox.remove(0);
                keep_copy(&mut file, &m, "sent");
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

/// The copies of what was sent, newest first.
pub fn sent(path: &Path) -> Vec<Sent> {
    let mut list = read(path).sent;
    list.reverse();
    list
}

#[derive(Deserialize)]
struct Replies {
    #[serde(default)]
    replies: Vec<Replied>,
}

#[derive(Deserialize)]
struct Replied {
    id: String,
    #[serde(default)]
    local_id: Option<String>,
    #[serde(default)]
    kind: String,
    #[serde(default)]
    about: Option<String>,
    #[serde(default)]
    title: String,
    #[serde(default)]
    text: String,
    #[serde(default)]
    written: u64,
    reply: Answer,
}

/// Asks the mailbox for the author's answers to this install's messages and
/// keeps them. Returns how many are new or changed (and so not read yet).
/// An install that never sent anything has no id yet and asks nothing.
pub fn collect(inbox: &str, path: &Path) -> Result<usize> {
    let sender = read(path).sender;
    if sender.len() != 32 {
        return Ok(0);
    }
    let url = format!("{}/v1/replies", inbox.trim_end_matches('/'));
    let got: Replies = agent(15)
        .post(&url)
        .send_json(serde_json::json!({ "sender": sender }))
        .map_err(|e| crate::http_error(&url, e))?
        .into_json()
        .map_err(|e| e.to_string())?;
    let _busy = BUSY.lock().unwrap_or_else(|e| e.into_inner());
    let mut file = read(path);
    let mut fresh = 0;
    for r in got.replies {
        let mut answer = r.reply;
        answer.text = answer.text.chars().take(REPLY_MAX).collect();
        // Its own copy; or, for a message sent before copies were kept, one
        // made from what the mailbox has
        let at = file.sent.iter().position(|c| Some(&c.id) == r.local_id.as_ref() || c.inbox_id.as_deref() == Some(r.id.as_str()));
        let copy = match at {
            Some(i) => &mut file.sent[i],
            None => {
                file.sent.push(Sent {
                    id: r.local_id.clone().filter(|l| !l.is_empty()).unwrap_or_else(|| format!("inbox-{}", r.id)),
                    inbox_id: None,
                    kind: r.kind,
                    about: r.about,
                    title: r.title,
                    text: r.text,
                    written: r.written,
                    status: "sent".into(),
                    reply: None,
                    reply_seen: false,
                    reaction: None,
                    read_told: false,
                });
                file.sent.last_mut().unwrap()
            }
        };
        copy.inbox_id = Some(r.id);
        copy.status = "sent".into();
        // A new or changed answer: to be read (again)
        if copy.reply.as_ref() != Some(&answer) {
            copy.reply = Some(answer);
            copy.reply_seen = false;
            copy.reaction = None;
            copy.read_told = false;
            fresh += 1;
        }
    }
    trim_sent(&mut file.sent);
    write(path, &file);
    Ok(fresh)
}

/// The user read this answer and closed it, with an emoji or none. The
/// mailbox is told by `tell_read`.
pub fn mark_seen(path: &Path, id: &str, reaction: Option<String>) {
    let reaction = reaction.filter(|r| REACTIONS.contains(&r.as_str()));
    let _busy = BUSY.lock().unwrap_or_else(|e| e.into_inner());
    let mut file = read(path);
    if let Some(c) = file.sent.iter_mut().find(|c| c.id == id && c.reply.is_some()) {
        if !c.reply_seen || c.reaction != reaction {
            c.reply_seen = true;
            c.reaction = reaction;
            c.read_told = false;
            write(path, &file);
        }
    }
}

/// Tells the mailbox which answers were read (and the emoji), for those it
/// does not know yet. What fails waits for the next time. Returns how many
/// it was told.
pub fn tell_read(inbox: &str, path: &Path) -> usize {
    let file = read(path);
    let todo: Vec<(String, String, Option<String>)> = file
        .sent
        .iter()
        .filter(|c| c.reply_seen && !c.read_told)
        .filter_map(|c| Some((c.id.clone(), c.inbox_id.clone()?, c.reaction.clone())))
        .collect();
    if todo.is_empty() || file.sender.len() != 32 {
        return 0;
    }
    let url = format!("{}/v1/read", inbox.trim_end_matches('/'));
    let a = agent(15);
    let mut told = Vec::new();
    for (id, inbox_id, reaction) in todo {
        match a.post(&url).send_json(serde_json::json!({ "sender": file.sender, "id": inbox_id, "reaction": reaction })) {
            Ok(_) => told.push((id, reaction)),
            // The author deleted it (or the mailbox refuses it): nobody left
            // to tell. A mailbox too old to know of reading answers "not
            // found" instead: that waits until it is updated
            Err(ureq::Error::Status(code @ (400 | 404), r)) => {
                let why = r.into_json::<Status>().ok().and_then(|s| s.error).unwrap_or_default();
                if code == 400 || why == "no such answer" {
                    told.push((id, reaction));
                }
            }
            Err(_) => break,
        }
    }
    if told.is_empty() {
        return 0;
    }
    let _busy = BUSY.lock().unwrap_or_else(|e| e.into_inner());
    let mut file = read(path);
    let mut n = 0;
    for c in file.sent.iter_mut() {
        // Only if nothing changed meanwhile (another emoji chosen)
        if told.iter().any(|(id, r)| *id == c.id && *r == c.reaction) && c.reply_seen {
            c.read_told = true;
            n += 1;
        }
    }
    write(path, &file);
    n
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
            local_id: None,
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

    #[test]
    fn keeps_a_copy_and_collects_the_answers() {
        let path = temp("answers");
        // Nothing sent yet: no id, nothing asked
        assert_eq!(collect("http://127.0.0.1:9", &path).unwrap(), 0);
        let (url, server) = mailbox(vec![(200, CHALLENGE), (200, SENT)]);
        send(&url, &path, message("Camera")).unwrap();
        let seen = server.join().unwrap();
        let body: serde_json::Value = serde_json::from_str(&seen[1][seen[1].find('{').unwrap()..]).unwrap();
        let local = body["local_id"].as_str().unwrap().to_string();
        let copies = sent(&path);
        assert_eq!((copies.len(), copies[0].id.as_str(), copies[0].status.as_str()), (1, local.as_str(), "sent"));
        assert!(copies[0].reply.is_none());
        // One answer to it, and one to a message sent before copies were kept
        let answers: &'static str = Box::leak(format!(
            r#"{{"replies":[{{"id":"100-aa","local_id":"{local}","kind":"idea","title":"Camera","text":"Text","written":5,"reply":{{"text":"Done!","at":50}}}},
               {{"id":"90-bb","local_id":null,"kind":"recomp","about":null,"title":"Daytona","text":"Please","written":4,"reply":{{"text":"Maybe","at":40}}}}]}}"#
        ).into_boxed_str());
        let (url, server) = mailbox(vec![(200, answers)]);
        assert_eq!(collect(&url, &path).unwrap(), 2);
        let seen = server.join().unwrap();
        assert!(seen[0].starts_with("POST /v1/replies"));
        assert!(seen[0].contains(&read(&path).sender));
        let copies = sent(&path);
        assert_eq!(copies.len(), 2);
        let cam = copies.iter().find(|c| c.id == local).unwrap();
        assert_eq!((cam.reply.as_ref().unwrap().text.as_str(), cam.inbox_id.as_deref(), cam.reply_seen), ("Done!", Some("100-aa"), false));
        let old = copies.iter().find(|c| c.title == "Daytona").unwrap();
        assert_eq!((old.id.as_str(), old.kind.as_str()), ("inbox-90-bb", "recomp"));
        // Read: the same answers again are nothing new
        mark_seen(&path, &local, Some("👍".into()));
        mark_seen(&path, &old.id, Some("not an emoji".into()));
        let (url, server) = mailbox(vec![(200, answers)]);
        assert_eq!(collect(&url, &path).unwrap(), 0);
        server.join().unwrap();
        assert!(sent(&path).iter().all(|c| c.reply_seen && !c.read_told));
        assert_eq!(sent(&path).iter().find(|c| c.id == local).unwrap().reaction.as_deref(), Some("👍"));
        assert!(sent(&path).iter().find(|c| c.id == old.id).unwrap().reaction.is_none());
        // The mailbox is told, once: the first time it is away, then it answers
        let away = {
            let l = TcpListener::bind("127.0.0.1:0").unwrap();
            format!("http://{}", l.local_addr().unwrap())
        };
        assert_eq!(tell_read(&away, &path), 0);
        // A mailbox too old for it: still to tell
        let (url, server) = mailbox(vec![(404, r#"{"status":"error","error":"not found"}"#), (404, r#"{"status":"error","error":"not found"}"#)]);
        assert_eq!(tell_read(&url, &path), 0);
        server.join().unwrap();
        let (url, server) = mailbox(vec![(200, r#"{"status":"ok"}"#), (404, r#"{"status":"error","error":"no such answer"}"#)]);
        assert_eq!(tell_read(&url, &path), 2);
        let seen = server.join().unwrap();
        assert!(seen[0].starts_with("POST /v1/read"));
        assert!(seen.iter().any(|s| s.contains("\"100-aa\"") && s.contains("👍")));
        assert!(sent(&path).iter().all(|c| c.read_told));
        assert_eq!(tell_read("http://127.0.0.1:9", &path), 0, "nothing left to tell");
        // Another emoji later is told again
        mark_seen(&path, &local, Some("🎉".into()));
        assert!(!sent(&path).iter().find(|c| c.id == local).unwrap().read_told);
    }

    #[test]
    fn a_kept_message_is_marked_sent_when_it_goes() {
        let path = temp("copyqueued");
        let away = {
            let l = TcpListener::bind("127.0.0.1:0").unwrap();
            format!("http://{}", l.local_addr().unwrap())
        };
        send(&away, &path, message("Later")).unwrap();
        assert_eq!(sent(&path)[0].status, "queued");
        let (url, server) = mailbox(vec![(200, CHALLENGE), (200, SENT)]);
        assert_eq!(flush(&url, &path), 1);
        server.join().unwrap();
        let copies = sent(&path);
        assert_eq!((copies.len(), copies[0].status.as_str()), (1, "sent"));
    }

    #[test]
    fn old_copies_go_but_not_unread_answers() {
        let mk = |n: u64, reply: bool, seen: bool| Sent {
            id: n.to_string(), inbox_id: None, kind: "idea".into(), about: None, title: "t".into(), text: String::new(), written: n,
            status: "sent".into(), reply: reply.then(|| Answer { text: "a".into(), at: n }), reply_seen: seen,
            reaction: None, read_told: false,
        };
        let mut list: Vec<Sent> = (0..SENT_MAX as u64 + 2).map(|n| mk(n, n == 0, false)).collect();
        trim_sent(&mut list);
        assert_eq!(list.len(), SENT_MAX);
        assert_eq!(list[0].id, "0", "the unread answer stays");
        assert_eq!(list[1].id, "3");
    }
}
