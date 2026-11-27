//! The list of projects the launcher offers. A copy is built into the exe and a
//! newer one is read from the launcher's repository, so adding a project needs
//! no new release of the launcher.

use crate::requirements::Requires;
use crate::{err, http_get, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Where GitHub serves a file of the Hub repository, next to registry.json.
pub fn hub_raw_url(rel: &str) -> String {
    format!("{}/{rel}", REGISTRY_URL.rsplit_once('/').map_or(REGISTRY_URL, |(dir, _)| dir))
}

pub const REGISTRY_URL: &str =
    "https://raw.githubusercontent.com/snicolast/NicoLauncher-Hub/main/registry.json";
const BUNDLED: &str = include_str!("../../../registry.json");
/// The registry format this build understands.
pub const SCHEMA: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Registry {
    pub schema: u32,
    pub projects: Vec<Project>,
    /// Projects on the way: shown greyed out, with nothing to install or play.
    /// Launchers older than 0.3.16 skip the whole list.
    #[serde(default)]
    pub coming_soon: Vec<Teaser>,
    /// The author's private mailbox (https), where the Feedback page sends
    /// ideas, recomp proposals and messages (docs/feedback.md). Without it,
    /// those cards are greyed out. Launchers older than 0.3.21 skip it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feedback_inbox: Option<String>,
}

impl Registry {
    /// The mailbox's address, when it is a web address the launcher may use.
    pub fn inbox(&self) -> Option<&str> {
        self.feedback_inbox.as_deref().map(str::trim).filter(|u| u.starts_with("https://") && u.len() > 8)
    }
}

/// A project that is announced but not out: its name, a few words and its
/// pictures, all kept in the Hub (it may have no public repository yet).
/// When it comes out, it moves to `projects` under the same id.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Teaser {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub tagline: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub tags: Vec<String>,
    /// When it is expected, in a few words ("2027", "this summer"): shown as
    /// "Coming 2027". Without it, "Coming soon".
    #[serde(default)]
    pub when: Option<String>,
    #[serde(default)]
    pub art: TeaserArt,
}

/// Paths in the Hub, next to registry.json.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TeaserArt {
    /// A wide picture for its card, its page and Home.
    #[serde(default)]
    pub hero: Option<String>,
    #[serde(default)]
    pub logo: Option<String>,
    /// Square, for the sidebar.
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub screens: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    /// Stable key: used for the install folder and the saved settings.
    pub id: String,
    pub name: String,
    /// One line under the name.
    #[serde(default)]
    pub tagline: String,
    #[serde(default)]
    pub description: String,
    /// Short labels shown as chips (platform, year...).
    #[serde(default)]
    pub tags: Vec<String>,
    /// Pictures the project's own repository already publishes (README
    /// screenshots, its logo). Shown from GitHub, never bundled in the launcher.
    #[serde(default)]
    pub art: Art,
    /// GitHub `owner/name`.
    pub repo: String,
    pub branch: String,
    /// The program the project's build leaves in its folder.
    #[serde(default)]
    pub exe: Option<String>,
    /// News file in the repository (the `[N] title` format), if it has one.
    #[serde(default)]
    pub news: Option<String>,
    /// A file only the user can provide (a ROM). Never downloaded.
    #[serde(default)]
    pub input: Option<UserInput>,
    #[serde(default)]
    pub scripts: Scripts,
    /// What must be on the PC to build it.
    #[serde(default)]
    pub requires: Requires,
    /// Folder where the project's own updater keeps `version.txt` and
    /// `manifest.txt`. When set, an install writes them too.
    #[serde(default)]
    pub update_state: Option<String>,
    /// The systems it runs on, shown as small icons on its card and page.
    /// The launcher installs the Windows version only; the others say where
    /// their own guide is. Without the list: Windows only. Launchers older
    /// than 0.3.17 skip it.
    #[serde(default)]
    pub platforms: Vec<Platform>,
    /// Who else made part of it (a port to another system, say), shown on
    /// the Credits page under its author. Launchers older than 0.3.18 skip it.
    #[serde(default)]
    pub credits: Vec<Credit>,
}

