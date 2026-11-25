//! The panel: the mailbox as a mail client. Folders on the left, the list in
//! the middle, the open message on the right (one column on a phone). From
//! there the author answers, marks important, archives, blocks and deletes.
//! Served only on the panel's port, which only your own machines reach
//! (Tailscale serve, not funnel). Every change needs the token the page was
//! made with, so another website cannot press its buttons for you.

use crate::notify::{about_name, kind_name};
use crate::store::{Blocked, Stored};

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;").replace('\'', "&#39;")
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum View {
    Inbox,
    Unread,
    Important,
    ToAnswer,
    Replied,
    Flagged,
    Archived,
    Blocked,
}

const VIEWS: [View; 8] = [View::Inbox, View::Unread, View::Important, View::ToAnswer, View::Replied, View::Flagged, View::Archived, View::Blocked];

impl View {
    pub fn parse(q: &str) -> View {
        VIEWS.into_iter().find(|v| v.key() == q).unwrap_or(View::Inbox)
    }
    fn key(self) -> &'static str {
        match self {
            View::Inbox => "inbox",
            View::Unread => "unread",
            View::Important => "important",
            View::ToAnswer => "toanswer",
            View::Replied => "replied",
            View::Flagged => "flagged",
            View::Archived => "archived",
            View::Blocked => "blocked",
        }
    }
    fn label(self) -> &'static str {
        match self {
            View::Inbox => "Inbox",
            View::Unread => "Unread",
            View::Important => "Important",
            View::ToAnswer => "To answer",
            View::Replied => "Answered",
            View::Flagged => "Flagged",
            View::Archived => "Archived",
            View::Blocked => "Blocked",
        }
    }
    fn icon(self) -> &'static str {
        match self {
            View::Inbox => ICON_INBOX,
            View::Unread => ICON_DOT,
            View::Important => ICON_STAR,
            View::ToAnswer => ICON_REPLY,
            View::Replied => ICON_CHECK,
            View::Flagged => ICON_FLAG,
            View::Archived => ICON_ARCHIVE,
            View::Blocked => ICON_BAN,
        }
    }
    pub fn shows(self, m: &Stored) -> bool {
        match self {
            View::Inbox => !m.archived,
            View::Unread => !m.read && !m.archived,
            View::Important => m.important,
            View::ToAnswer => m.reply.is_none() && !m.archived,
            View::Replied => m.reply.is_some(),
            View::Flagged => !m.flags.is_empty() && !m.archived,
            View::Archived => m.archived,
            View::Blocked => false,
        }
    }
}

/// What the page shows: a folder, maybe narrowed by kind, game and words,
/// and maybe one message open.
#[derive(Clone, Debug, Default)]
pub struct Query {
    pub view: String,
    pub id: String,
    pub q: String,
    pub kind: String,
    pub about: String,
}

impl Query {
    pub fn parse(pairs: &[(String, String)]) -> Query {
        let get = |k: &str| pairs.iter().find(|(n, _)| n == k).map(|(_, v)| v.trim().to_string()).unwrap_or_default();
        Query { view: get("view"), id: get("id"), q: get("q"), kind: get("kind"), about: get("about") }
    }
    fn view(&self) -> View {
        View::parse(&self.view)
    }
    /// This query with some parts changed, as a link.
    fn link(&self, change: &[(&str, &str)]) -> String {
        let mut parts = [("view", self.view.as_str()), ("q", self.q.as_str()), ("kind", self.kind.as_str()), ("about", self.about.as_str()), ("id", self.id.as_str())];
        for (k, v) in change {
            if let Some(p) = parts.iter_mut().find(|(n, _)| n == k) {
                p.1 = v;
            }
        }
        let qs: Vec<String> = parts.iter().filter(|(_, v)| !v.is_empty()).map(|(k, v)| format!("{k}={}", enc(v))).collect();
        format!("/?{}", qs.join("&"))
    }
    fn matches(&self, m: &Stored) -> bool {
        if !self.kind.is_empty() && m.message.kind != self.kind {
            return false;
        }
        if !self.about.is_empty() && about_key(m) != self.about {
            return false;
        }
        if self.q.is_empty() {
            return true;
        }
        let hay = format!(
            "{} {} {} {} {}",
            m.message.title,
            m.message.text,
            m.message.system.as_deref().unwrap_or(""),
            m.message.contact.as_deref().unwrap_or(""),
            m.reply.as_ref().map(|r| r.text.as_str()).unwrap_or("")
        )
        .to_lowercase();
        self.q.to_lowercase().split_whitespace().all(|w| hay.contains(w))
    }
}

