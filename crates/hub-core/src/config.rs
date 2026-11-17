//! The launcher's own settings, in one JSON file per user.

use crate::art::{self, Art};
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
    /// How the library lists the projects.
    #[serde(default, skip_serializing_if = "Sort::is_custom", deserialize_with = "Sort::read")]
    pub sort: Sort,
    /// The user's own order of the projects, by id. Projects it does not name
    /// (all of them, until the user moves one) follow in the registry's order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub order: Vec<String>,
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
    /// The name the user gave the project, shown instead of the registry's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Show the project's name as lettering instead of its logo (the user's
    /// own `logo`, or the registry's).
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub text_title: bool,
    /// The user's own pictures for the project (see `art`), shown instead of
    /// the registry's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover: Option<Art>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<Art>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<Art>,
    /// Shown in the sidebar and given to its desktop shortcut (as the .ico
    /// `art::ico_name` names) instead of the registry's.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<Art>,
    /// Kept out of the library, the sidebar and Home, by the user's choice.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub hidden: bool,
    /// When the launcher last started the game (Unix seconds).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_played: Option<u64>,
}

/// How the library lists the projects.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Sort {
    /// The user's own order (`Config::order`).
    #[default]
    Custom,
    /// By the name shown, A to Z.
    Name,
    /// The one played last first; those never played follow in the user's order.
    Recent,
}

impl Sort {
    fn is_custom(&self) -> bool {
        *self == Sort::Custom
    }

    /// A word for it, as the window sends it; an unknown one is None.
    pub fn parse(word: &str) -> Option<Sort> {
        serde_json::from_value(serde_json::Value::String(word.to_string())).ok()
    }