/// One contributor of a project, credited by their GitHub account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Credit {
    /// Their GitHub login: the name shown and the profile linked.
    pub github: String,
    /// What they did: `port` (to `os`), or words of their own, shown as
    /// they are.
    pub role: String,
    /// The system of a port: `macos`, `linux`, `windows`.
    #[serde(default)]
    pub os: Option<String>,
}

impl Credit {
    pub fn profile_url(&self) -> String {
        format!("https://github.com/{}", self.github)
    }
}

/// One system a project runs on, or will soon.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Platform {
    /// `windows`, `macos` or `linux`. Another name is kept but not shown, so
    /// a newer list does not break this launcher.
    pub os: String,
    /// The install guide in the project's repository (`docs/MACOS.md`), for
    /// a system the launcher does not install on.
    #[serde(default)]
    pub guide: Option<String>,
    /// Announced but not ready yet: shown dimmed, as "Coming soon".
    #[serde(default)]
    pub soon: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInput {
    /// What to ask for, as the project's README names it.
    pub label: String,
}

/// Paths in the project's repository.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Art {
    /// A wide picture for the cover and the project page.
    #[serde(default)]
    pub hero: Option<String>,
    #[serde(default)]
    pub logo: Option<String>,
    /// A logo kept in this launcher's own repository (the Hub), next to
    /// registry.json, instead of in the project's: shown in place of `logo`.
    /// Launchers older than 0.3.13 skip it and show the name.
    #[serde(default)]
    pub hub_logo: Option<String>,
    /// A square icon kept in the Hub (a PNG): the sidebar's, and the one a
    /// desktop shortcut has until the user picks their own (the game's exe
    /// has it too). Launchers older than 0.3.15 skip it.
    #[serde(default)]
    pub hub_icon: Option<String>,
    #[serde(default)]
    pub screens: Vec<String>,
}

/// The project's own scripts, relative to its folder. Phase 1 of the launcher
/// runs these as they are; later a manifest in each repository replaces them.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Scripts {
    #[serde(default)]
    pub install: Option<String>,
    #[serde(default)]
    pub build: Option<String>,
    #[serde(default)]
    pub update: Option<String>,
}

impl Project {
    pub fn url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The logo to show, as a URL: the one in the Hub, else the project's own.
    pub fn logo_url(&self) -> Option<String> {
        match (&self.art.hub_logo, &self.art.logo) {
            (Some(rel), _) => Some(hub_raw_url(rel)),
            (None, Some(rel)) => Some(self.raw_url(rel)),
            (None, None) => None,
        }
    }

    /// The icon to show, as a URL: the one in the Hub.
    pub fn icon_url(&self) -> Option<String> {
        self.art.hub_icon.as_deref().map(hub_raw_url)
    }

    /// Where GitHub shows a file of the repository's branch, as a page.
    pub fn page_url(&self, rel: &str) -> String {
        format!("https://github.com/{}/blob/{}/{}", self.repo, self.branch, rel)
    }

    /// Where GitHub serves a file of the repository's branch.
    pub fn raw_url(&self, rel: &str) -> String {
        format!("https://raw.githubusercontent.com/{}/{}/{}", self.repo, self.branch, rel)
    }
}

/// A GitHub login: letters, digits and single hyphens, at most 39.
fn login_ok(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 39
        && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !s.starts_with('-')
        && !s.ends_with('-')
        && !s.contains("--")
}

