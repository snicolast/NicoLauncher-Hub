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
}
