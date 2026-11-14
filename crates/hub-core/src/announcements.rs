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
    /// Plain text: a line is a paragraph, "- " starts an item of a list.
    #[serde(default)]
    pub body: Vec<String>,
    #[serde(default)]
    pub buttons: Vec<Button>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Button {
    pub label: String,
    pub url: String,
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
        .map(|mut a| {
            if a.level != "important" {
                a.level = info();
            }
            // Only web pages, opened in the browser: nothing else is a link
            a.buttons.retain(|b| b.url.starts_with("https://") && !b.label.trim().is_empty());
            a.buttons.truncate(MAX_BUTTONS);
            a
        })
        .collect();
    let mut seen = std::collections::HashSet::new();
    list.retain(|a| seen.insert(a.id.clone()));
    list.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(list)
}

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
        { "id": "sr2", "date": "2026-11-13", "level": "urgent!!", "game": "segarally2", "title": "Testers" },
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
}
