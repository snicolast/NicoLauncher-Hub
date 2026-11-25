//! nico-inbox: snicolast's private mailbox for the launcher's ideas, recomp
//! proposals and messages. It runs on one machine of his, behind Tailscale:
//!
//! - the public port (Tailscale Funnel) only takes messages and hands each
//!   launcher the answers to its own: /v1/challenge, /v1/message,
//!   /v1/replies, /v1/read and /v1/ping;
//! - the panel port (Tailscale serve, his machines only) shows them.
//!
//! The launcher is open source, so nothing in it is secret: a small sum
//! before each message (pow.rs) and limits per sender (limits.rs) keep spam
//! down. See docs/feedback.md.

mod config;
mod limits;
mod notify;
mod panel;
mod pow;
mod store;

use config::Config;
use limits::Limiter;
use std::io::Read;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use store::{Incoming, Store, Stored};
use tiny_http::{Header, Method, Request, Response, Server};

/// A message bigger than this is not read.
const BODY_MAX: u64 = 64 * 1024;

pub fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

fn random_hex(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    getrandom::getrandom(&mut buf).expect("the system has no random numbers");
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

struct App {
    config: Config,
    store: Store,
    limiter: Mutex<Limiter>,
    /// Put in the panel's forms; changes without it are refused.
    token: String,
}

fn header(name: &str, value: &str) -> Header {
    Header::from_bytes(name.as_bytes(), value.as_bytes()).unwrap()
}

fn json(code: u16, value: serde_json::Value) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_data(value.to_string().into_bytes())
        .with_status_code(code)
        .with_header(header("Content-Type", "application/json"))
        .with_header(header("Cache-Control", "no-store"))
}

fn status(code: u16, status: &str, error: Option<&str>) -> Response<std::io::Cursor<Vec<u8>>> {
    json(code, serde_json::json!({ "status": status, "error": error }))
}

/// The sender's address: the first one in X-Forwarded-For when a proxy on
/// this machine (Tailscale) passed the request on, else the connection's.
fn address(app: &App, req: &Request) -> String {
    let direct = req.remote_addr().map(|a| a.ip().to_string()).unwrap_or_default();
    if app.config.trust_forwarded {
        if let Some(h) = req.headers().iter().find(|h| h.field.equiv("X-Forwarded-For")) {
            if let Some(first) = h.value.as_str().split(',').next().map(str::trim).filter(|s| !s.is_empty()) {
                return first.to_string();
            }
        }
    }
    direct
}

fn public(app: Arc<App>, server: Server) {
    for mut req in server.incoming_requests() {
        let addr = address(&app, &req);
        // With or without /v1: Tailscale may pass the path on whole or
        // without the part it was published under
        let path = req.url().split('?').next().unwrap_or("").to_string();
        let path = path.strip_prefix("/v1").filter(|p| p.starts_with('/')).unwrap_or(&path).to_string();
        let reply = match (req.method(), path.as_str()) {
            (Method::Get, "/ping") => json(200, serde_json::json!({ "ok": true })),
            (Method::Get, "/challenge") => {
                let given = app.limiter.lock().unwrap().challenge(&addr, now(), random_hex(16));
                match given {
                    Some(c) => json(200, serde_json::json!({ "challenge": c, "bits": app.config.pow_bits })),
                    None => status(429, "limited", None),
                }
            }
            (Method::Post, "/message") => receive(&app, &mut req, &addr),
            (Method::Post, "/replies") => replies(&app, &mut req, &addr),
            (Method::Post, "/read") => read(&app, &mut req, &addr),
            _ => status(404, "error", Some("not found")),
        };
        let _ = req.respond(reply);
    }
}

