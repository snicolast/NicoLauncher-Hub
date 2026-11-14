//! The launcher's own settings, in one JSON file per user.

use crate::{err, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Paths in the file may be relative: they start from the folder that holds
/// it (`.\Games`, `..\ROMs\game.zip`). Read them through `install_root()`,
/// `project_dir()` and `input()`, which resolve them, and store new ones with
/// `store()`.
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
    /// The launcher version whose news the user has seen: a newer one running
    /// shows what changed since, once.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_version: Option<String>,
    /// The folder relative paths start from: the one the file is in.
    #[serde(skip)]
    pub base: PathBuf,
    /// A portable launcher: folders and files picked inside `base` are stored
    /// relative to it, so they still work when the folder is moved.
    #[serde(skip)]
    pub portable: bool,
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

    /// A missing or empty file is an empty config; a damaged one is an error,
    /// not a reset.
    pub fn load(path: &Path) -> Result<Config> {
        let mut config: Config = match std::fs::read_to_string(path) {
            Ok(text) if text.trim().is_empty() => Config::default(),
            Ok(text) => serde_json::from_str(&text).map_err(err("Bad settings file"))?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Config::default(),
            Err(e) => return Err(format!("Could not read {}: {e}", path.display())),
        };
        config.base = path.parent().map(Path::to_path_buf).unwrap_or_default();
        Ok(config)
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

    /// Where projects are installed.
    pub fn install_root(&self) -> Option<PathBuf> {
        self.install_root.as_deref().map(|p| self.resolve(p))
    }

    /// Where a project lives, or would be installed.
    pub fn project_dir(&self, id: &str) -> Option<PathBuf> {
        if let Some(dir) = self.projects.get(id).and_then(|p| p.dir.as_deref()) {
            return Some(self.resolve(dir));
        }
        self.install_root().map(|root| root.join(id))
    }

    pub fn input(&self, id: &str) -> Option<PathBuf> {
        self.projects.get(id).and_then(|p| p.input.as_deref()).map(|p| self.resolve(p))
    }

    /// A path from the file as a full one.
    pub fn resolve(&self, path: &Path) -> PathBuf {
        if path.is_absolute() {
            return path.to_path_buf();
        }
        normalize(&self.base.join(path))
    }

    /// A full path the user picked, as it is written to the file: relative
    /// when the launcher is portable and it is inside the launcher's folder.
    pub fn store(&self, path: PathBuf) -> PathBuf {
        if !self.portable {
            return path;
        }
        match normalize(&path).strip_prefix(normalize(&self.base)) {
            Ok(rest) if rest.as_os_str().is_empty() => PathBuf::from("."),
            Ok(rest) => Path::new(".").join(rest),
            Err(_) => path,
        }
    }
}

/// `.` and `..` taken out of a path, without asking the disk.
fn normalize(path: &Path) -> PathBuf {
    use std::path::Component;
    let mut out = PathBuf::new();
    for c in path.components() {
        match c {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    out.push("..");
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
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
        assert_eq!(Config::load(&path).unwrap(), Config { base: dir.clone(), ..Default::default() });

        let mut c = Config { install_root: Some(PathBuf::from("/games")), ..Default::default() };
        c.projects.entry("b".into()).or_default().dir = Some(PathBuf::from("/elsewhere/b"));
        c.projects.entry("a".into()).or_default().input = Some(PathBuf::from("/roms/a.zip"));
        c.save(&path).unwrap();
        let back = Config::load(&path).unwrap();
        assert_eq!(back.base, dir);
        assert_eq!(back, Config { base: dir.clone(), ..c.clone() });
        assert_eq!(back.project_dir("a").unwrap(), PathBuf::from("/games/a"));
        assert_eq!(back.project_dir("b").unwrap(), PathBuf::from("/elsewhere/b"));
        assert_eq!(back.input("a").unwrap(), PathBuf::from("/roms/a.zip"));

        std::fs::write(&path, "{ not json").unwrap();
        assert!(Config::load(&path).is_err());
        // An empty file, made by hand to go portable, is an empty config
        std::fs::write(&path, "\n").unwrap();
        assert_eq!(Config::load(&path).unwrap().projects, BTreeMap::new());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn relative_paths_start_from_the_settings_folder() {
        let base = PathBuf::from("/portable/NicoLauncher");
        let text = r#"{
            "install_root": "./Games",
            "projects": {
                "lostworld": { "input": "./_ROMs/lostwsga.zip" },
                "segarally2": { "dir": "../Elsewhere/SR2", "input": "/roms/srally2.zip" }
            }
        }"#;
        let mut c: Config = serde_json::from_str(text).unwrap();
        c.base = base.clone();
        assert_eq!(c.install_root().unwrap(), base.join("Games"));
        assert_eq!(c.project_dir("lostworld").unwrap(), base.join("Games").join("lostworld"));
        assert_eq!(c.input("lostworld").unwrap(), base.join("_ROMs").join("lostwsga.zip"));
        assert_eq!(c.project_dir("segarally2").unwrap(), PathBuf::from("/portable/Elsewhere/SR2"));
        assert_eq!(c.input("segarally2").unwrap(), PathBuf::from("/roms/srally2.zip"));
        // The file keeps them as they were written
        assert!(serde_json::to_string(&c).unwrap().contains("./Games"));
    }

    #[test]
    fn a_portable_launcher_stores_its_own_folder_relative() {
        let mut c = Config { base: PathBuf::from("/portable/NicoLauncher"), ..Default::default() };
        let inside = PathBuf::from("/portable/NicoLauncher/Games");
        let outside = PathBuf::from("/roms/srally2.zip");
        assert_eq!(c.store(inside.clone()), inside, "installed: always full paths");
        c.portable = true;
        assert_eq!(c.store(inside.clone()), Path::new(".").join("Games"));
        assert_eq!(c.store(PathBuf::from("/portable/NicoLauncher")), PathBuf::from("."));
        assert_eq!(c.store(outside.clone()), outside);
        assert_eq!(c.store(PathBuf::from("/portable/NicoLauncherOld/x")), PathBuf::from("/portable/NicoLauncherOld/x"));
        // And reads them back as they were picked
        c.install_root = Some(c.store(inside.clone()));
        assert_eq!(c.install_root().unwrap(), inside);
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
