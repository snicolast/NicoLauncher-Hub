//! The panel: a page to read the messages, answer them, mark them, archive,
//! block and delete. Served only on the panel's port, which only your own machines
//! reach (Tailscale serve, not funnel). Every change needs the token the page
//! was made with, so another website cannot press its buttons for you.

use crate::notify::{about_name, kind_name};
use crate::store::{Blocked, Stored};

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}

#[derive(Clone, Copy, PartialEq)]
pub enum View {
    New,
    All,
    Replied,
    Flagged,
    Archived,
    Blocked,
}

impl View {
    pub fn parse(q: &str) -> View {
        match q {
            "all" => View::All,
            "replied" => View::Replied,
            "flagged" => View::Flagged,
            "archived" => View::Archived,
            "blocked" => View::Blocked,
            _ => View::New,
        }
    }
    fn key(self) -> &'static str {
        match self {
            View::New => "new",
            View::All => "all",
            View::Replied => "replied",
            View::Flagged => "flagged",
            View::Archived => "archived",
            View::Blocked => "blocked",
        }
    }
    fn shows(self, m: &Stored) -> bool {
        match self {
            View::New => !m.read && !m.archived,
            View::All => !m.archived,
            View::Replied => m.reply.is_some(),
            View::Flagged => !m.flags.is_empty() && !m.archived,
            View::Archived => m.archived,
            View::Blocked => false,
        }
    }
}

fn button(token: &str, view: View, id: &str, act: &str, label: &str, class: &str) -> String {
    let confirm = if act == "delete" { " onsubmit=\"return confirm('Delete this message for good?')\"" } else { "" };
    format!(
        "<form method=post action=/act{confirm}><input type=hidden name=token value={token}><input type=hidden name=view value={v}>\
         <input type=hidden name=id value=\"{id}\"><button name=do value={act} class=\"{class}\">{label}</button></form>",
        v = view.key(),
        id = esc(id)
    )
}

fn hidden(token: &str, view: View, id: &str) -> String {
    format!(
        "<input type=hidden name=token value={token}><input type=hidden name=view value={v}><input type=hidden name=id value=\"{id}\">",
        v = view.key(),
        id = esc(id)
    )
}

/// The answer under a message: a box to write it, or the answer sent, with
/// whether the sender's launcher has collected it.
fn answer(m: &Stored, token: &str, view: View) -> String {
    let form = |text: &str, label: &str| {
        format!(
            "<form method=post action=/act class=reply>{h}<label>Answer (only the launcher that sent this sees it, the next time it starts or refreshes)</label>\
             <textarea name=text maxlength={max} placeholder=\"Write an answer…\">{text}</textarea>\
             <div class=row><span class=dim>Plain text. You can change or remove it later.</span><button name=do value=reply class=primary>{label}</button></div></form>",
            h = hidden(token, view, &m.id),
            max = crate::store::REPLY_MAX,
            text = esc(text)
        )
    };
    match &m.reply {
        None => form("", "Send answer"),
        Some(r) => {
            let got = match r.delivered {
                Some(d) => format!("<span class=ok>✓ Collected by the launcher <time data-t={d}></time></span>"),
                None => "<span class=dim>Waiting for the launcher to collect it</span>".to_string(),
            };
            let remove = format!(
                "<form method=post action=/act onsubmit=\"return confirm('Remove this answer?')\">{h}<button name=do value=unreply class=bad>Remove answer</button></form>",
                h = hidden(token, view, &m.id)
            );
            format!(
                "<div class=answered><div class=who>Your answer <span class=dim>· <time data-t={at}></time> ·</span> {got}</div>\
                 <p>{text}</p><details><summary>Change</summary>{change}</details><div class=row>{remove}</div></div>",
                at = r.at,
                text = esc(&r.text),
                change = form(&r.text, "Save answer"),
            )
        }
    }
}