impl Registry {
    pub fn parse(text: &str) -> Result<Registry> {
        let reg: Registry = serde_json::from_str(text).map_err(err("Bad registry"))?;
        if reg.schema > SCHEMA {
            return Err(format!(
                "The project list is in a newer format ({}) than this launcher reads ({SCHEMA}): update the launcher.",
                reg.schema
            ));
        }
        for p in &reg.projects {
            let repo_ok = p.repo.split('/').count() == 2
                && p.repo.chars().all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c))
                && !p.repo.contains("..");
            let branch_ok = !p.branch.is_empty()
                && p.branch.chars().all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c))
                && !p.branch.contains("..");
            if !id_ok(&p.id) || !repo_ok || !branch_ok {
                return Err(format!("Bad project entry in the registry: {:?}", p.id));
            }
            p.requires.validate().map_err(|e| format!("{e} ({})", p.id))?;
            for c in &p.credits {
                if !login_ok(&c.github) || c.role.trim().is_empty() {
                    return Err(format!("Bad credit in the registry ({}): {:?}", p.id, c.github));
                }
            }
            let s = &p.scripts;
            let files = [&p.exe, &p.news, &s.install, &s.build, &s.update, &p.update_state, &p.art.hero, &p.art.logo, &p.art.hub_logo, &p.art.hub_icon];
            let guides = p.platforms.iter().filter_map(|x| x.guide.as_ref());
            for rel in files.into_iter().flatten().chain(&p.art.screens).chain(guides) {
                if !is_plain_relative(rel) {
                    return Err(format!("Bad path in the registry ({}): {rel}", p.id));
                }
            }
        }
        for t in &reg.coming_soon {
            if !id_ok(&t.id) {
                return Err(format!("Bad project entry in the registry: {:?}", t.id));
            }
            let a = &t.art;
            for rel in [&a.hero, &a.logo, &a.icon].into_iter().flatten().chain(&a.screens) {
                if !is_plain_relative(rel) {
                    return Err(format!("Bad path in the registry ({}): {rel}", t.id));
                }
            }
        }
        Ok(reg)
    }

    /// The list built into this exe.
    pub fn bundled() -> Registry {
        Registry::parse(BUNDLED).expect("the bundled registry.json is valid")
    }

    /// The current list from GitHub.
    pub fn fetch() -> Result<Registry> {
        let text = http_get(REGISTRY_URL)?.into_string().map_err(err("Bad registry"))?;
        Registry::parse(&text)
    }

    /// Adds the projects of a local list (registry.local.json next to the
    /// config): one with the id of a listed project takes its place, the
    /// others go at the end. For trying a project on one PC before it is in
    /// the public list. A missing or broken file changes nothing.
    pub fn with_local(mut self, path: &Path) -> Registry {
        let Ok(text) = std::fs::read_to_string(path) else { return self };
        match local_projects(&text) {
            Ok(projects) => {
                for p in projects {
                    match self.projects.iter_mut().find(|q| q.id == p.id) {
                        Some(q) => *q = p,
                        None => self.projects.push(p),
                    }
                }
            }
            Err(e) => eprintln!("{}: {e}", path.display()),
        }
        self
    }

    pub fn find(&self, id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }

    /// The projects still to come, without one that is out already (listed in
    /// `projects` too, as when it has just moved there).
    pub fn teasers(&self) -> impl Iterator<Item = &Teaser> {
        self.coming_soon.iter().filter(|t| self.find(&t.id).is_none())
    }
}

/// Where the local list goes: next to the config.
pub fn local_file_next_to(config: &Path) -> PathBuf {
    config.with_file_name("registry.local.json")
}

/// The projects of a local list, checked like the public one's. Only
/// `projects` is read; the rest of the file is left alone.
fn local_projects(text: &str) -> Result<Vec<Project>> {
    #[derive(Deserialize)]
    struct Local {
        #[serde(default)]
        projects: serde_json::Value,
    }
    let local: Local = serde_json::from_str(text).map_err(err("Bad local registry"))?;
    let full = serde_json::json!({ "schema": SCHEMA, "projects": local.projects });
    Ok(Registry::parse(&full.to_string())?.projects)
}