    /// One a newer launcher wrote that this one does not know is the user's
    /// own order, not a damaged settings file.
    fn read<'de, D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Sort, D::Error> {
        let word = Option::<String>::deserialize(d)?;
        Ok(word.as_deref().and_then(Sort::parse).unwrap_or_default())
    }
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

    /// The name the user gave a project, if any.
    pub fn name(&self, id: &str) -> Option<&str> {
        self.projects.get(id).and_then(|p| p.name.as_deref())
    }

    /// Whether the user chose to see a project's name instead of its logo.
    pub fn text_title(&self, id: &str) -> bool {
        self.projects.get(id).is_some_and(|p| p.text_title)
    }

    /// The user's own picture of that kind for a project, if any.
    pub fn art(&self, id: &str, kind: art::Kind) -> Option<&Art> {
        let p = self.projects.get(id)?;
        match kind {
            art::Kind::Cover => p.cover.as_ref(),
            art::Kind::Background => p.background.as_ref(),
            art::Kind::Logo => p.logo.as_ref(),
            art::Kind::Icon => p.icon.as_ref(),
        }
    }

    /// Whether the user hid a project.
    pub fn hidden(&self, id: &str) -> bool {
        self.projects.get(id).is_some_and(|p| p.hidden)
    }

    /// When the launcher last started a project.
    pub fn last_played(&self, id: &str) -> Option<u64> {
        self.projects.get(id).and_then(|p| p.last_played)
    }

    /// The order the library lists projects in. `projects` are each one's id,
    /// the name shown and when it was last played, in the registry's order;
    /// the answer is their places in it.
    pub fn arrange(&self, projects: &[(&str, &str, Option<u64>)]) -> Vec<usize> {
        let mine = |id: &str| self.order.iter().position(|o| o == id).unwrap_or(usize::MAX);
        let mut out: Vec<usize> = (0..projects.len()).collect();
        // Stable sorts: what ties keeps the order before it
        out.sort_by_key(|&i| mine(projects[i].0));
        match self.sort {
            Sort::Custom => {}
            Sort::Name => out.sort_by_cached_key(|&i| projects[i].1.to_lowercase()),
            Sort::Recent => out.sort_by_key(|&i| std::cmp::Reverse(projects[i].2)),
        }
        out
    }

    /// Every picture file the settings name, and the .ico of each icon: what
    /// `art::prune` keeps.
    pub fn art_files(&self) -> Vec<String> {
        let pictures = self.projects.values().flat_map(|p| [&p.cover, &p.background, &p.logo, &p.icon]).flatten().map(|a| a.file.clone());
        let icos = self.projects.values().filter_map(|p| p.icon.as_ref()).map(art::ico_name);
        pictures.chain(icos).collect()
    }

    /// The folder the user's pictures are kept in, next to the settings.
    pub fn art_dir(&self) -> PathBuf {
        art::dir(&self.base)
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

/// The longest name a user can give a project.
pub const NAME_MAX: usize = 40;

/// A name the user typed, as it is kept: trimmed, one line, at most
/// `NAME_MAX` characters. None for an empty one or the project's own name,
/// which both mean "the registry's name".
pub fn clean_name(typed: &str, default: &str) -> Option<String> {
    let one_line: String = typed.chars().map(|c| if c.is_control() { ' ' } else { c }).collect();
    let words: Vec<&str> = one_line.split_whitespace().collect();
    let name: String = words.join(" ").chars().take(NAME_MAX).collect();
    let name = name.trim_end().to_string();
    (!name.is_empty() && name != default).then_some(name)
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

    // Off is the default and is not written, so settings files stay as they were
    #[test]
    fn text_title_is_kept_only_when_on() {
        let mut c = Config::default();
        c.projects.entry("a".into()).or_default().name = Some("Mine".into());
        assert!(!serde_json::to_string(&c).unwrap().contains("text_title"));
        assert!(!c.text_title("a") && !c.text_title("missing"));
        c.projects.get_mut("a").unwrap().text_title = true;
        let back: Config = serde_json::from_str(&serde_json::to_string(&c).unwrap()).unwrap();
        assert!(back.text_title("a"));
    }

    #[test]
    fn round_trips_and_resolves_folders() {
        let dir = std::env::temp_dir().join(format!("nl-config-{}", std::process::id()));
        let path = dir.join("config.json");
        assert_eq!(Config::load(&path).unwrap(), Config { base: dir.clone(), ..Default::default() });

        // Full paths on any system: "/games" has no drive letter on Windows
        let full = |p: &str| std::env::temp_dir().join(p);
        let mut c = Config { install_root: Some(full("games")), ..Default::default() };
        c.projects.entry("b".into()).or_default().dir = Some(full("elsewhere/b"));
        c.projects.entry("a".into()).or_default().input = Some(full("roms/a.zip"));
        c.save(&path).unwrap();
        let back = Config::load(&path).unwrap();
        assert_eq!(back.base, dir);
        assert_eq!(back, Config { base: dir.clone(), ..c.clone() });
        assert_eq!(back.project_dir("a").unwrap(), full("games").join("a"));
        assert_eq!(back.project_dir("b").unwrap(), full("elsewhere/b"));
        assert_eq!(back.input("a").unwrap(), full("roms/a.zip"));

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
    fn names_the_user_gives() {
        assert_eq!(clean_name("  My   Rally\n2 ", "Nico SR2").as_deref(), Some("My Rally 2"));
        assert_eq!(clean_name("", "Nico SR2"), None);
        assert_eq!(clean_name("   ", "Nico SR2"), None);
        assert_eq!(clean_name(" Nico SR2 ", "Nico SR2"), None, "the default name is no custom name");
        let long = clean_name(&"é".repeat(100), "x").unwrap();
        assert_eq!(long.chars().count(), NAME_MAX);

        let text = r#"{ "projects": { "segarally2": { "name": "Rally", "input": "/roms/r.zip" }, "lostworld": {} } }"#;
        let c: Config = serde_json::from_str(text).unwrap();
        assert_eq!(c.name("segarally2"), Some("Rally"));
        assert_eq!(c.name("lostworld"), None);
        assert_eq!(c.name("other"), None);
        // No name, no key: the file stays as older launchers wrote it
        assert!(!serde_json::to_string(&c.projects["lostworld"]).unwrap().contains("name"));
    }

    #[test]
    fn pictures_the_user_picks() {
        let text = r#"{ "projects": { "segarally2": { "cover": { "file": "a.jpg", "focus": [20, 70] }, "background": { "file": "b.jpg" }, "icon": { "file": "i.png", "focus": [10, 50] } }, "lostworld": { "cover": { "file": "a.jpg" } } } }"#;
        let c: Config = serde_json::from_str(text).unwrap();
        assert_eq!(c.art("segarally2", art::Kind::Cover).unwrap().focus, [20, 70]);
        assert_eq!(c.art("segarally2", art::Kind::Background).unwrap().focus, [50, 50]);
        assert!(c.art("lostworld", art::Kind::Background).is_none());
        assert!(c.art("other", art::Kind::Cover).is_none());
        let mut files = c.art_files();
        files.sort();
        assert_eq!(files, ["a.jpg", "a.jpg", "b.jpg", "i-10-50.ico", "i.png"]);
        assert_eq!(c.art("segarally2", art::Kind::Icon).unwrap().focus, [10, 50]);
        let c = Config { base: PathBuf::from("/portable/NicoLauncher"), ..c };
        assert_eq!(c.art_dir(), PathBuf::from("/portable/NicoLauncher/art"));
        // None, no key
        let none: Config = serde_json::from_str(r#"{ "projects": { "x": {} } }"#).unwrap();
        let out = serde_json::to_string(&none).unwrap();
        assert!(!out.contains("cover") && !out.contains("background") && !out.contains("icon"), "{out}");
    }

    #[test]
    fn the_order_of_the_library() {
        let projects = [("swt", "Nico SWT", Some(50)), ("tlw", "nico TLW", None), ("sr2", "Arcade Rally", Some(90)), ("new", "Zed", None)];
        let ids = |c: &Config| c.arrange(&projects).into_iter().map(|i| projects[i].0).collect::<Vec<_>>();
        let mut c = Config::default();
        assert_eq!(ids(&c), ["swt", "tlw", "sr2", "new"], "the registry's until the user moves one");
        // One the user's order does not name (new in the registry) goes last
        c.order = vec!["sr2".into(), "swt".into(), "gone".into(), "tlw".into()];
        assert_eq!(ids(&c), ["sr2", "swt", "tlw", "new"]);
        c.sort = Sort::Name;
        assert_eq!(ids(&c), ["sr2", "swt", "tlw", "new"], "A to Z whatever the case");
        c.sort = Sort::Recent;
        assert_eq!(ids(&c), ["sr2", "swt", "tlw", "new"]);
        c.order = vec!["new".into(), "tlw".into()];
        assert_eq!(ids(&c), ["sr2", "swt", "new", "tlw"], "never played: in the user's order");
    }

    #[test]
    fn library_settings_in_the_file() {
        let c = Config::default();
        let out = serde_json::to_string(&c).unwrap();
        assert!(!out.contains("sort") && !out.contains("order"), "the defaults are not written: {out}");
        let mut c: Config = serde_json::from_str(r#"{ "sort": "recent", "order": ["b", "a"], "projects": { "a": { "hidden": true, "last_played": 7 } } }"#).unwrap();
        assert_eq!(c.sort, Sort::Recent);
        assert!(c.hidden("a") && !c.hidden("b") && !c.hidden("x"));
        assert_eq!(c.last_played("a"), Some(7));
        assert_eq!(c.last_played("b"), None);
        // Written back as it was read
        c.base = PathBuf::new();
        let back: Config = serde_json::from_str(&serde_json::to_string(&c).unwrap()).unwrap();
        assert_eq!(back, c);
        // A newer launcher's way of sorting is not a damaged file
        let c: Config = serde_json::from_str(r#"{ "sort": "by_genre" }"#).unwrap();
        assert_eq!(c.sort, Sort::Custom);
        assert_eq!(Sort::parse("name"), Some(Sort::Name));
        assert_eq!(Sort::parse("Name"), None);
        // Off is not written
        let mut c = Config::default();
        c.projects.entry("a".into()).or_default().name = Some("x".into());
        let out = serde_json::to_string(&c).unwrap();
        assert!(!out.contains("hidden") && !out.contains("last_played"), "{out}");
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