fn enc(s: &str) -> String {
    s.bytes()
        .map(|b| if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) { (b as char).to_string() } else { format!("%{b:02X}") })
        .collect()
}

/// The game (or "launcher", or "recomp") a message is about, for the filter.
fn about_key(m: &Stored) -> String {
    match m.message.about.as_deref() {
        Some(a) if !a.is_empty() => a.to_string(),
        _ => m.message.kind.clone(),
    }
}

fn about_label(m: &Stored) -> String {
    about_name(m).unwrap_or_else(|| if m.message.kind == "recomp" { "New recomp".into() } else { "General".into() })
}

fn hidden(token: &str, back: &str, id: &str) -> String {
    format!(
        "<input type=hidden name=token value={token}><input type=hidden name=back value=\"{back}\"><input type=hidden name=id value=\"{id}\">",
        back = esc(back),
        id = esc(id)
    )
}

fn button(token: &str, back: &str, id: &str, act: &str, label: &str, icon: &str, class: &str) -> String {
    let confirm = match act {
        "delete" => " onsubmit=\"return confirm('Delete this message for good?')\"",
        "block" => " onsubmit=\"return confirm('Block this sender? Their messages will be dropped.')\"",
        _ => "",
    };
    format!(
        "<form method=post action=/act{confirm}>{h}<button name=do value={act} class=\"tool {class}\" title=\"{label}\">{icon}<span>{label}</span></button></form>",
        h = hidden(token, back, id)
    )
}

fn kind_short(kind: &str) -> &'static str {
    match kind {
        "idea" => "Idea",
        "recomp" => "Recomp",
        _ => "Message",
    }
}

/// One line of the list.
fn row(m: &Stored, query: &Query, open: bool) -> String {
    let msg = &m.message;
    let mut marks = String::new();
    if m.important {
        marks += &format!("<span class=\"mark star\" title=Important>{ICON_STAR}</span>");
    }
    if !m.flags.is_empty() {
        marks += &format!("<span class=\"mark warn\" title=\"{}\">{ICON_FLAG}</span>", esc(&m.flags.join(", ")));
    }
    let state = match &m.reply {
        Some(r) if r.read.is_some() => format!(
            "<span class=\"state read\" title=\"Answered, read\">{ICON_CHECK}{}</span>",
            r.reaction.as_deref().map(|e| format!("<b>{}</b>", esc(e))).unwrap_or_default()
        ),
        Some(_) => format!("<span class=\"state answered\" title=\"Answered, not read yet\">{ICON_REPLY}</span>"),
        None => String::new(),
    };
    let snippet: String = msg.text.chars().take(140).collect();
    format!(
        "<a href=\"{href}\" class=\"row {kind}{unread}{open}\" id=\"r-{id}\">\
           <span class=dot></span>\
           <span class=rmain><span class=rtop><span class=rtitle>{title}</span>{marks}<time data-t={t} data-short></time></span>\
           <span class=rsub><span class=\"pill {kind}\">{kname}</span><span class=rabout>{about}</span>{state}</span>\
           <span class=rsnip>{snip}</span></span>\
         </a>",
        href = esc(&query.link(&[("id", &m.id)])),
        kind = esc(&msg.kind),
        unread = if m.read { "" } else { " unread" },
        open = if open { " on" } else { "" },
        id = esc(&m.id),
        title = esc(&msg.title),
        t = m.received,
        kname = kind_short(&msg.kind),
        about = esc(&about_label(m)),
        snip = esc(&snippet),
    )
}

