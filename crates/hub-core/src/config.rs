//! The launcher's own settings, in one JSON file per user.

use crate::{err, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Config {
    /// Where projects are installed, one folder each. Unset until the user picks it.
    #[serde(default)]
    pub install_root: Option<PathBuf>,
    #[serde(default)]
    pub language: Option<String>,
    /// The version of the license agreement the user accepted on first run.
    /// Unset, or older than the current one, asks again.
    #[serde(default)]
    pub license_accepted: Option<u32>,
    #[serde(default)]
    pub projects: BTreeMap<String, ProjectConfig>,
    /// Ids of the announcements the user has closed: they no longer make the
    /// megaphone beat.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub seen_announcements: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ProjectConfig {
    /// A folder of its own, for a copy the user already had. Otherwise
    /// `install_root/<id>`.
    #[serde(default)]
    pub dir: Option<PathBuf>,
    /// The user's own file (ROM) for this project: a path on their disk.
    #[serde(default)]
    pub input: Option<PathBuf>,
}

impl Config {
    /// `%APPDATA%\NicoLauncher\config.json` (`~/.config/...` elsewhere).
    pub fn default_path() -> PathBuf {
        let base = std::env::var_os("APPDATA")
            .or_else(|| std::env::var_os("XDG_CONFIG_HOME"))
            .map(PathBuf::from)
            .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))
            .unwrap_or_else(|| PathBuf::from("."));
        base.join("NicoLauncher").join("config.json")
    }

    /// A missing file is an empty config; a damaged one is an error, not a reset.
    pub fn load(path: &Path) -> Result<Config> {
        match std::fs::read_to_string(path) {
            Ok(text) => serde_json::from_str(&text).map_err(err("Bad settings file")),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
            Err(e) => Err(format!("Could not read {}: {e}", path.display())),
        }
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(err("Could not create the settings folder"))?;
        }
        let text = serde_json::to_string_pretty(self).map_err(err("Could not write settings"))?;
        // Written whole next to it, then swapped in: never a half-written file
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, text).map_err(err("Could not write settings"))?;
        std::fs::rename(&tmp, path).map_err(err("Could not write settings"))
    }

    /// Where a project lives, or would be installed.
    pub fn project_dir(&self, id: &str) -> Option<PathBuf> {
        if let Some(dir) = self.projects.get(id).and_then(|p| p.dir.clone()) {
            return Some(dir);
        }
        self.install_root.as_ref().map(|root| root.join(id))
    }

    pub fn input(&self, id: &str) -> Option<PathBuf> {
        self.projects.get(id).and_then(|p| p.input.clone())
    }
}

/// Characters a project's folder cannot have: CMake reads `[` `]` as wildcards
/// when it lists files (SDL3 is then unpacked but never found, issue #20) and
/// `;` as a list separator. Spaces and other characters are fine.
pub const BAD_FOLDER_CHARS: [char; 3] = ['[', ']', ';'];

/// Why projects cannot be built in this folder, if they cannot.
pub fn folder_problem(path: &Path) -> Option<String> {
    let text = path.to_string_lossy();
    let found: Vec<String> = BAD_FOLDER_CHARS.iter().filter(|c| text.contains(**c)).map(|c| c.to_string()).collect();
    if found.is_empty() {
        return None;
    }
    Some(format!(
        "{} has {} in its name, and the build tools cannot work in a folder like that. Choose a folder without [ ] or ; in its name.",
        path.display(),
        found.join(" ")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_and_resolves_folders() {
        let dir = std::env::temp_dir().join(format!("nl-config-{}", std::process::id()));
        let path = dir.join("config.json");
        assert_eq!(Config::load(&path).unwrap(), Config::default());

        let mut c = Config { install_root: Some(PathBuf::from("/games")), ..Default::default() };
        c.projects.entry("b".into()).or_default().dir = Some(PathBuf::from("/elsewhere/b"));
        c.projects.entry("a".into()).or_default().input = Some(PathBuf::from("/roms/a.zip"));
        c.save(&path).unwrap();
        let back = Config::load(&path).unwrap();
        assert_eq!(back, c);
        assert_eq!(back.project_dir("a").unwrap(), PathBuf::from("/games/a"));
        assert_eq!(back.project_dir("b").unwrap(), PathBuf::from("/elsewhere/b"));
        assert_eq!(back.input("a").unwrap(), PathBuf::from("/roms/a.zip"));

        std::fs::write(&path, "{ not json").unwrap();
        assert!(Config::load(&path).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn folders_the_build_tools_cannot_use() {
        assert!(folder_problem(Path::new(r"D:\Games\Arcade - Sega Rally 2\NicoLauncher")).is_none());
        assert!(folder_problem(Path::new(r"C:\Juegos\Señor")).is_none());
        let problem = folder_problem(Path::new(r"D:\[Recompilados]\Racing\[Arcade] - Sega Rally 2")).unwrap();
        assert!(problem.starts_with(r"D:\[Recompilados]") && problem.contains("has [ ] in its name"), "{problem}");
        assert!(folder_problem(Path::new("/games/a;b")).is_some());
    }
}
