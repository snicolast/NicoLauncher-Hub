//! The projects' `news.txt`: numbered entries, newest first.
//!
//! ```text
//! # comment
//! [10] 2026-11-09
//! - a line of the entry
//! ```

use crate::{err, http_get, registry::Project, Result};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Entry {
    pub number: u32,
    pub title: String,
    pub lines: Vec<String>,
}

pub fn parse(text: &str) -> Vec<Entry> {
    let mut entries: Vec<Entry> = Vec::new();
    for raw in text.lines() {
        let line = raw.trim_end();
        if line.trim_start().starts_with('#') || line.trim().is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('[') {
            if let Some((num, title)) = rest.split_once(']') {
                if let Ok(number) = num.trim().parse() {
                    entries.push(Entry { number, title: title.trim().to_string(), lines: Vec::new() });
                    continue;
                }
            }
        }
        if let Some(entry) = entries.last_mut() {
            let text = line.trim_start();
            entry.lines.push(text.strip_prefix("- ").unwrap_or(text).to_string());
        }
    }
    entries.sort_by(|a, b| b.number.cmp(&a.number));
    entries
}

/// The highest entry number, 0 for a file with none.
pub fn latest(entries: &[Entry]) -> u32 {
    entries.iter().map(|e| e.number).max().unwrap_or(0)
}

/// The entries of `remote` that `local` does not have yet.
pub fn newer_than(remote: &[Entry], local: &[Entry]) -> Vec<Entry> {
    let have = latest(local);
    remote.iter().filter(|e| e.number > have).cloned().collect()
}

/// The launcher's own news, for its home page: announcements that are not
/// about one project. Same format, in the launcher's public repository (NicoLauncher-Hub).
pub const LAUNCHER_NEWS_URL: &str = "https://raw.githubusercontent.com/snicolast/NicoLauncher-Hub/main/news.txt";

pub fn fetch_launcher() -> Result<Vec<Entry>> {
    Ok(parse(&http_get(LAUNCHER_NEWS_URL)?.into_string().map_err(err("Bad news file"))?))
}

/// The project's news as it is on GitHub now.
pub fn fetch(project: &Project) -> Result<Vec<Entry>> {
    let Some(file) = &project.news else { return Ok(Vec::new()) };
    let url = project.raw_url(file);
    Ok(parse(&http_get(&url)?.into_string().map_err(err("Bad news file"))?))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "# what is new\n#\n\n[10] 2026-11-09\n- New explosions\n- Flight sticks\n\n[9] 2026-11-08\n- Mac build\n";

    #[test]
    fn parses_entries_newest_first() {
        let e = parse(SAMPLE);
        assert_eq!(e.len(), 2);
        assert_eq!(e[0].number, 10);
        assert_eq!(e[0].title, "2026-11-09");
        assert_eq!(e[0].lines, ["New explosions", "Flight sticks"]);
        assert_eq!(latest(&e), 10);
    }

    #[test]
    fn finds_what_is_new() {
        let remote = parse(SAMPLE);
        let local = parse("[9] 2026-11-08\n- Mac build\n");
        let new = newer_than(&remote, &local);
        assert_eq!(new.len(), 1);
        assert_eq!(new[0].number, 10);
        assert!(newer_than(&remote, &remote).is_empty());
        assert_eq!(newer_than(&remote, &[]).len(), 2);
    }

    #[test]
    fn ignores_text_before_the_first_entry_and_crlf() {
        let e = parse("stray line\r\n[1] first\r\n- one\r\n");
        assert_eq!(e.len(), 1);
        assert_eq!(e[0].lines, ["one"]);
    }
}