/// The answer under the open message: a box to write it, or the answer
/// sent, with whether the user has collected it, read it, and their emoji.
fn answer(m: &Stored, token: &str, back: &str) -> String {
    let form = |text: &str, label: &str| {
        format!(
            "<form method=post action=/act class=compose>{h}\
             <textarea name=text maxlength={max} placeholder=\"Write an answer. Only the launcher that sent this sees it, the next time it starts or refreshes.\">{text}</textarea>\
             <div class=crow><span class=dim>Plain text · one answer, they cannot write back</span><button name=do value=reply class=\"btn primary\">{ICON_SEND}{label}</button></div></form>",
            h = hidden(token, back, &m.id),
            max = crate::store::REPLY_MAX,
            text = esc(text)
        )
    };
    match &m.reply {
        None => format!("<section class=answer><h3>Answer</h3>{}</section>", form("", "Send answer")),
        Some(r) => {
            let steps = [
                ("Sent", Some(r.at)),
                ("Collected", r.delivered),
                ("Read", r.read),
            ];
            let track: String = steps
                .iter()
                .map(|(label, t)| match t {
                    Some(t) => format!("<li class=done>{ICON_CHECK}<span>{label}</span><time data-t={t}></time></li>"),
                    None => format!("<li><span class=ring></span><span>{label}</span><time></time></li>"),
                })
                .collect();
            let reaction = r.reaction.as_deref().map(|e| format!("<div class=reaction title=\"Their reaction\">{}</div>", esc(e))).unwrap_or_default();
            let safe = if r.read.is_some() { "<p class=\"hint ok\">Read: you can archive or delete this message, their launcher keeps its copy.</p>" } else { "" };
            let remove = format!(
                "<form method=post action=/act onsubmit=\"return confirm('Remove this answer? If it was not collected yet, it never arrives.')\">{h}<button name=do value=unreply class=\"btn ghost bad\">Remove answer</button></form>",
                h = hidden(token, back, &m.id)
            );
            format!(
                "<section class=answer><h3>Your answer</h3>\
                 <div class=sent><div class=bubble><p>{text}</p></div>{reaction}</div>\
                 <ol class=track>{track}</ol>{safe}\
                 <details><summary>Change the answer</summary>{change}</details>\
                 <div class=aend>{remove}</div></section>",
                text = esc(&r.text).replace('\n', "</p><p>"),
                change = form(&r.text, "Save answer"),
            )
        }
    }
}

/// The open message.
fn detail(m: &Stored, token: &str, query: &Query, blocked: &Blocked) -> String {
    let msg = &m.message;
    let back = query.link(&[]);
    let is_blocked = blocked.keys.iter().any(|k| *k == msg.sender || *k == m.address);
    let mut tools = String::new();
    tools += &if m.important {
        button(token, &back, &m.id, "unimportant", "Important", ICON_STAR, "starred")
    } else {
        button(token, &back, &m.id, "important", "Mark important", ICON_STAR, "")
    };
    tools += &button(token, &back, &m.id, "unread", "Mark unread", ICON_DOT, "");
    tools += &if m.archived {
        button(token, &back, &m.id, "unarchive", "Back to inbox", ICON_INBOX, "")
    } else {
        button(token, &back, &m.id, "archive", "Archive", ICON_ARCHIVE, "")
    };
    if !is_blocked {
        tools += &button(token, &back, &m.id, "block", "Block sender", ICON_BAN, "warn");
    }
    tools += &button(token, &back, &m.id, "delete", "Delete", ICON_TRASH, "bad");
    let flags: String = m.flags.iter().map(|f| format!("<span class=flag>{ICON_FLAG}{}</span>", esc(f))).collect();
    let title = match msg.system.as_deref().filter(|s| !s.is_empty()) {
        Some(s) => format!("{} <span class=dim>· {}</span>", esc(&msg.title), esc(s)),
        None => esc(&msg.title),
    };
    let contact = msg.contact.as_deref().filter(|c| !c.is_empty())
        .map(|c| format!("<div class=contact><span class=dim>Contact</span> <b>{}</b></div>", esc(c))).unwrap_or_default();
    let details: String = msg.details.iter().map(|(k, v)| format!("<dt>{}</dt><dd>{}</dd>", esc(k), esc(v))).collect();
    let late = msg.written.filter(|w| m.received.saturating_sub(*w) > 600)
        .map(|w| format!(" · written <time data-t={w}></time>")).unwrap_or_default();
    let text = if msg.text.trim().is_empty() { "<p class=dim>(no text)</p>".to_string() } else { format!("<p>{}</p>", esc(&msg.text).replace('\n', "</p><p>")) };
    format!(
        "<article class=\"detail {kind}\">\
           <a class=back href=\"{list}\">{ICON_BACK}Back</a>\
           <div class=toolbar>{tools}</div>\
           <header><div class=dtags><span class=\"pill {kind}\">{kname}</span><span class=rabout>{about}</span>{flags}{blocked_note}</div>\
           <h1>{title}</h1>\
           <div class=meta><time data-t={received}></time>{late}</div></header>\
           <div class=body>{text}</div>{contact}\
           <dl class=specs>{details}</dl>\
           <div class=sender>Sender <code>{sender}</code> · {address}{lang}</div>\
           {answer}\
         </article>",
        kind = esc(&msg.kind),
        list = esc(&query.link(&[("id", "")])),
        kname = kind_name(&msg.kind),
        about = esc(&about_label(m)),
        blocked_note = if is_blocked { "<span class=flag>blocked</span>" } else { "" },
        received = m.received,
        sender = esc(&msg.sender.chars().take(8).collect::<String>()),
        address = esc(&m.address),
        lang = msg.lang.as_deref().map(|l| format!(" · {}", esc(l))).unwrap_or_default(),
        answer = if is_blocked { String::new() } else { answer(m, token, &back) },
    )
}

