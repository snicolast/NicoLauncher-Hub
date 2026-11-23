//! nico-inbox: snicolast's private mailbox for the launcher's ideas, recomp
//! proposals and messages. It runs on one machine of his, behind Tailscale:
//!
//! - the public port (Tailscale Funnel) only takes messages: /v1/challenge,
//!   /v1/message and /v1/ping;
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
                let view = form(query).into_iter().find(|(k, _)| k == "view").map(|(_, v)| v).unwrap_or_default();
                html(panel::page(&app.store.all(), &app.store.blocked(), panel::View::parse(&view), &app.token))
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
                    act(&app, get("do"), get("id"));
                    let back = format!("/?view={}", get("view").chars().filter(char::is_ascii_alphabetic).collect::<String>());
                    Response::from_data(Vec::new()).with_status_code(303).with_header(header("Location", &back))
                }
            }
            _ => html("<p>Not found</p>".into()).with_status_code(404),
        };
        let _ = req.respond(reply);
    }
}

fn act(app: &App, what: &str, id: &str) {
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
        "delete" => store.delete(id),
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
}
