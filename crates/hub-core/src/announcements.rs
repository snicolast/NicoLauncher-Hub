//! Announcements from the author: help wanted, a call for testers, something
//! every user must know. They are not news of a game. Read from
//! `announcements.json` in the launcher's public repository; the format is in
//! `docs/announcements.md`.

use crate::{err, http_get, Result};
use serde::{Deserialize, Serialize};

pub const URL: &str = "https://raw.githubusercontent.com/snicolast/NicoLauncher-Hub/main/announcements.json";

/// At most this many link buttons; "Close" is always added by the launcher.
const MAX_BUTTONS: usize = 2;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Announcement {
    /// Never reused: the launcher remembers which ids the user closed.
    pub id: String,
    /// YYYY-MM-DD, shown in the window.
    #[serde(default)]
    pub date: String,
    /// YYYY-MM-DD: from that day on it is not shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// `info`, or `important` (opens by itself at start).
    #[serde(default = "info")]
    pub level: String,
    /// A project id: shown only to users who have that game installed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub game: Option<String>,
    /// The short tag on the megaphone ("Help wanted").
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub title: String,
    /// A picture at the top of the window: a path in this repository
    /// (`art/announcements/...`) or a raw.githubusercontent.com URL. Read as
    /// a URL; anything else is dropped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// A YouTube video played in the window: its link (watch?v=, youtu.be/,
    /// shorts/) or its 11-character id. A Shorts link plays upright, beside
    /// the text. A link that cannot be read is dropped. Launchers older than
    /// 0.3.27 ignore it: give the announcement a picture and a button to the
    /// video for them.
    #[serde(default, deserialize_with = "video_link", skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    /// Plain text: a line is a paragraph, "- " starts an item of a list.
    #[serde(default)]
    pub body: Vec<String>,
    #[serde(default)]
    pub buttons: Vec<Button>,
    /// A launcher version: only launchers older than it show the
    /// announcement ("update the launcher to use..."). Launchers older than
    /// this key ignore it and show the announcement, which is the point.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub before_launcher: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Button {
    pub label: String,
    pub url: String,
}

/// A YouTube video, as the window plays it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Video {
    /// The 11 characters YouTube names it by.
    pub id: String,
    /// A Short: taller than wide.
    pub vertical: bool,
    /// Its page on YouTube, for the browser.
    pub url: String,
}

impl Video {
    pub fn from_link(link: &str) -> Option<Video> {
        let link = link.trim();
        let ok = |id: &str| id.len() == 11 && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        if ok(link) {
            return Some(Video::new(link, false));
        }
        let rest = link.strip_prefix("https://").or_else(|| link.strip_prefix("http://"))?;
        let (host, path) = rest.split_once('/')?;
        let host = host.strip_prefix("www.").or_else(|| host.strip_prefix("m.")).unwrap_or(host);
        let (id, vertical) = match host {
            "youtu.be" => (path.split(['?', '&', '#', '/']).next()?, false),
            "youtube.com" => {
                if let Some(id) = path.strip_prefix("shorts/") {
                    (id.split(['?', '&', '#', '/']).next()?, true)
                } else {
                    let query = path.strip_prefix("watch?")?;
                    (query.split(['&', '#']).find_map(|kv| kv.strip_prefix("v="))?, false)
                }
            }
            _ => return None,
        };
        ok(id).then(|| Video::new(id, vertical))
    }

    fn new(id: &str, vertical: bool) -> Video {
        let url = if vertical { format!("https://www.youtube.com/shorts/{id}") } else { format!("https://www.youtube.com/watch?v={id}") };
        Video { id: id.to_string(), vertical, url }
    }
}

/// `video` in the file is a link; one that cannot be read leaves the
/// announcement without a video, never without the announcement.
fn video_link<'de, D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Option<Video>, D::Error> {
    let v = serde_json::Value::deserialize(d)?;
    Ok(match v {
        serde_json::Value::String(link) => Video::from_link(&link),
        // What the launcher itself writes back (a copy of what it read)
        serde_json::Value::Object(_) => serde_json::from_value::<Video>(v).ok().and_then(|x| Video::from_link(&x.url)),
        _ => None,
    })
}

fn info() -> String {
    "info".into()
}

#[derive(Deserialize)]
struct File {
    #[serde(default)]
    announcements: Vec<serde_json::Value>,
}