pub fn page(all: &[Stored], blocked: &Blocked, query: &Query, token: &str) -> String {
    let view = query.view();
    let count = |v: View| all.iter().filter(|m| v.shows(m)).count();
    let folders: String = VIEWS
        .iter()
        .map(|&v| {
            let n = if v == View::Blocked { blocked.keys.len() } else { count(v) };
            let strong = matches!(v, View::Unread | View::ToAnswer) && n > 0;
            format!(
                "<a href=\"{href}\" class=\"folder{on}\">{icon}<span>{label}</span><span class=\"n{strong}\">{n}</span></a>",
                href = esc(&Query { view: v.key().into(), ..Default::default() }.link(&[])),
                on = if v == view { " on" } else { "" },
                icon = v.icon(),
                label = v.label(),
                strong = if strong { " strong" } else { "" },
            )
        })
        .collect();
    // Kinds and games to narrow the list, from the messages there are
    let mut abouts: Vec<(String, String)> = Vec::new();
    for m in all {
        let k = about_key(m);
        if !abouts.iter().any(|(a, _)| *a == k) {
            abouts.push((k, about_label(m)));
        }
    }
    abouts.sort_by(|a, b| a.1.cmp(&b.1));
    let chip = |param: &str, value: &str, label: &str, on: bool| {
        format!(
            "<a href=\"{}\" class=\"chip{}\">{}</a>",
            esc(&query.link(&[(param, if on { "" } else { value }), ("id", "")])),
            if on { " on" } else { "" },
            esc(label)
        )
    };
    let kinds: String = [("idea", "Ideas"), ("recomp", "Recomps"), ("talk", "Messages")]
        .iter()
        .map(|(k, l)| chip("kind", k, l, query.kind == *k))
        .collect();
    let games: String = abouts.iter().map(|(k, l)| chip("about", k, l, query.about == *k)).collect();

    let open = all.iter().find(|m| m.id == query.id);
    let (list, right) = if view == View::Blocked {
        let rows: String = if blocked.keys.is_empty() {
            "<p class=empty>Nobody is blocked.</p>".into()
        } else {
            blocked.keys.iter().map(|k| format!(
                "<div class=blockrow><code>{}</code>{}</div>", esc(k), button(token, &query.link(&[]), k, "unblock", "Unblock", ICON_BAN, ""))).collect()
        };
        (rows, "<div class=placeholder>Blocked senders are told their messages were sent, and the messages are dropped.</div>".to_string())
    } else {
        let shown: Vec<&Stored> = all.iter().filter(|m| view.shows(m) && query.matches(m)).collect();
        let rows: String = if shown.is_empty() {
            "<p class=empty>Nothing here.</p>".into()
        } else {
            shown.iter().map(|m| row(m, query, open.is_some_and(|o| o.id == m.id))).collect()
        };
        let bar = if view == View::Unread && !shown.is_empty() {
            format!("<div class=listbar>{}</div>", button(token, &query.link(&[]), "", "allread", "Mark all read", ICON_CHECK, ""))
        } else {
            String::new()
        };
        let right = match open {
            Some(m) => detail(m, token, query, blocked),
            None => format!("<div class=placeholder>{ICON_INBOX}<p>Choose a message to read it.</p></div>"),
        };
        (format!("{bar}{rows}"), right)
    };
    let unread = count(View::Unread);
    format!(
        "<!doctype html><html lang=en><head><meta charset=utf-8><meta name=viewport content=\"width=device-width,initial-scale=1\">\
         <title>{title}</title><style>{STYLE}</style></head><body class=\"{state}\">\
         <aside class=side><div class=brand><span class=logo>{ICON_INBOX}</span><b>Nico inbox</b></div>\
           <nav>{folders}</nav>\
           <div class=filters><h4>Kind</h4><div class=chips>{kinds}</div><h4>About</h4><div class=chips>{games}</div></div></aside>\
         <section class=list><form class=search method=get action=/>\
           <input type=hidden name=view value=\"{viewkey}\"><input type=hidden name=kind value=\"{kind}\"><input type=hidden name=about value=\"{about}\">\
           {ICON_SEARCH}<input type=search name=q value=\"{q}\" placeholder=\"Search messages and answers\"></form>\
           <div class=listhead><h2>{vlabel}</h2><span class=dim>{shown_n}</span></div>\
           <div class=rows>{list}</div></section>\
         <main class=pane>{right}</main>\
         <script>{SCRIPT}</script></body></html>",
        title = if unread > 0 { format!("({unread}) Nico inbox") } else { "Nico inbox".into() },
        state = if open.is_some() { "reading" } else { "" },
        viewkey = view.key(),
        kind = esc(&query.kind),
        about = esc(&query.about),
        q = esc(&query.q),
        vlabel = view.label(),
        shown_n = if view == View::Blocked { blocked.keys.len() } else { all.iter().filter(|m| view.shows(m) && query.matches(m)).count() },
    )
}