fn receive(app: &App, req: &mut Request, addr: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    let mut body = Vec::new();
    if req.as_reader().take(BODY_MAX + 1).read_to_end(&mut body).is_err() || body.len() as u64 > BODY_MAX {
        return status(413, "error", Some("too big"));
    }
    let msg: Incoming = match serde_json::from_slice(&body) {
        Ok(m) => m,
        Err(_) => return status(400, "error", Some("not a message")),
    };
    if let Err(e) = msg.check() {
        return status(400, "error", Some(e));
    }
    let t = now();
    // An unknown or used challenge: the launcher asks for a new one and sends again
    if !app.limiter.lock().unwrap().redeem(&msg.challenge, t) {
        return status(403, "retry", Some("challenge"));
    }
    if !pow::valid(&msg.challenge, msg.nonce, app.config.pow_bits) {
        return status(400, "error", Some("wrong sum"));
    }
    let blocked = app.store.blocked();
    if blocked.keys.iter().any(|k| *k == msg.sender || k == addr) {
        app.store.log(&format!("dropped a message from blocked {} {addr}", msg.sender));
        return status(200, "sent", None);
    }
    {
        let mut lim = app.limiter.lock().unwrap();
        let keys = [msg.sender.as_str(), addr];
        if !lim.allows(&keys, t, &app.config.limits) {
            app.store.log(&format!("limited {} {addr}", msg.sender));
            return status(429, "limited", None);
        }
        lim.count(&keys, t);
    }
    let stored = Stored {
        id: format!("{t}-{}", random_hex(3)),
        received: t,
        address: addr.to_string(),
        flags: store::flags(&msg),
        message: msg,
        read: false,
        archived: false,
        important: false,
        reply: None,
    };
    if let Err(e) = app.store.save(&stored) {
        app.store.log(&format!("could not save a message: {e}"));
        return status(503, "error", Some("could not save"));
    }
    app.store.log(&format!("kept {} ({}) from {}", stored.id, stored.message.kind, addr));
    if let Some(ntfy) = app.config.ntfy.clone() {
        let log_dir = app.config.data.clone();
        notify::send(ntfy, stored, move |line| {
            if let Ok(s) = Store::open(&log_dir) {
                s.log(&line);
            }
        });
    }
    status(200, "sent", None)
}

/// The answers to one launcher's messages. It asks with its sender id, which
/// only it and this mailbox know (random, never shown anywhere public), so
/// nobody else can read them. Only messages that have an answer are listed,
/// with what the user wrote, so a launcher that kept no copy (older ones) can
/// still show what the answer is about.
fn replies(app: &App, req: &mut Request, addr: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    #[derive(serde::Deserialize)]
    struct Ask {
        sender: String,
    }
    let mut body = Vec::new();
    if req.as_reader().take(4096).read_to_end(&mut body).is_err() {
        return status(400, "error", Some("not a request"));
    }
    let Ok(ask) = serde_json::from_slice::<Ask>(&body) else {
        return status(400, "error", Some("not a request"));
    };
    // The launcher's ids are 32 hex characters: anything else has no answers
    if ask.sender.len() != 32 || !ask.sender.chars().all(|c| c.is_ascii_hexdigit()) {
        return status(400, "error", Some("not a sender"));
    }
    let t = now();
    if !app.limiter.lock().unwrap().lookup(addr, t) {
        return status(429, "limited", None);
    }
    let blocked = app.store.blocked();
    if blocked.keys.iter().any(|k| *k == ask.sender || k == addr) {
        return json(200, serde_json::json!({ "replies": [] }));
    }
    let mut out = Vec::new();
    for mut m in app.store.all().into_iter().filter(|m| m.message.sender == ask.sender && m.reply.is_some()) {
        let r = m.reply.clone().unwrap();
        out.push(serde_json::json!({
            "id": m.id, "local_id": m.message.local_id, "kind": m.message.kind, "about": m.message.about,
            "title": m.message.title, "text": m.message.text, "written": m.message.written.unwrap_or(m.received),
            "reply": { "text": r.text, "at": r.at },
        }));
        if r.delivered.is_none() {
            m.reply.as_mut().unwrap().delivered = Some(t);
            let _ = app.store.save(&m);
            app.store.log(&format!("answer to {} collected by its launcher", m.id));
        }
    }
    json(200, serde_json::json!({ "replies": out }))
}

/// The user opened an answer and closed it again, maybe with an emoji: the
/// panel shows it, and a quiet notice goes to the phone. Asked with the
/// sender id, like the answers, so only the launcher that got it can say so.
fn read(app: &App, req: &mut Request, addr: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    #[derive(serde::Deserialize)]
    struct Told {
        sender: String,
        id: String,
        #[serde(default)]
        reaction: Option<String>,
    }
    let mut body = Vec::new();
    if req.as_reader().take(4096).read_to_end(&mut body).is_err() {
        return status(400, "error", Some("not a request"));
    }
    let Ok(told) = serde_json::from_slice::<Told>(&body) else {
        return status(400, "error", Some("not a request"));
    };
    if told.reaction.as_deref().is_some_and(|r| !store::REACTIONS.contains(&r)) {
        return status(400, "error", Some("unknown reaction"));
    }
    let t = now();
    if !app.limiter.lock().unwrap().lookup(addr, t) {
        return status(429, "limited", None);
    }
    let blocked = app.store.blocked();
    if blocked.keys.iter().any(|k| *k == told.sender || k == addr) {
        return status(200, "ok", None);
    }
    // Another launcher's message, or one deleted since: nothing to tell
    let Some(mut m) = app.store.get(&told.id).filter(|m| m.message.sender == told.sender && m.reply.is_some()) else {
        return status(404, "error", Some("no such answer"));
    };
    let r = m.reply.as_mut().unwrap();
    let news = r.read.is_none() || r.reaction != told.reaction;
    if r.read.is_none() {
        r.read = Some(t);
    }
    r.reaction = told.reaction;
    if news {
        let _ = app.store.save(&m);
        app.store.log(&format!("answer to {} read{}", m.id, m.reply.as_ref().unwrap().reaction.as_deref().map(|e| format!(" {e}")).unwrap_or_default()));
        if let Some(ntfy) = app.config.ntfy.clone() {
            let log_dir = app.config.data.clone();
            notify::read(ntfy, m, move |line| {
                if let Ok(s) = Store::open(&log_dir) {
                    s.log(&line);
                }
            });
        }
    }
    status(200, "ok", None)
}