/// An id fit for a folder name and a settings key.
fn id_ok(id: &str) -> bool {
    !id.is_empty() && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

impl Teaser {
    pub fn hero_url(&self) -> Option<String> {
        self.art.hero.as_deref().map(hub_raw_url)
    }
    pub fn logo_url(&self) -> Option<String> {
        self.art.logo.as_deref().map(hub_raw_url)
    }
    pub fn icon_url(&self) -> Option<String> {
        self.art.icon.as_deref().map(hub_raw_url)
    }
    pub fn screen_urls(&self) -> Vec<String> {
        self.art.screens.iter().map(|s| hub_raw_url(s)).collect()
    }
}

/// A path that stays inside the project folder: relative, no `..`, no drive.
pub fn is_plain_relative(rel: &str) -> bool {
    !rel.is_empty()
        && !rel.contains(':')
        && !rel.starts_with(['/', '\\'])
        && rel.split(['/', '\\']).all(|part| !part.is_empty() && part != ".." && part != ".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_registry_is_valid() {
        let reg = Registry::bundled();
        assert!(reg.find("swtrilogy").is_some());
        assert_eq!(reg.find("segarally2").unwrap().exe.as_deref(), Some("SegaRally2.exe"));
        let lw = reg.find("lostworld").unwrap();
        assert_eq!(lw.branch, "master");
        assert_eq!(lw.requires.python.as_deref(), Some("3.10"));
        assert!(lw.requires.msvc);
        assert_eq!(
            lw.raw_url(lw.art.hero.as_ref().unwrap()),
            "https://raw.githubusercontent.com/snicolast/LostWorldNative/master/docs/img/hero.jpg"
        );
    }

    #[test]
    fn a_logo_in_the_hub_comes_first() {
        let entry = |art: &str| Registry::parse(&format!(r#"{{"schema":1,"projects":[{{"id":"a","name":"A","repo":"o/r","branch":"main","art":{art}}}]}}"#));
        let both = entry(r#"{"logo":"res/logo.png","hub_logo":"art/logos/a.png"}"#).unwrap();
        assert_eq!(both.projects[0].logo_url().unwrap(), "https://raw.githubusercontent.com/snicolast/NicoLauncher-Hub/main/art/logos/a.png");
        let own = entry(r#"{"logo":"res/logo.png"}"#).unwrap();
        assert_eq!(own.projects[0].logo_url().unwrap(), "https://raw.githubusercontent.com/o/r/main/res/logo.png");
        assert_eq!(entry("{}").unwrap().projects[0].logo_url(), None);
        assert!(entry(r#"{"hub_logo":"../x.png"}"#).is_err());
        let sr2 = Registry::bundled();
        assert!(sr2.find("segarally2").unwrap().logo_url().unwrap().ends_with("/NicoLauncher-Hub/main/art/logos/segarally2.png"));
        for p in &sr2.projects {
            assert_eq!(p.icon_url().unwrap(), format!("https://raw.githubusercontent.com/snicolast/NicoLauncher-Hub/main/art/icons/{}.png", p.id));
        }
        assert_eq!(entry("{}").unwrap().projects[0].icon_url(), None);
        assert!(entry(r#"{"hub_icon":"../x.png"}"#).is_err());
    }

    #[test]
    fn projects_to_come_are_listed_apart() {
        let reg = Registry::bundled();
        for t in reg.teasers() {
            assert!(reg.find(&t.id).is_none());
        }

        let text = |soon: &str| format!(r#"{{"schema":1,"projects":[{{"id":"a","name":"A","repo":"o/r","branch":"main"}}],"coming_soon":[{soon}]}}"#);
        let reg = Registry::parse(&text(r#"{"id":"b","name":"B","when":"2027","art":{"hero":"art/soon/b.jpg","screens":["art/soon/b1.jpg"]}}"#)).unwrap();
        let b = &reg.coming_soon[0];
        assert_eq!(b.hero_url().unwrap(), "https://raw.githubusercontent.com/snicolast/NicoLauncher-Hub/main/art/soon/b.jpg");
        assert_eq!(b.screen_urls().len(), 1);
        assert_eq!(b.when.as_deref(), Some("2027"));
        assert_eq!(b.icon_url(), None);
        // Out already: listed only once, as a project
        let out = Registry::parse(&text(r#"{"id":"a","name":"A"}"#)).unwrap();
        assert_eq!(out.teasers().count(), 0);
        assert!(Registry::parse(&text(r#"{"id":"../b","name":"B"}"#)).is_err());
        assert!(Registry::parse(&text(r#"{"id":"b","name":"B","art":{"hero":"../x.jpg"}}"#)).is_err());
        // A list without it, as before
        assert!(Registry::parse(r#"{"schema":1,"projects":[]}"#).unwrap().coming_soon.is_empty());
    }

    #[test]
    fn a_local_list_adds_and_replaces_projects() {
        let dir = std::env::temp_dir().join(format!("nico-local-registry-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = local_file_next_to(&dir.join("config.json"));
        assert_eq!(path.file_name().unwrap(), "registry.local.json");
        let base = || Registry::parse(r#"{"schema":1,"projects":[{"id":"a","name":"A","repo":"o/a","branch":"main"}],"coming_soon":[{"id":"b","name":"B"}]}"#).unwrap();

        // No file: as it was
        let _ = std::fs::remove_file(&path);
        assert_eq!(base().with_local(&path), base());

        // A new one goes at the end and leaves the coming-soon list
        std::fs::write(&path, r#"{"projects":[{"id":"b","name":"B","repo":"o/b","branch":"dev","exe":"B.exe"}]}"#).unwrap();
        let reg = base().with_local(&path);
        assert_eq!(reg.projects.iter().map(|p| p.id.as_str()).collect::<Vec<_>>(), ["a", "b"]);
        assert_eq!(reg.find("b").unwrap().branch, "dev");
        assert_eq!(reg.teasers().count(), 0);

        // The same id takes the place of the listed one
        std::fs::write(&path, r#"{"projects":[{"id":"a","name":"A test","repo":"o/a","branch":"test"}]}"#).unwrap();
        let reg = base().with_local(&path);
        assert_eq!(reg.projects.len(), 1);
        assert_eq!(reg.find("a").unwrap().branch, "test");

        // Broken or with a bad entry: nothing changes
        for bad in ["{", r#"{"projects":[{"id":"../x","name":"X","repo":"o/x","branch":"main"}]}"#, r#"{"projects":[{"id":"x"}]}"#] {
            std::fs::write(&path, bad).unwrap();
            assert_eq!(base().with_local(&path), base());
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn platforms_and_their_guides() {
        let entry = |plats: &str| Registry::parse(&format!(r#"{{"schema":1,"projects":[{{"id":"a","name":"A","repo":"o/r","branch":"main","platforms":{plats}}}]}}"#));
        let reg = entry(r#"[{"os":"windows"},{"os":"macos","guide":"docs/MACOS.md"},{"os":"linux","soon":true},{"os":"amiga"}]"#).unwrap();
        let p = &reg.projects[0];
        assert_eq!(p.platforms.len(), 4);
        assert!(p.platforms[2].soon && !p.platforms[1].soon);
        assert_eq!(p.page_url(p.platforms[1].guide.as_ref().unwrap()), "https://github.com/o/r/blob/main/docs/MACOS.md");
        assert!(entry(r#"[{"os":"macos","guide":"../x.md"}]"#).is_err());
        assert!(entry(r#"[{"guide":"x.md"}]"#).is_err());
        // Without the list, as before
        assert!(Registry::parse(r#"{"schema":1,"projects":[{"id":"a","name":"A","repo":"o/r","branch":"main"}]}"#).unwrap().projects[0].platforms.is_empty());
        let swt = Registry::bundled();
        let mac = swt.find("swtrilogy").unwrap().platforms.iter().find(|x| x.os == "macos").unwrap();
        assert_eq!(mac.guide.as_deref(), Some("docs/MACOS.md"));
        let linux = swt.find("swtrilogy").unwrap().platforms.iter().find(|x| x.os == "linux").unwrap();
        assert_eq!(linux.guide.as_deref(), Some("docs/LINUX.md"));
        assert!(!linux.soon);
    }

    #[test]
    fn credits_of_contributors() {
        let entry = |credits: &str| Registry::parse(&format!(r#"{{"schema":1,"projects":[{{"id":"a","name":"A","repo":"o/r","branch":"main","credits":{credits}}}]}}"#));
        let reg = entry(r#"[{"github":"Some-One","role":"port","os":"macos"},{"github":"x2","role":"Sound fixes"}]"#).unwrap();
        let c = &reg.projects[0].credits;
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].os.as_deref(), Some("macos"));
        assert_eq!(c[0].profile_url(), "https://github.com/Some-One");
        assert_eq!(c[1].os, None);
        for bad in ["../x", "a b", "-a", "a--b", "", "a/b"] {
            assert!(entry(&format!(r#"[{{"github":"{bad}","role":"port"}}]"#)).is_err(), "{bad}");
        }
        assert!(entry(r#"[{"github":"a","role":" "}]"#).is_err());
        assert!(entry(r#"[{"role":"port"}]"#).is_err());
        // Without the list, as before
        assert!(Registry::parse(r#"{"schema":1,"projects":[{"id":"a","name":"A","repo":"o/r","branch":"main"}]}"#).unwrap().projects[0].credits.is_empty());
        let swt = Registry::bundled();
        let credits: Vec<_> = swt.find("swtrilogy").unwrap().credits.iter().map(|c| (c.github.as_str(), c.os.as_deref())).collect();
        assert_eq!(credits, [("Nutcasey", Some("macos")), ("vojenfr", Some("linux")), ("BrianGilbert", None)]);
    }

    #[test]
    fn rejects_paths_that_leave_the_folder() {
        assert!(is_plain_relative("tools/update.ps1"));
        assert!(is_plain_relative("install.bat"));
        for bad in ["..\\x.bat", "a/../../x", "C:\\x.bat", "/x", "\\\\host\\x", ""] {
            assert!(!is_plain_relative(bad), "{bad}");
        }
        let text = r#"{"schema":1,"projects":[{"id":"a","name":"A","repo":"o/r","branch":"main",
            "scripts":{"install":"..\\evil.bat"}}]}"#;
        assert!(Registry::parse(text).is_err());
    }

    #[test]
    fn requirements_are_optional_and_checked() {
        let entry = |extra: &str| format!(r#"{{"schema":1,"projects":[{{"id":"a","name":"A","repo":"o/r","branch":"main"{extra}}}]}}"#);
        assert!(Registry::parse(&entry("")).unwrap().projects[0].requires.is_empty());
        // A key from a later launcher is ignored, a bad version is not
        let reg = Registry::parse(&entry(r#","requires":{"python":"3.10","ninja":"1.11"}"#)).unwrap();
        assert_eq!(reg.projects[0].requires.python.as_deref(), Some("3.10"));
        assert!(Registry::parse(&entry(r#","requires":{"cmake":"latest"}"#)).is_err());
    }

    #[test]
    fn rejects_a_newer_schema_and_bad_repos() {
        assert!(Registry::parse(r#"{"schema":2,"projects":[]}"#).is_err());
        let text = r#"{"schema":1,"projects":[{"id":"a","name":"A","repo":"o/r/../x","branch":"main"}]}"#;
        assert!(Registry::parse(text).is_err());
    }
}