const SCRIPT: &str = r#"
const fmt=(t,short)=>{const d=new Date(t*1000),now=new Date();
 if(short){if(d.toDateString()===now.toDateString())return d.toLocaleTimeString([],{hour:'2-digit',minute:'2-digit'});
  return d.toLocaleDateString([],d.getFullYear()===now.getFullYear()?{day:'numeric',month:'short'}:{day:'numeric',month:'short',year:'numeric'});}
 return d.toLocaleString([],{dateStyle:'medium',timeStyle:'short'});};
for(const t of document.querySelectorAll('time[data-t]'))t.textContent=fmt(+t.dataset.t,t.hasAttribute('data-short'));
document.querySelector('.row.on')?.scrollIntoView({block:'nearest'});
document.addEventListener('keydown',e=>{if(e.target.closest('input,textarea'))return;
 const rows=[...document.querySelectorAll('.row')],i=rows.findIndex(r=>r.classList.contains('on'));
 if(e.key==='j'&&rows[i+1])location=rows[i+1].href;if(e.key==='k'&&i>0)location=rows[i-1].href;
 if(e.key==='/'){e.preventDefault();document.querySelector('.search input[type=search]').focus();}});
"#;

const ICON_INBOX: &str = r#"<svg viewBox="0 0 24 24"><path d="M3 13l3-8h12l3 8v5a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><path d="M3 13h5l1.5 2.5h5L16 13h5"/></svg>"#;
const ICON_DOT: &str = r#"<svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="4.5" fill="currentColor"/></svg>"#;
const ICON_STAR: &str = r#"<svg viewBox="0 0 24 24"><path d="M12 3.5l2.6 5.400 5.900.8-4.300 4.100 1 5.800L12 16.800 6.800 19.600l1-5.800L3.500 9.700l5.900-.8z"/></svg>"#;
const ICON_REPLY: &str = r#"<svg viewBox="0 0 24 24"><path d="M10 6L4 12l6 6"/><path d="M4 12h10a6 6 0 0 1 6 6v1"/></svg>"#;
const ICON_CHECK: &str = r#"<svg viewBox="0 0 24 24"><path d="M5 12.5l4.500 4.500L19 7"/></svg>"#;
const ICON_FLAG: &str = r#"<svg viewBox="0 0 24 24"><path d="M5 21V4M5 4h11l-2 4 2 4H5"/></svg>"#;
const ICON_ARCHIVE: &str = r#"<svg viewBox="0 0 24 24"><rect x="3" y="4" width="18" height="4" rx="1"/><path d="M5 8v11a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V8M10 12h4"/></svg>"#;
const ICON_BAN: &str = r#"<svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="8.500"/><path d="M6 6l12 12"/></svg>"#;
const ICON_TRASH: &str = r#"<svg viewBox="0 0 24 24"><path d="M4 7h16M10 11v6M14 11v6M6 7l1 13h10l1-13M9 7V4h6v3"/></svg>"#;
const ICON_SEND: &str = r#"<svg viewBox="0 0 24 24"><path d="M21 3L10 14M21 3l-7 18-4-7-7-4z"/></svg>"#;
const ICON_SEARCH: &str = r#"<svg viewBox="0 0 24 24"><circle cx="11" cy="11" r="7"/><path d="M20 20l-4-4"/></svg>"#;
const ICON_BACK: &str = r#"<svg viewBox="0 0 24 24"><path d="M15 5l-7 7 7 7"/></svg>"#;