fn card(m: &Stored, token: &str, view: View, blocked: &Blocked, all: &[Stored]) -> String {
    let msg = &m.message;
    let about = about_name(m).map(|a| format!("<span class=about>{}</span>", esc(&a))).unwrap_or_default();
    let flags: String = m.flags.iter().map(|f| format!("<span class=flag>{}</span>", esc(f))).collect();
    let title = match msg.system.as_deref().filter(|s| !s.is_empty()) {
        Some(s) => format!("{} <span class=dim>({})</span>", esc(&msg.title), esc(s)),
        None => esc(&msg.title),
    };
    let contact = msg.contact.as_deref().filter(|c| !c.is_empty())
        .map(|c| format!("<div class=contact><b>Contact</b> {}</div>", esc(c))).unwrap_or_default();
    let details: String = msg.details.iter().map(|(k, v)| format!("<dt>{}</dt><dd>{}</dd>", esc(k), esc(v))).collect();
    let late = msg.written.filter(|w| m.received.saturating_sub(*w) > 600)
        .map(|w| format!(" · written <time data-t={w}></time>")).unwrap_or_default();
    let is_blocked = blocked.keys.iter().any(|k| *k == msg.sender || *k == m.address);
    let mut actions = String::new();
    actions += &if m.read { button(token, view, &m.id, "unread", "Mark unread", "") } else { button(token, view, &m.id, "read", "Mark read", "primary") };
    actions += &if m.archived { button(token, view, &m.id, "unarchive", "Back to inbox", "") } else { button(token, view, &m.id, "archive", "Archive", "") };
    if !is_blocked {
        actions += &button(token, view, &m.id, "block", "Block sender", "warn");
    }
    actions += &button(token, view, &m.id, "delete", "Delete", "bad");
    format!(
        "<article id=\"{id}\" class=\"msg {kind}{unread}\">\
           <header><span class=kind>{kname}</span>{about}{flags}<time data-t={received}></time></header>\
           <h2>{title}</h2>\
           <p class=text>{text}</p>{contact}\
           <dl>{details}</dl>{answer}\
           <footer><span class=dim>Sender {sender} · {address}{lang}{late}{blocked_note}</span><span class=actions>{actions}</span></footer>\
         </article>",
        id = esc(&m.id),
        kind = esc(&msg.kind),
        unread = if m.read { "" } else { " unread" },
        kname = kind_name(&msg.kind),
        received = m.received,
        text = esc(&msg.text),
        sender = esc(&msg.sender.chars().take(8).collect::<String>()),
        address = esc(&m.address),
        lang = msg.lang.as_deref().map(|l| format!(" · {}", esc(l))).unwrap_or_default(),
        blocked_note = if is_blocked { " · <b class=bad>blocked</b>" } else { "" },
        answer = if is_blocked { String::new() } else { answer(m, token, view) },
    )
}

pub fn page(all: &[Stored], blocked: &Blocked, view: View, token: &str) -> String {
    let count = |v: View| all.iter().filter(|m| v.shows(m)).count();
    let tab = |v: View, label: &str, n: Option<usize>| {
        let n = n.map(|n| format!(" <span class=n>{n}</span>")).unwrap_or_default();
        format!("<a href=\"/?view={}\" class=\"{}\">{label}{n}</a>", v.key(), if v == view { "on" } else { "" })
    };
    let tabs = [
        tab(View::New, "New", Some(count(View::New))),
        tab(View::All, "Inbox", Some(count(View::All))),
        tab(View::Replied, "Answered", Some(count(View::Replied))),
        tab(View::Flagged, "Flagged", Some(count(View::Flagged))),
        tab(View::Archived, "Archived", Some(count(View::Archived))),
        tab(View::Blocked, "Blocked", Some(blocked.keys.len())),
    ]
    .concat();
    let body = if view == View::Blocked {
        if blocked.keys.is_empty() {
            "<p class=empty>Nobody is blocked.</p>".to_string()
        } else {
            blocked.keys.iter().map(|k| format!(
                "<div class=blockrow><code>{}</code>{}</div>", esc(k), button(token, view, k, "unblock", "Unblock", ""))).collect()
        }
    } else {
        let list: Vec<String> = all.iter().filter(|m| view.shows(m)).map(|m| card(m, token, view, blocked, all)).collect();
        if list.is_empty() {
            "<p class=empty>Nothing here.</p>".to_string()
        } else {
            let all_read = if view == View::New { button(token, view, "", "allread", "Mark all read", "") } else { String::new() };
            format!("<div class=bar>{all_read}</div>{}", list.concat())
        }
    };
    format!(
        "<!doctype html><html lang=en><head><meta charset=utf-8><meta name=viewport content=\"width=device-width,initial-scale=1\">\
         <title>Nico inbox</title><style>{STYLE}</style></head><body>\
         <main><h1>Nico inbox</h1><nav>{tabs}</nav>{body}</main>\
         <script>for(const t of document.querySelectorAll('time[data-t]'))t.textContent=new Date(t.dataset.t*1000).toLocaleString([], {{dateStyle:'medium',timeStyle:'short'}});</script>\
         </body></html>"
    )
}