/// application/x-www-form-urlencoded, as the panel's forms send it.
fn form(body: &str) -> Vec<(String, String)> {
    fn decode(s: &str) -> String {
        let s = s.replace('+', " ");
        let bytes = s.as_bytes();
        let mut out = Vec::with_capacity(bytes.len());
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                if let Ok(b) = u8::from_str_radix(&s[i + 1..i + 3], 16) {
                    out.push(b);
                    i += 3;
                    continue;
                }
            }
            out.push(bytes[i]);
            i += 1;
        }
        String::from_utf8_lossy(&out).into_owned()
    }
    body.split('&').filter_map(|kv| kv.split_once('=')).map(|(k, v)| (decode(k), decode(v))).collect()
}

fn html(text: String) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_data(text.into_bytes())
        .with_header(header("Content-Type", "text/html; charset=utf-8"))
        .with_header(header("Cache-Control", "no-store"))
        .with_header(header("X-Frame-Options", "DENY"))
        .with_header(header("Content-Security-Policy", "default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline'; form-action 'self'"))
}

fn panel(app: Arc<App>, server: Server) {
    for mut req in server.incoming_requests() {
        let url = req.url().to_string();
        let (path, query) = url.split_once('?').unwrap_or((&url, ""));
        let reply = match (req.method(), path) {
            (Method::Get, "/") => {
                let q = panel::Query::parse(&form(query));
                // Opening a message marks it read, like a mail client
                if let Some(mut m) = app.store.get(&q.id).filter(|m| !m.read) {
                    m.read = true;
                    let _ = app.store.save(&m);
                }
                html(panel::page(&app.store.all(), &app.store.blocked(), &q, &app.token))
            }
            (Method::Post, "/act") => {
                let mut body = String::new();
                let _ = req.as_reader().take(BODY_MAX).read_to_string(&mut body);
                let f = form(&body);
                let get = |k: &str| f.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str()).unwrap_or("");
                if get("token") != app.token {
                    // The panel was restarted (new token), or another page tried: back to the list
                    Response::from_data(Vec::new()).with_status_code(303).with_header(header("Location", "/"))
                } else {
                    act(&app, get("do"), get("id"), get("text"));
                    let back = back_link(get("back"), get("do") == "delete");
                    Response::from_data(Vec::new()).with_status_code(303).with_header(header("Location", &back))
                }
            }
            _ => html("<p>Not found</p>".into()).with_status_code(404),
        };
        let _ = req.respond(reply);
    }
}

/// Where a button sends the panel back to: the page it was pressed on, as long
/// as that is a plain link of this panel. After a delete, without the message.
fn back_link(back: &str, deleted: bool) -> String {
    let safe = back.starts_with("/?") && back.chars().all(|c| c.is_ascii_alphanumeric() || "/?=&%-_.~".contains(c));
    if !safe {
        return "/".into();
    }
    if !deleted {
        return back.to_string();
    }
    let (_, qs) = back.split_at(2);
    let rest: Vec<&str> = qs.split('&').filter(|p| !p.starts_with("id=") && !p.is_empty()).collect();
    format!("/?{}", rest.join("&"))
}

