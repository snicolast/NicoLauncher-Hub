//! A project's achievements, as the project itself writes them in its folder.
//!
//! The launcher knows nothing of any project's achievements: a project that
//! has them writes `save/achievements.json` (see `docs/manifest.md`) with the
//! whole list, how far the player is on each one and when each was unlocked,
//! and the launcher only shows that file. A missing or damaged file is the
//! same as no achievements.

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Where a project leaves the file, inside its folder.
pub const FILE: &str = "save/achievements.json";

/// The format this launcher reads. A newer one is not shown.
const SCHEMA: u32 = 1;

/// Not to be fooled by a huge file: a project has a few dozen.
const MAX_ACHIEVEMENTS: usize = 1000;
const MAX_FILE_BYTES: u64 = 1 << 20;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    /// Progress needed; 1 for those that are done or not.
    #[serde(default = "one")]
    pub goal: i64,
    #[serde(default)]
    pub progress: i64,
    /// How the project wants the progress written ("2h 13m / 5h"), when the
    /// plain numbers would not read well.
    #[serde(default)]
    pub progress_text: Option<String>,
    /// Unix time it was unlocked; 0 still locked.
    #[serde(default)]
    pub unlocked: i64,
}

fn one() -> i64 {
    1
}

#[derive(Deserialize)]
struct File {
    schema: u32,
    #[serde(default)]
    achievements: Vec<Achievement>,
}

/// Parses the file's text. Unknown keys are ignored, so a project can add
/// more without breaking older launchers.
pub fn parse(text: &str) -> Vec<Achievement> {
    let Ok(file) = serde_json::from_str::<File>(text) else {
        return Vec::new();
    };
    if file.schema != SCHEMA {
        return Vec::new();
    }
    file.achievements
        .into_iter()
        .take(MAX_ACHIEVEMENTS)
        .filter(|a| !a.id.is_empty() && !a.name.is_empty())
        .map(|mut a| {
            a.goal = a.goal.max(1);
            a.unlocked = a.unlocked.max(0);
            a.progress = if a.unlocked > 0 {
                a.goal
            } else {
                a.progress.clamp(0, a.goal)
            };
            a
        })
        .collect()
}

/// The achievements of the project installed in `dir`, in the project's own
/// order. Empty when it has none or has not written the file yet.
pub fn read(dir: &Path) -> Vec<Achievement> {
    let path = dir.join(FILE);
    match std::fs::metadata(&path) {
        Ok(m) if m.len() <= MAX_FILE_BYTES => {}
        _ => return Vec::new(),
    }
    std::fs::read_to_string(&path)
        .map(|t| parse(&t))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_what_a_game_writes() {
        let list = parse(
            r#"{
              "schema": 1,
              "achievements": [
                {"id": "ace", "name": "ACE PILOT", "description": "Destroy 100 enemies", "goal": 100, "progress": 37, "unlocked": 0},
                {"id": "veteran", "name": "VETERAN", "description": "Play for 5 hours", "goal": 1035432, "progress": 2000,
                 "progress_text": "0h 00m / 5h", "unlocked": 0, "something_new": true},
                {"id": "yavin", "name": "GREAT SHOT, KID", "goal": 1, "progress": 0, "unlocked": 1762900000}
              ]
            }"#,
        );
        assert_eq!(list.len(), 3);
        assert_eq!(list[0].progress, 37);
        assert_eq!(list[1].progress_text.as_deref(), Some("0h 00m / 5h"));
        // Unlocked is always complete
        assert_eq!((list[2].progress, list[2].unlocked), (1, 1762900000));
        assert_eq!(list[2].description, "");
    }

    #[test]
    fn bad_or_newer_files_show_nothing() {
        assert!(parse("").is_empty());
        assert!(parse("not json").is_empty());
        assert!(parse(r#"{"schema": 2, "achievements": [{"id": "a", "name": "A"}]}"#).is_empty());
        // Out of range numbers are brought back in, nameless entries dropped
        let list = parse(
            r#"{"schema": 1, "achievements": [{"id": "a", "name": "A", "goal": 0, "progress": 9}, {"id": "", "name": "B"}]}"#,
        );
        assert_eq!(list.len(), 1);
        assert_eq!((list[0].goal, list[0].progress), (1, 1));
    }

    #[test]
    fn reads_from_the_project_folder() {
        let dir = std::env::temp_dir().join(format!("nl-ach-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        assert!(read(&dir).is_empty());
        std::fs::create_dir_all(dir.join("save")).unwrap();
        std::fs::write(
            dir.join(FILE),
            r#"{"schema": 1, "achievements": [{"id": "a", "name": "A"}]}"#,
        )
        .unwrap();
        assert_eq!(read(&dir).len(), 1);
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
