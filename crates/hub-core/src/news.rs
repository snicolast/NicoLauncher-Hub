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

/// The launcher version an entry of its own news is about: the entries of a
/// release start "Launcher 0.3.6: ...". Others (a new project, a notice) are
/// about no version.
pub fn release_of(entry: &Entry) -> Option<&str> {
    let rest = entry.lines.first()?.strip_prefix("Launcher ")?;
    let (version, _) = rest.split_once(':')?;
    parse_version(version).map(|_| version)
}

fn parse_version(v: &str) -> Option<Vec<u32>> {
    let parts: Option<Vec<u32>> = v.split('.').map(|n| n.parse().ok()).collect();
    parts.filter(|p| p.len() == 3)
}

/// Whether version `a` is newer than `b`, compared as numbers (0.3.10 is
/// newer than 0.3.9). False when either is not a version.
pub fn is_newer(a: &str, b: &str) -> bool {
    matches!((parse_version(a), parse_version(b)), (Some(a), Some(b)) if a > b)
}

/// One launcher release in its news: the version and what it brought.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Release {
    pub version: String,
    pub title: String,
    pub lines: Vec<String>,
}

/// What the launcher brought after `from` up to `to` (the running version),
/// newest first, with "Launcher 0.3.6: " taken off. With `from` unknown (a
/// launcher too old to have recorded it), only what `to` brought.
pub fn launcher_changes(entries: &[Entry], from: Option<&str>, to: &str) -> Vec<Release> {
    let (Some(to_v), from_v) = (parse_version(to), from.and_then(parse_version)) else { return Vec::new() };
    entries
        .iter()
        .filter(|e| {
            release_of(e).and_then(parse_version).is_some_and(|v| {
                v <= to_v && from_v.as_ref().map_or(v == to_v, |f| v > *f)
            })
        })
        .map(|e| {
            let mut lines = e.lines.clone();
            let rest = lines[0].split_once(':').map_or("", |(_, r)| r.trim_start()).to_string();
            if rest.is_empty() {
                lines.remove(0);
            } else {
                lines[0] = rest;
            }
            Release { version: release_of(e).unwrap_or_default().to_string(), title: e.title.clone(), lines }
        })
        .collect()
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

    const LAUNCHER: &str = "[6] 2026-11-15\n- Launcher 0.3.6: popups\n- more\n\n[5] 2026-11-14\n- A new project\n\n[4] 2026-11-14\n- Launcher 0.3.5: folders\n\n[3] 2026-11-13\n- Launcher 0.3.4: feedback\n\n[2] 2026-11-12\n- Launcher 0.3.10: later\n";

    #[test]
    fn picks_what_a_launcher_update_brought() {
        let e = parse(LAUNCHER);
        let versions = |v: Vec<Release>| v.into_iter().map(|r| r.version).collect::<Vec<_>>();
        assert_eq!(versions(launcher_changes(&e, Some("0.3.4"), "0.3.6")), vec!["0.3.6", "0.3.5"]);
        assert_eq!(versions(launcher_changes(&e, Some("0.3.5"), "0.3.6")), vec!["0.3.6"]);
        assert!(launcher_changes(&e, Some("0.3.6"), "0.3.6").is_empty());
        assert_eq!(versions(launcher_changes(&e, None, "0.3.6")), vec!["0.3.6"]);
        // 0.3.10 is newer than 0.3.6, compared as numbers
        assert_eq!(versions(launcher_changes(&e, Some("0.3.6"), "0.3.10")), vec!["0.3.10"]);
        let first = &launcher_changes(&e, Some("0.3.5"), "0.3.6")[0];
        assert_eq!((first.title.as_str(), first.lines.clone()), ("2026-11-15", vec!["popups".to_string(), "more".to_string()]));
        assert_eq!(release_of(&e[1]), None);
    }

    #[test]
    fn compares_versions_as_numbers() {
        assert!(is_newer("0.3.10", "0.3.9"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("0.3.9", "0.3.9"));
        assert!(!is_newer("0.3.8", "0.3.9"));
        assert!(!is_newer("soon", "0.3.9"));
    }

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