/// The announcements in the file, newest first. One that cannot be read (a
/// missing key, a typo) is left out without hiding the others; unknown keys
/// are ignored, so later versions can add some.
pub fn parse(text: &str) -> Result<Vec<Announcement>> {
    let file: File = serde_json::from_str(text).map_err(err("Bad announcements file"))?;
    let mut list: Vec<Announcement> = file
        .announcements
        .into_iter()
        .filter_map(|v| serde_json::from_value::<Announcement>(v).ok())
        .filter(|a| !a.id.trim().is_empty() && !a.title.trim().is_empty())
        .filter(|a| a.before_launcher.as_deref().is_none_or(|v| older(env!("CARGO_PKG_VERSION"), v)))
        .map(|mut a| {
            if a.level != "important" {
                a.level = info();
            }
            // Only web pages, opened in the browser: nothing else is a link
            a.buttons.retain(|b| b.url.starts_with("https://") && !b.label.trim().is_empty());
            a.buttons.truncate(MAX_BUTTONS);
            a.image = a.image.as_deref().and_then(image_url);
            a
        })
        .collect();
    let mut seen = std::collections::HashSet::new();
    list.retain(|a| seen.insert(a.id.clone()));
    list.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(list)
}

/// Where the window loads a picture from. Only GitHub's raw files: the
/// launcher's page refuses pictures from anywhere else, and a picture in this
/// repository is reviewed like the rest of the announcement.
/// Whether version `a` comes before `b` ("0.3.9" before "0.3.21"). A version
/// that cannot be read is not older: the announcement is not shown.
fn older(a: &str, b: &str) -> bool {
    let nums = |v: &str| v.trim().trim_start_matches('v').split('.').map(|n| n.parse::<u32>().ok()).collect::<Option<Vec<u32>>>();
    matches!((nums(a), nums(b)), (Some(a), Some(b)) if a < b)
}

fn image_url(image: &str) -> Option<String> {
    let image = image.trim();
    if let Some(rest) = image.strip_prefix(IMAGE_HOST) {
        return crate::registry::is_plain_relative(rest).then(|| image.to_string());
    }
    crate::registry::is_plain_relative(image).then(|| crate::registry::hub_raw_url(&image.replace('\\', "/")))
}

const IMAGE_HOST: &str = "https://raw.githubusercontent.com/";

pub fn fetch() -> Result<Vec<Announcement>> {
    parse(&http_get(URL)?.into_string().map_err(err("Bad announcements file"))?)
}

/// Today as YYYY-MM-DD (UTC).
pub fn today() -> String {
    crate::update::timestamp()[..10].to_string()
}

