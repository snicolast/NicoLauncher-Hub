//! The notice on the phone, through ntfy. Sent on its own thread: a slow or
//! failing ntfy never holds up the launcher that sent the message.

use crate::config::Ntfy;
use crate::store::Stored;

pub fn kind_name(kind: &str) -> &'static str {
    match kind {
        "idea" => "Idea",
        "recomp" => "Recomp proposal",
        _ => "Message",
    }
}

/// What the message is about, in words: the game's name as the launcher
/// wrote it in the details, or "Launcher".
pub fn about_name(m: &Stored) -> Option<String> {
    match m.message.about.as_deref() {
        None => None,
        Some("launcher") => Some("Launcher".into()),
        Some(id) => Some(m.message.details.get(1).map(|(k, _)| k.clone()).unwrap_or_else(|| id.to_string())),
    }
}

pub fn send(ntfy: Ntfy, m: Stored, log: impl Fn(String) + Send + 'static) {
    std::thread::spawn(move || {
        let title = match about_name(&m) {
            Some(a) => format!("{} · {a}", kind_name(&m.message.kind)),
            None => kind_name(&m.message.kind).to_string(),
        };
        let mut body = m.message.title.clone();
        if let Some(s) = m.message.system.as_deref().filter(|s| !s.is_empty()) {
            body.push_str(&format!(" ({s})"));
        }
        if ntfy.full_text && !m.message.text.is_empty() {
            body.push_str("\n\n");
            body.push_str(&m.message.text);
        }
        let tag = match m.message.kind.as_str() {
            "idea" => "bulb",
            "recomp" => "joystick",
            _ => "speech_balloon",
        };
        let mut tags = vec![tag];
        if !m.flags.is_empty() {
            tags.insert(0, "warning");
        }
        // Published as JSON: titles with accents or "·" do not fit in HTTP headers
        let (server, topic) = ntfy.url.trim_end_matches('/').rsplit_once('/').unwrap_or(("https://ntfy.sh", &ntfy.url));
        let mut json = serde_json::json!({
            "topic": topic,
            "title": title,
            "message": body,
            "tags": tags,
            "priority": if m.flags.is_empty() { 3 } else { 2 },
        });
        if let Some(click) = &ntfy.click {
            json["click"] = format!("{}#{}", click.trim_end_matches('#'), m.id).into();
        }
        let mut req = ureq::post(server).timeout(std::time::Duration::from_secs(20));
        if let Some(token) = ntfy.token.as_deref().filter(|t| !t.is_empty()) {
            req = req.set("Authorization", &format!("Bearer {token}"));
        }
        if let Err(e) = req.send_json(json) {
            log(format!("ntfy failed for {}: {e}", m.id));
        }
    });
}