const STYLE: &str = r#"
:root{--bg:#0d111c;--surface:#151b2b;--line:#252e45;--text:#e8ecf5;--dim:#8a94ab;--accent:#4f8cff;--bad:#ff6b6b;--warn:#ffb84d}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--text);font:15px/1.5 "Segoe UI",system-ui,sans-serif}
main{max-width:860px;margin:0 auto;padding:24px 16px 60px}h1{font-size:24px;margin:0 0 14px}
nav{display:flex;gap:6px;flex-wrap:wrap;margin-bottom:18px}nav a{color:var(--dim);text-decoration:none;padding:6px 12px;border-radius:9px;border:1px solid var(--line)}
nav a.on{color:#fff;background:var(--accent);border-color:var(--accent)}.n{opacity:.75;font-size:12px}
.bar{display:flex;justify-content:flex-end;margin-bottom:10px}
.msg{background:var(--surface);border:1px solid var(--line);border-radius:14px;padding:14px 16px;margin-bottom:12px;--tone:79,140,255}
.msg.idea{--tone:255,184,77}.msg.recomp{--tone:167,128,255}.msg.talk{--tone:61,220,151}
.msg.unread{border-left:4px solid rgb(var(--tone))}
.msg header{display:flex;align-items:center;gap:8px;flex-wrap:wrap;font-size:13px;color:var(--dim)}
.msg header time{margin-left:auto}.kind{color:rgb(var(--tone));background:rgba(var(--tone),.14);border:1px solid rgba(var(--tone),.35);padding:1px 8px;border-radius:7px;font-weight:600}
.about{color:var(--text)}.flag{color:var(--warn);border:1px solid rgba(255,184,77,.45);padding:0 7px;border-radius:7px}
.msg h2{font-size:17px;margin:8px 0 4px}.text{white-space:pre-wrap;margin:0 0 8px;overflow-wrap:anywhere}
.contact{margin:6px 0;font-size:14px}.contact b{color:var(--dim);font-weight:600;margin-right:6px}
dl{display:grid;grid-template-columns:max-content 1fr;gap:2px 14px;font-size:13px;color:var(--dim);margin:8px 0}dt{color:var(--text)}dd{margin:0}
footer{display:flex;align-items:center;gap:10px;flex-wrap:wrap;border-top:1px solid var(--line);padding-top:10px;margin-top:8px;font-size:12.5px}
.actions{margin-left:auto;display:flex;gap:6px;flex-wrap:wrap}form{margin:0}
button{font:inherit;font-size:13px;color:var(--text);background:rgba(255,255,255,.07);border:1px solid var(--line);border-radius:8px;padding:5px 10px;cursor:pointer}
button:hover{border-color:var(--dim)}button.primary{background:var(--accent);border-color:var(--accent);color:#fff}
button.warn{color:var(--warn)}button.bad{color:var(--bad)}.bad{color:var(--bad)}.dim{color:var(--dim)}
.reply{margin-top:10px;border-top:1px dashed var(--line);padding-top:10px}
.reply label{display:block;font-size:13px;color:var(--dim);margin-bottom:6px}
.reply textarea{width:100%;min-height:84px;resize:vertical;font:inherit;font-size:14px;color:var(--text);background:#0f1422;border:1px solid var(--line);border-radius:10px;padding:9px 11px}
.reply textarea:focus{outline:none;border-color:var(--accent)}
.reply .row{display:flex;align-items:center;gap:10px;margin-top:8px}.reply .row .dim{font-size:12.5px}.reply .row button{margin-left:auto}
.answered{background:rgba(61,220,151,.08);border:1px solid rgba(61,220,151,.35);border-radius:10px;padding:10px 12px;margin-top:10px}
.answered .who{font-size:12.5px;color:#3ddc97;font-weight:600}.answered .who .dim{font-weight:400}
.answered p{margin:6px 0 4px;white-space:pre-wrap;overflow-wrap:anywhere}.answered .reply{border:0;padding-top:4px}
.answered details summary{cursor:pointer;font-size:13px;color:var(--accent);margin-top:4px}
.answered>.row{display:flex;justify-content:flex-end;margin-top:6px}
.ok{color:#3ddc97}
.empty{color:var(--dim);text-align:center;padding:40px 0}.blockrow{display:flex;align-items:center;justify-content:space-between;background:var(--surface);border:1px solid var(--line);border-radius:10px;padding:8px 12px;margin-bottom:8px}
"#;