/// Whether the user should see it: not expired on `today`, and, when it is
/// about one game, that game is installed.
pub fn shown(a: &Announcement, today: &str, installed: impl Fn(&str) -> bool) -> bool {
    a.expires.as_deref().is_none_or(|e| today < e) && a.game.as_deref().is_none_or(installed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
      "schema": 1,
      "announcements": [
        { "id": "old", "date": "2026-10-01", "title": "Old one", "body": ["Hello"] },
        { "id": "help", "date": "2026-11-14", "expires": "2027-03-01", "level": "info", "label": "Help wanted",
          "title": "Help wanted", "body": ["Text", "- item"], "something_new": true,
          "buttons": [ { "label": "I can help", "url": "https://github.com/x" },
                       { "label": "Bad", "url": "file:///C:/Windows" },
                       { "label": "Two", "url": "https://a" }, { "label": "Three", "url": "https://b" } ] },
        { "id": "sr2", "date": "2026-11-13", "level": "urgent!!", "game": "segarally2", "title": "Testers",
          "image": "art/announcements/sr2-deck.jpg" },
        { "id": "", "title": "No id" },
        { "id": "broken", "title": 5 },
        { "id": "help", "date": "2026-11-15", "title": "Same id again" }
      ]
    }"#;

    #[test]
    fn parses_newest_first_and_skips_bad_entries() {
        let list = parse(SAMPLE).unwrap();
        let ids: Vec<&str> = list.iter().map(|a| a.id.as_str()).collect();
        assert_eq!(ids, ["help", "sr2", "old"]);
        assert_eq!(list[0].title, "Help wanted");
        assert_eq!(list[0].body, ["Text", "- item"]);
    }

    #[test]
    fn keeps_two_https_buttons_and_known_levels() {
        let list = parse(SAMPLE).unwrap();
        let labels: Vec<&str> = list[0].buttons.iter().map(|b| b.label.as_str()).collect();
        assert_eq!(labels, ["I can help", "Two"]);
        assert_eq!(list[1].level, "info");
        assert_eq!(list[2].level, "info");
    }

    #[test]
    fn images_come_from_github_only() {
        let list = parse(SAMPLE).unwrap();
        assert_eq!(list[0].image, None);
        assert_eq!(
            list[1].image.as_deref(),
            Some("https://raw.githubusercontent.com/snicolast/NicoLauncher-Hub/main/art/announcements/sr2-deck.jpg")
        );
        let one = |image: &str| {
            let text = format!(r#"{{"announcements":[{{"id":"a","title":"A","image":{image:?}}}]}}"#);
            parse(&text).unwrap().remove(0).image
        };
        let raw = "https://raw.githubusercontent.com/snicolast/SegaRally2Native/main/docs/img/hero.jpg";
        assert_eq!(one(raw).as_deref(), Some(raw));
        assert_eq!(one("http://raw.githubusercontent.com/a/b/main/x.png"), None);
        assert_eq!(one("https://example.com/x.png"), None);
        assert_eq!(one("https://raw.githubusercontent.com/../x.png"), None);
        assert_eq!(one("../x.png"), None);
        assert_eq!(one("/x.png"), None);
        assert_eq!(one("C:\\x.png"), None);
        assert_eq!(one("file:///C:/x.png"), None);
        assert_eq!(one(""), None);
    }

    #[test]
    fn videos_from_youtube_links() {
        let one = |video: &str| {
            let text = format!(r#"{{"announcements":[{{"id":"a","title":"A","video":{video:?}}}]}}"#);
            parse(&text).unwrap().remove(0).video
        };
        let short = one("https://www.youtube.com/shorts/abcDEF123_-?feature=share").unwrap();
        assert_eq!((short.id.as_str(), short.vertical), ("abcDEF123_-", true));
        assert_eq!(short.url, "https://www.youtube.com/shorts/abcDEF123_-");
        for wide in ["https://www.youtube.com/watch?v=abcDEF123_-&t=10", "https://youtu.be/abcDEF123_-?si=x", "https://m.youtube.com/watch?feature=x&v=abcDEF123_-", "abcDEF123_-"] {
            let v = one(wide).unwrap();
            assert_eq!((v.id.as_str(), v.vertical), ("abcDEF123_-", false), "{wide}");
            assert_eq!(v.url, "https://www.youtube.com/watch?v=abcDEF123_-");
        }
        for bad in ["", "https://example.com/watch?v=abcDEF123_-", "https://www.youtube.com/shorts/short", "https://www.youtube.com/watch?v=abc\"/x", "javascript:alert(1)", "https://www.youtube.com/@snicolast"] {
            assert_eq!(one(bad), None, "{bad}");
        }
        // Not a string: no video, but the announcement stays
        let list = parse(r#"{"announcements":[{"id":"a","title":"A","video":5}]}"#).unwrap();
        assert_eq!(list[0].video, None);
        // What the launcher sends to its window reads back the same
        let back: Announcement = serde_json::from_str(&serde_json::to_string(&parse(r#"{"announcements":[{"id":"a","title":"A","video":"https://youtube.com/shorts/abcDEF123_-"}]}"#).unwrap()[0]).unwrap()).unwrap();
        assert!(back.video.unwrap().vertical);
    }

    #[test]
    fn hides_expired_and_games_not_installed() {
        let list = parse(SAMPLE).unwrap();
        let none = |_: &str| false;
        assert!(shown(&list[0], "2026-11-14", none));
        assert!(shown(&list[0], "2027-02-28", none));
        assert!(!shown(&list[0], "2027-03-01", none));
        assert!(!shown(&list[1], "2026-11-14", none));
        assert!(shown(&list[1], "2026-11-14", |id| id == "segarally2"));
    }

    #[test]
    fn a_broken_file_is_an_error() {
        assert!(parse("not json").is_err());
        assert!(parse("{}").unwrap().is_empty());
    }

    #[test]
    fn the_files_in_the_repository_parse() {
        parse(include_str!("../../../announcements.json")).unwrap();
        assert_eq!(parse(include_str!("../../../examples/announcements.json")).unwrap().len(), 2);
    }

    #[test]
    fn today_is_a_date() {
        let t = today();
        assert_eq!(t.len(), 10);
        assert_eq!(&t[4..5], "-");
    }

    #[test]
    fn before_launcher_is_for_older_launchers_only() {
        assert!(older("0.3.9", "0.3.21"));
        assert!(older("0.3.20", "0.3.21"));
        assert!(!older("0.3.21", "0.3.21"));
        assert!(!older("1.0.0", "0.3.21"));
        assert!(!older("0.3.20", "soon"));
        let json = |v: &str| format!(r#"{{"announcements":[{{"id":"a","date":"2026-11-24","title":"Update","before_launcher":"{v}"}}]}}"#);
        assert!(parse(&json("999.0.0")).unwrap().len() == 1);
        assert!(parse(&json(env!("CARGO_PKG_VERSION"))).unwrap().is_empty());
    }
}