const STYLE: &str = r#"
:root{--bg:#0b0f19;--side:#0e1320;--surface:#131a2a;--raise:#18213a;--line:#222c44;--line2:#2d3a5a;--text:#e8ecf5;--dim:#8a94ab;--accent:#4f8cff;--good:#3ddc97;--bad:#ff6b6b;--warn:#ffb84d;--idea:255,184,77;--recomp:167,128,255;--talk:61,220,151}
*{box-sizing:border-box}html,body{height:100%}
body{margin:0;background:var(--bg);color:var(--text);font:14px/1.5 "Segoe UI Variable Text","Segoe UI",Inter,system-ui,sans-serif;display:grid;grid-template-columns:220px minmax(300px,400px) 1fr;overflow:hidden}
svg{width:16px;height:16px;fill:none;stroke:currentColor;stroke-width:1.8;stroke-linecap:round;stroke-linejoin:round;flex:none}
a{color:inherit;text-decoration:none}.dim{color:var(--dim)}code{font:12px ui-monospace,Consolas,monospace;color:var(--dim)}
.side{background:var(--side);border-right:1px solid var(--line);padding:18px 12px;overflow-y:auto}
.brand{display:flex;align-items:center;gap:10px;padding:0 8px 18px;font-size:16px}.logo{width:30px;height:30px;border-radius:9px;display:grid;place-items:center;background:linear-gradient(135deg,#4f8cff,#7a5cff);color:#fff}.logo svg{width:17px;height:17px}
nav{display:grid;gap:2px}.folder{display:flex;align-items:center;gap:10px;padding:8px 10px;border-radius:9px;color:var(--dim)}
.folder:hover{background:rgba(255,255,255,.04);color:var(--text)}.folder.on{background:rgba(79,140,255,.15);color:#fff}.folder.on svg{color:var(--accent)}
.folder .n{margin-left:auto;font-size:12px;opacity:.7}.folder .n.strong{opacity:1;color:#fff;background:var(--accent);border-radius:999px;padding:0 7px;font-weight:600}
.filters{margin-top:22px;padding:0 4px}.filters h4{margin:14px 6px 8px;font-size:11px;letter-spacing:.08em;text-transform:uppercase;color:var(--dim);font-weight:600}
.chips{display:flex;flex-wrap:wrap;gap:6px}.chip{font-size:12.5px;padding:3px 10px;border-radius:999px;border:1px solid var(--line2);color:var(--dim)}.chip:hover{color:var(--text)}.chip.on{background:var(--accent);border-color:var(--accent);color:#fff}
.list{border-right:1px solid var(--line);display:flex;flex-direction:column;min-height:0;background:var(--bg)}
.search{display:flex;align-items:center;gap:8px;margin:14px 14px 6px;padding:0 12px;height:38px;border-radius:10px;background:var(--surface);border:1px solid var(--line);color:var(--dim)}
.search:focus-within{border-color:var(--accent)}.search input{flex:1;background:none;border:0;outline:0;color:var(--text);font:inherit}
.listhead{display:flex;align-items:baseline;gap:8px;padding:8px 18px 8px}.listhead h2{margin:0;font-size:17px}
.listbar{display:flex;justify-content:flex-end;padding:0 14px 6px}
.rows{overflow-y:auto;flex:1;padding:0 8px 20px}
.row{display:flex;gap:10px;padding:11px 10px;border-radius:11px;margin-bottom:2px;border:1px solid transparent}
.row:hover{background:rgba(255,255,255,.03)}.row.on{background:var(--raise);border-color:var(--line2)}
.dot{width:8px;height:8px;border-radius:50%;margin-top:7px;flex:none}.row.unread .dot{background:var(--accent);box-shadow:0 0 0 3px rgba(79,140,255,.18)}
.rmain{min-width:0;flex:1;display:grid;grid-template-columns:minmax(0,1fr);gap:2px}.rtop{display:flex;align-items:center;gap:6px;min-width:0}.rtop svg,.rtop .mark{flex:none}
.rtitle{min-width:0;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;color:var(--dim)}.row.unread .rtitle{color:var(--text);font-weight:600}
.rtop time{margin-left:auto;font-size:12px;color:var(--dim);white-space:nowrap}
.rsub{display:flex;align-items:center;gap:7px;font-size:12px;color:var(--dim)}
.rsnip{font-size:12.5px;color:var(--dim);overflow:hidden;text-overflow:ellipsis;white-space:nowrap;opacity:.8}
.pill{font-size:11px;font-weight:600;padding:0 7px;border-radius:6px;color:rgb(var(--tone));background:rgba(var(--tone),.13);border:1px solid rgba(var(--tone),.3);--tone:79,140,255}
.pill.idea,.detail.idea{--tone:var(--idea)}.pill.recomp,.detail.recomp{--tone:var(--recomp)}.pill.talk,.detail.talk{--tone:var(--talk)}
.mark svg{width:14px;height:14px}.mark.star{color:var(--warn)}.mark.star svg{fill:currentColor}.mark.warn{color:var(--warn)}
.state{display:inline-flex;align-items:center;gap:3px;margin-left:auto}.state svg{width:14px;height:14px}.state.answered{color:var(--accent)}.state.read{color:var(--good)}.state b{font-weight:400;font-size:13px}
.empty{color:var(--dim);text-align:center;padding:40px 0}
.pane{overflow-y:auto;min-width:0}
.placeholder{height:100%;display:grid;place-content:center;justify-items:center;gap:10px;color:var(--dim);padding:30px;text-align:center}.placeholder svg{width:42px;height:42px;opacity:.5}
.detail{max-width:860px;margin:0 auto;padding:18px 34px 60px}
.back{display:none;align-items:center;gap:4px;color:var(--accent);margin-bottom:8px}
.toolbar{display:flex;flex-wrap:wrap;gap:6px;padding-bottom:14px;border-bottom:1px solid var(--line);position:sticky;top:0;background:var(--bg);z-index:2;padding-top:4px}
form{margin:0}
.tool{display:inline-flex;align-items:center;gap:7px;font:inherit;font-size:13px;color:var(--text);background:var(--surface);border:1px solid var(--line);border-radius:9px;padding:6px 11px;cursor:pointer}
.tool:hover{border-color:var(--line2);background:var(--raise)}.tool.starred{color:var(--warn);border-color:rgba(255,184,77,.45)}.tool.starred svg{fill:currentColor}
.tool.warn{color:var(--warn)}.tool.bad{color:var(--bad)}
.detail header{padding:20px 0 6px}.dtags{display:flex;align-items:center;gap:8px;flex-wrap:wrap;font-size:13px;color:var(--dim)}
.flag{display:inline-flex;align-items:center;gap:4px;color:var(--warn);border:1px solid rgba(255,184,77,.45);padding:0 7px;border-radius:6px;font-size:12px}.flag svg{width:12px;height:12px}
.detail h1{font-size:24px;line-height:1.25;margin:10px 0 4px}.meta{color:var(--dim);font-size:13px}
.body{font-size:15.5px;line-height:1.65;margin:16px 0;padding-left:14px;border-left:3px solid rgba(var(--tone),.6)}.body p{margin:0 0 10px;overflow-wrap:anywhere}
.contact{margin:12px 0;font-size:14px}
.specs{display:grid;grid-template-columns:max-content 1fr;gap:3px 18px;font-size:13px;margin:16px 0 8px;padding:12px 14px;border-radius:11px;background:var(--surface);border:1px solid var(--line)}.specs dt{color:var(--dim)}.specs dd{margin:0}
.sender{font-size:12px;color:var(--dim)}
.answer{margin-top:24px;padding-top:18px;border-top:1px solid var(--line)}.answer h3{margin:0 0 10px;font-size:13px;letter-spacing:.06em;text-transform:uppercase;color:var(--dim)}
.compose textarea{width:100%;min-height:120px;resize:vertical;font:inherit;font-size:14.5px;color:var(--text);background:var(--surface);border:1px solid var(--line);border-radius:12px;padding:12px 14px}
.compose textarea:focus{outline:none;border-color:var(--accent)}.crow{display:flex;align-items:center;gap:10px;margin-top:10px;font-size:12.5px}.crow .btn{margin-left:auto}
.btn{display:inline-flex;align-items:center;gap:8px;font:inherit;font-weight:600;font-size:13.5px;border-radius:10px;padding:8px 14px;cursor:pointer;border:1px solid var(--line);background:var(--surface);color:var(--text)}
.btn.primary{background:var(--accent);border-color:var(--accent);color:#fff}.btn.primary:hover{filter:brightness(1.1)}.btn.ghost{background:none}.btn.bad{color:var(--bad)}
.sent{display:flex;align-items:flex-end;gap:10px}.bubble{flex:1;padding:12px 16px;border-radius:14px 14px 4px 14px;background:rgba(61,220,151,.1);border:1px solid rgba(61,220,151,.35)}.bubble p{margin:0 0 8px;overflow-wrap:anywhere}.bubble p:last-child{margin:0}
.reaction{font-size:30px;line-height:1;padding:4px 6px;border-radius:12px;background:var(--surface);border:1px solid var(--line)}
.track{list-style:none;display:flex;gap:6px;flex-wrap:wrap;padding:0;margin:12px 0 0}.track li{display:flex;align-items:center;gap:6px;font-size:12.5px;color:var(--dim);padding:4px 10px;border-radius:999px;border:1px solid var(--line)}
.track li.done{color:var(--good);border-color:rgba(61,220,151,.35)}.track li time{color:var(--dim)}.track li time:empty{display:none}.track svg{width:13px;height:13px}
.ring{width:9px;height:9px;border-radius:50%;border:1.5px solid var(--dim)}
.hint{font-size:13px;margin:10px 0 0}.hint.ok{color:var(--good)}
.answer details{margin-top:14px}.answer summary{cursor:pointer;color:var(--accent);font-size:13px}.answer details .compose{margin-top:10px}
.aend{display:flex;justify-content:flex-end;margin-top:10px}
.blockrow{display:flex;align-items:center;justify-content:space-between;gap:10px;background:var(--surface);border:1px solid var(--line);border-radius:10px;padding:8px 12px;margin:0 6px 8px}
@media (max-width:1100px){body{grid-template-columns:64px minmax(280px,360px) 1fr}.side{padding:14px 8px}.brand b,.folder span:not(.n),.filters{display:none}.folder{justify-content:center;position:relative}.folder .n{position:absolute;top:2px;right:2px;font-size:10px}.folder .n:not(.strong){display:none}.brand{justify-content:center;padding:0 0 14px}}
@media (max-width:760px){body{display:block;overflow:auto}.side{position:sticky;top:0;z-index:5;padding:8px;border-right:0;border-bottom:1px solid var(--line)}.brand{display:none}nav{display:flex;overflow-x:auto;gap:4px}.folder{flex:none}
 .list{border-right:0}.rows{overflow:visible}body.reading .list{display:none}body:not(.reading) .pane{display:none}.detail{padding:14px 16px 50px}.back{display:inline-flex}.tool span{display:none}.tool{padding:7px 9px}.detail h1{font-size:20px}}
"#;