fn act(app: &App, what: &str, id: &str, text: &str) {
    let store = &app.store;
    let change = |f: &dyn Fn(&mut Stored)| {
        if let Some(mut m) = store.get(id) {
            f(&mut m);
            let _ = store.save(&m);
        }
    };
    match what {
        "read" => change(&|m| m.read = true),
        "unread" => change(&|m| m.read = false),
        "archive" => change(&|m| { m.archived = true; m.read = true; }),
        "unarchive" => change(&|m| m.archived = false),
        "important" => change(&|m| m.important = true),
        "unimportant" => change(&|m| m.important = false),
        "delete" => store.delete(id),
        // Writing an answer also marks the message read. An empty one removes it.
        "reply" => {
            let text = text.replace("\r\n", "\n");
            let text = text.trim();
            if text.is_empty() {
                change(&|m| m.reply = None);
            } else {
                let text: String = text.chars().take(store::REPLY_MAX).collect();
                change(&|m| {
                    m.reply = Some(store::Reply { text: text.clone(), at: now(), delivered: None, read: None, reaction: None });
                    m.read = true;
                });
                store.log(&format!("answered {id}"));
            }
        }
        "unreply" => change(&|m| m.reply = None),
        "allread" => {
            for mut m in store.all().into_iter().filter(|m| !m.read) {
                m.read = true;
                let _ = store.save(&m);
            }
        }
        "block" => {
            if let Some(m) = store.get(id) {
                let mut b = store.blocked();
                for key in [m.message.sender.clone(), m.address.clone()] {
                    if !key.is_empty() && !b.keys.contains(&key) {
                        b.keys.push(key);
                    }
                }
                let _ = store.set_blocked(&b);
                change(&|m| { m.archived = true; m.read = true; });
                store.log(&format!("blocked {} {}", m.message.sender, m.address));
            }
        }
        "unblock" => {
            let mut b = store.blocked();
            b.keys.retain(|k| k != id);
            let _ = store.set_blocked(&b);
        }
        _ => {}
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut path: Option<PathBuf> = None;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--config" => path = args.next().map(PathBuf::from),
            "--version" => {
                println!("nico-inbox {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => {
                eprintln!("usage: nico-inbox [--config <config.toml>]");
                std::process::exit(2);
            }
        }
    }
    let path = path.unwrap_or_else(|| {
        std::env::current_exe().ok().and_then(|e| e.parent().map(|d| d.join("config.toml"))).unwrap_or_else(|| "config.toml".into())
    });
    let config = Config::load(&path).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    let store = Store::open(&config.data).unwrap_or_else(|e| {
        eprintln!("{}: {e}", config.data.display());
        std::process::exit(1);
    });
    let public_server = Server::http(&config.public).unwrap_or_else(|e| {
        eprintln!("{}: {e}", config.public);
        std::process::exit(1);
    });
    let panel_server = Server::http(&config.panel).unwrap_or_else(|e| {
        eprintln!("{}: {e}", config.panel);
        std::process::exit(1);
    });
    store.log(&format!("nico-inbox {} listening: messages on {}, panel on {}", env!("CARGO_PKG_VERSION"), config.public, config.panel));
    let app = Arc::new(App { config, store, limiter: Mutex::new(Limiter::default()), token: random_hex(16) });
    let a = app.clone();
    let messages = std::thread::spawn(move || public(a, public_server));
    panel(app, panel_server);
    let _ = messages.join();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_forms() {
        let f = form("token=ab12&id=1700-x%2Fy&do=read+all");
        assert_eq!(f[1], ("id".to_string(), "1700-x/y".to_string()));
        assert_eq!(f[2].1, "read all");
        assert_eq!(form("a=%E2%9C%93")[0].1, "✓");
        assert_eq!(form("a=%2")[0].1, "%2");
    }

    fn app(name: &str) -> Arc<App> {
        let dir = std::env::temp_dir().join(format!("nico-inbox-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let mut config: Config = toml::from_str("").unwrap();
        config.data = dir;
        let store = Store::open(&config.data).unwrap();
        Arc::new(App { config, store, limiter: Mutex::new(Limiter::default()), token: "tok".into() })
    }

    fn stored(id: &str, sender: &str, title: &str) -> Stored {
        let message: Incoming = serde_json::from_value(serde_json::json!({
            "sender": sender, "challenge": "c", "nonce": 0, "kind": "idea", "title": title, "text": "Please", "local_id": format!("l{id}"),
        })).unwrap();
        Stored { id: id.into(), received: 10, address: "1.2.3.4".into(), message, flags: vec![], read: false, archived: false, important: false, reply: None }
    }

    #[test]
    fn hands_each_launcher_only_its_own_answers() {
        let app = app("replies");
        let me = "a".repeat(32);
        let other = "b".repeat(32);
        app.store.save(&stored("1-a", &me, "Camera")).unwrap();
        app.store.save(&stored("2-b", &me, "No answer yet")).unwrap();
        app.store.save(&stored("3-c", &other, "Someone else")).unwrap();
        act(&app, "reply", "1-a", "  Done!\r\nThanks  ");
        act(&app, "reply", "3-c", "Not for you");
        let m = app.store.get("1-a").unwrap();
        assert_eq!(m.reply.as_ref().unwrap().text, "Done!\nThanks");
        assert!(m.read, "answering marks it read");
        assert!(m.reply.as_ref().unwrap().delivered.is_none());

        let server = Server::http("127.0.0.1:0").unwrap();
        let url = format!("http://{}/v1/replies", server.server_addr().to_ip().unwrap());
        let a = app.clone();
        std::thread::spawn(move || public(a, server));
        let ask = |sender: &str| -> Result<serde_json::Value, u16> {
            match ureq::post(&url).send_json(serde_json::json!({ "sender": sender })) {
                Ok(r) => Ok(r.into_json().unwrap()),
                Err(ureq::Error::Status(c, _)) => Err(c),
                Err(e) => panic!("{e}"),
            }
        };
        let got = ask(&me).unwrap();
        let list = got["replies"].as_array().unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0]["id"], "1-a");
        assert_eq!(list[0]["local_id"], "l1-a");
        assert_eq!(list[0]["title"], "Camera");
        assert_eq!(list[0]["reply"]["text"], "Done!\nThanks");
        assert!(list[0].get("address").is_none() && list[0].get("sender").is_none());
        assert!(app.store.get("1-a").unwrap().reply.unwrap().delivered.is_some());
        assert_eq!(ask(&"c".repeat(32)).unwrap()["replies"].as_array().unwrap().len(), 0);
        assert_eq!(ask("short").unwrap_err(), 400);
        // Read, with an emoji: only by the launcher that sent it, and only a known emoji
        let read_url = url.replace("/v1/replies", "/v1/read");
        let tell = |sender: &str, id: &str, reaction: Option<&str>| -> u16 {
            match ureq::post(&read_url).send_json(serde_json::json!({ "sender": sender, "id": id, "reaction": reaction })) {
                Ok(r) => r.status(),
                Err(ureq::Error::Status(c, _)) => c,
                Err(e) => panic!("{e}"),
            }
        };
        assert_eq!(tell(&other, "1-a", None), 404, "not its message");
        assert_eq!(tell(&me, "2-b", None), 404, "no answer yet");
        assert_eq!(tell(&me, "1-a", Some("💩")), 400);
        assert_eq!(tell(&me, "1-a", Some("👍")), 200);
        let r = app.store.get("1-a").unwrap().reply.unwrap();
        let first = r.read.unwrap();
        assert_eq!(r.reaction.as_deref(), Some("👍"));
        assert_eq!(tell(&me, "1-a", Some("❤️")), 200);
        let r = app.store.get("1-a").unwrap().reply.unwrap();
        assert_eq!((r.read, r.reaction.as_deref()), (Some(first), Some("❤️")), "a new emoji keeps the first time it was read");
        let q = panel::Query::parse(&[("view".into(), "replied".into()), ("id".into(), "1-a".into())]);
        let page = panel::page(&app.store.all(), &app.store.blocked(), &q, "tok");
        assert!(page.contains("you can archive or delete") && page.contains("class=reaction") && page.contains("❤️"));
        // A changed answer is new: not read yet
        act(&app, "reply", "1-a", "Done, and more");
        let r = app.store.get("1-a").unwrap().reply.unwrap();
        assert!(r.read.is_none() && r.reaction.is_none());
        // Blocked: nothing
        app.store.set_blocked(&store::Blocked { keys: vec![me.clone()] }).unwrap();
        assert_eq!(ask(&me).unwrap()["replies"].as_array().unwrap().len(), 0);
        // An empty answer removes it
        act(&app, "reply", "1-a", "   ");
        assert!(app.store.get("1-a").unwrap().reply.is_none());
        let _ = std::fs::remove_dir_all(&app.config.data);
    }

    #[test]
    fn buttons_only_go_back_to_the_panel() {
        assert_eq!(back_link("/?view=important&id=1-a", false), "/?view=important&id=1-a");
        assert_eq!(back_link("/?view=important&id=1-a&q=cam", true), "/?view=important&q=cam");
        assert_eq!(back_link("https://evil.example/", false), "/");
        assert_eq!(back_link("//evil.example/?x", false), "/");
        assert_eq!(back_link("/?q=<script>", false), "/");
        assert_eq!(back_link("", false), "/");
    }

    #[test]
    fn asking_for_answers_is_limited() {
        let mut l = Limiter::default();
        assert!((0..60).all(|_| l.lookup("x", 1000)));
        assert!(!l.lookup("x", 1000));
        assert!(l.lookup("y", 1000));
        assert!(l.lookup("x", 1000 + 3600));
    }
}
