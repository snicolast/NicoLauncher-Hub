//! The list of projects the launcher offers. A copy is built into the exe and a
//! newer one is read from the launcher's repository, so adding a project needs
//! no new release of the launcher.

use crate::requirements::Requires;
use crate::{err, http_get, Result};
use serde::{Deserialize, Serialize};

pub const REGISTRY_URL: &str =
    "https://raw.githubusercontent.com/snicolast/NicoLauncher-Hub/main/registry.json";
const BUNDLED: &str = include_str!("../../../registry.json");
/// The registry format this build understands.
pub const SCHEMA: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Registry {
    pub schema: u32,
    pub projects: Vec<Project>,
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

    /// Where GitHub serves a file of the repository's branch.
    pub fn raw_url(&self, rel: &str) -> String {
        format!("https://raw.githubusercontent.com/{}/{}/{}", self.repo, self.branch, rel)
    }
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
            let id_ok = !p.id.is_empty()
                && p.id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
            let repo_ok = p.repo.split('/').count() == 2
                && p.repo.chars().all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c))
                && !p.repo.contains("..");
            let branch_ok = !p.branch.is_empty()
                && p.branch.chars().all(|c| c.is_ascii_alphanumeric() || "-_./".contains(c))
                && !p.branch.contains("..");
            if !id_ok || !repo_ok || !branch_ok {
                return Err(format!("Bad project entry in the registry: {:?}", p.id));
            }
            p.requires.validate().map_err(|e| format!("{e} ({})", p.id))?;
            let s = &p.scripts;
            let files = [&p.exe, &p.news, &s.install, &s.build, &s.update, &p.update_state, &p.art.hero, &p.art.logo];
            for rel in files.into_iter().flatten().chain(&p.art.screens) {
                if !is_plain_relative(rel) {
                    return Err(format!("Bad path in the registry ({}): {rel}", p.id));
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

    pub fn find(&self, id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
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
