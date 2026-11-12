//! `hub.toml`: the file a project keeps in its repository to tell the launcher
//! how to build it. It says what the project's `build.bat` and `get_deps.bat`
//! do today, as data, so that the launcher runs it the same way for every
//! project. The format is described in `docs/manifest.md`.
//!
//! This module reads and checks the file. `build` runs it.
//!
//! A manifest never says where a project's own file (its ROM) comes from: the
//! `[input]` table only names it, and `{input}` is the path the user picked.

use crate::registry::is_plain_relative;
use crate::requirements::Requires;
use crate::{err, Result};
use serde::{Deserialize, Serialize};

/// The file's name, at the top of the project's repository.
pub const FILE: &str = "hub.toml";
/// The manifest format this build understands.
pub const SCHEMA: u32 = 1;

/// What a step's `run` can refer to, as `{name}`.
pub const PLACEHOLDERS: [&str; 4] = ["python", "cmake", "ctest", "input"];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    pub schema: u32,
    /// The program the build leaves in the project's folder.
    pub exe: String,
    /// News file (the `[N] title` format), if the project has one.
    #[serde(default)]
    pub news: Option<String>,
    #[serde(default)]
    pub requires: Requires,
    #[serde(default)]
    pub input: Option<Input>,
    /// Files downloaded before the steps, each checked against its SHA256.
    #[serde(default, rename = "fetch")]
    pub fetches: Vec<Fetch>,
    /// The build, in order. The first step that fails stops it.
    #[serde(default, rename = "step")]
    pub steps: Vec<Step>,
}

/// A file only the user can provide. Never downloaded.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Input {
    /// What to ask for, as the project's README names it.
    pub label: String,
    /// Files that exist once the input has been used. When they are all
    /// there, the input is not asked for again.
    #[serde(default)]
    pub done_when: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Fetch {
    pub name: String,
    pub url: String,
    /// Of the whole file, in hex. What is downloaded is thrown away when it
    /// does not match.
    pub sha256: String,
    /// Where to leave it, inside the project's folder.
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    /// Shown while it runs.
    pub name: String,
    /// A program and its arguments, run in the project's folder. No shell:
    /// each item is one argument, whatever spaces it has.
    #[serde(default)]
    pub run: Vec<String>,
    /// Instead of `run`: copy a file inside the project's folder.
    #[serde(default)]
    pub copy: Option<Copy>,
    /// Run with the Visual Studio x64 environment (what `vcvars64.bat` sets).
    #[serde(default)]
    pub msvc_env: bool,
    /// The step is skipped when all of these exist.
    #[serde(default)]
    pub skip_if_exists: Vec<String>,
    /// A failure is reported as a warning and the build goes on (tests that
    /// run after the program is already built).
    #[serde(default)]
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Copy {
    pub from: String,
    pub to: String,
}

/// The `{name}` references in one argument. `{{` and `}}` are literal braces.
fn placeholders(arg: &str) -> Result<Vec<&str>> {
    let mut found = Vec::new();
    let mut rest = arg;
    while let Some(at) = rest.find(['{', '}']) {
        let (brace, after) = (rest.as_bytes()[at], &rest[at + 1..]);
        if after.as_bytes().first() == Some(&brace) {
            rest = &after[1..];
        } else if brace == b'{' {
            let end = after.find('}').ok_or_else(|| format!("Unclosed {{ in {arg:?}"))?;
            found.push(&after[..end]);
            rest = &after[end + 1..];
        } else {
            return Err(format!("Stray }} in {arg:?}"));
        }
    }
    Ok(found)
}

/// Puts the values in place of the `{name}` references of one argument.
pub fn expand(arg: &str, value: &dyn Fn(&str) -> Option<String>) -> Result<String> {
    let mut out = String::new();
    let mut rest = arg;
    while let Some(at) = rest.find(['{', '}']) {
        out.push_str(&rest[..at]);
        let (brace, after) = (rest.as_bytes()[at], &rest[at + 1..]);
        if after.as_bytes().first() == Some(&brace) {
            out.push(brace as char);
            rest = &after[1..];
        } else if brace == b'{' {
            let end = after.find('}').ok_or_else(|| format!("Unclosed {{ in {arg:?}"))?;
            let name = &after[..end];
            out.push_str(&value(name).ok_or_else(|| format!("Nothing to put in {{{name}}}"))?);
            rest = &after[end + 1..];
        } else {
            return Err(format!("Stray }} in {arg:?}"));
        }
    }
    out.push_str(rest);
    Ok(out)
}

fn inside(what: &str, rel: &str) -> Result<()> {
    if is_plain_relative(rel) {
        Ok(())
    } else {
        Err(format!("{what}: {rel:?} is not a path inside the project's folder"))
    }
}

impl Manifest {
    pub fn parse(text: &str) -> Result<Manifest> {
        // The schema first, alone: a newer file can have tables this build
        // would call unknown
        #[derive(Deserialize)]
        struct Head {
            schema: u32,
        }
        let head: Head = toml::from_str(text).map_err(err("Bad hub.toml"))?;
        if head.schema == 0 || head.schema > SCHEMA {
            return Err(format!(
                "hub.toml is in format {} and this launcher reads up to {SCHEMA}: update the launcher.",
                head.schema
            ));
        }
        let m: Manifest = toml::from_str(text).map_err(err("Bad hub.toml"))?;
        m.validate()?;
        Ok(m)
    }

    fn validate(&self) -> Result<()> {
        inside("exe", &self.exe)?;
        if let Some(news) = &self.news {
            inside("news", news)?;
        }
        self.requires.validate()?;
        if let Some(input) = &self.input {
            if input.label.trim().is_empty() {
                return Err("[input] needs a label".into());
            }
            for f in &input.done_when {
                inside("[input] done_when", f)?;
            }
        }
        for f in &self.fetches {
            let what = format!("[[fetch]] {}", f.name);
            // https only, and no user:password@ or port tricks in the host
            let host = f.url.strip_prefix("https://").and_then(|r| r.split('/').next()).unwrap_or("");
            let host_ok = !host.is_empty() && host.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-');
            if !host_ok || f.url.chars().any(|c| c.is_whitespace() || c.is_control()) {
                return Err(format!("{what}: the url must be a plain https:// address"));
            }
            if f.sha256.len() != 64 || !f.sha256.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(format!("{what}: sha256 must be 64 hex digits"));
            }
            inside(&what, &f.to)?;
        }
        for s in &self.steps {
            let what = format!("[[step]] {}", s.name);
            if s.name.trim().is_empty() {
                return Err("A [[step]] has no name".into());
            }
            match (&s.copy, s.run.is_empty()) {
                (Some(_), false) | (None, true) => return Err(format!("{what}: give it run or copy, one of them")),
                (Some(c), true) => {
                    inside(&what, &c.from)?;
                    inside(&what, &c.to)?;
                    if s.msvc_env {
                        return Err(format!("{what}: msvc_env is for run"));
                    }
                }
                (None, false) => {}
            }
            for f in &s.skip_if_exists {
                inside(&what, f)?;
            }
            for arg in &s.run {
                for name in placeholders(arg).map_err(|e| format!("{what}: {e}"))? {
                    let asked = match name {
                        "python" => self.requires.python.is_some(),
                        "cmake" | "ctest" => self.requires.cmake.is_some() || self.requires.msvc,
                        "input" => self.input.is_some(),
                        _ => return Err(format!("{what}: unknown {{{name}}} (there are: {})", PLACEHOLDERS.join(", "))),
                    };
                    if !asked {
                        let table = if name == "input" { "[input]" } else { "[requires]" };
                        return Err(format!("{what}: {{{name}}} is used but {table} does not ask for it"));
                    }
                }
            }
            if s.msvc_env && !self.requires.msvc {
                return Err(format!("{what}: msvc_env is used but [requires] does not have msvc = true"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SWT: &str = include_str!("../../../examples/StarWarsTrilogyNative.hub.toml");
    const LW: &str = include_str!("../../../examples/LostWorldNative.hub.toml");

    #[test]
    fn the_example_manifests_are_valid() {
        let swt = Manifest::parse(SWT).unwrap();
        assert_eq!(swt.exe, "StarWarsTrilogy.exe");
        assert_eq!(swt.requires, Requires { python: Some("3.10".into()), cmake: Some("3.27".into()), msvc: true, ..Default::default() });
        assert_eq!(swt.fetches.len(), 2);
        assert_eq!(swt.steps.first().unwrap().run[0], "{python}");
        assert!(swt.steps.iter().any(|s| s.msvc_env));
        let lw = Manifest::parse(LW).unwrap();
        assert_eq!(lw.input.unwrap().done_when, ["data/crom.bin", "data/sound_samples.bin"]);
        // Its tests run after the game is copied and do not take it away
        let copied = lw.steps.iter().position(|s| s.copy.is_some()).unwrap();
        let tests = lw.steps.iter().position(|s| s.run.first().is_some_and(|a| a == "{ctest}")).unwrap();
        assert!(copied < tests && lw.steps[tests].optional);
    }

    /// The example that builds with the kit names tools this launcher has.
    #[test]
    fn the_kit_example_is_valid() {
        let m = Manifest::parse(include_str!("../../../examples/StarWarsTrilogyNative.kit.hub.toml")).unwrap();
        assert!(!m.requires.msvc && m.steps.iter().all(|s| !s.msvc_env));
        assert_eq!(crate::kit::Kit::builtin().select(&m.requires.kit).unwrap().len(), 4);
    }

    /// The examples say the same as the registry about the same project.
    #[test]
    fn the_examples_agree_with_the_registry() {
        let reg = crate::registry::Registry::bundled();
        for (id, text) in [("swtrilogy", SWT), ("lostworld", LW)] {
            let (p, m) = (reg.find(id).unwrap(), Manifest::parse(text).unwrap());
            assert_eq!(p.exe.as_deref(), Some(m.exe.as_str()), "{id}");
            assert_eq!(p.news, m.news, "{id}");
            assert_eq!(p.requires, m.requires, "{id}");
            assert_eq!(p.input.as_ref().map(|i| &i.label), m.input.as_ref().map(|i| &i.label), "{id}");
        }
    }

    fn with(extra: &str) -> Result<Manifest> {
        Manifest::parse(&format!("schema = 1\nexe = \"a.exe\"\n{extra}"))
    }

    #[test]
    fn the_smallest_manifest() {
        let m = with("").unwrap();
        assert!(m.requires.is_empty() && m.input.is_none() && m.fetches.is_empty() && m.steps.is_empty());
    }

    #[test]
    fn refuses_a_newer_format_and_misspelt_keys() {
        assert!(Manifest::parse("schema = 2\nexe = \"a.exe\"\n[future]\nx = 1").unwrap_err().contains("update the launcher"));
        assert!(Manifest::parse("exe = \"a.exe\"").is_err(), "no schema");
        assert!(with("newz = \"news.txt\"").is_err());
        assert!(with("[[step]]\nname = \"x\"\nrun = [\"a\"]\nskip_if_exist = [\"b\"]").is_err());
    }

    #[test]
    fn refuses_paths_outside_the_project() {
        assert!(Manifest::parse("schema = 1\nexe = \"..\\\\a.exe\"").is_err());
        assert!(with("news = \"C:/news.txt\"").is_err());
        assert!(with("[input]\nlabel = \"x\"\ndone_when = [\"/etc/passwd\"]").is_err());
        assert!(with("[[step]]\nname = \"x\"\ncopy = { from = \"a\", to = \"../b\" }").is_err());
        assert!(with("[[step]]\nname = \"x\"\nrun = [\"a\"]\nskip_if_exists = [\"../b\"]").is_err());
    }

    #[test]
    fn checks_downloads() {
        let fetch = |url: &str, sha: &str, to: &str| {
            with(&format!("[[fetch]]\nname = \"lib\"\nurl = \"{url}\"\nsha256 = \"{sha}\"\nto = \"{to}\""))
        };
        let sha = "9c75cf16330322c217dedd2e0609f1124f1b54b8633e763467b4684d0f4334a3";
        assert!(fetch("https://github.com/o/r/releases/download/1/lib.zip", sha, "third_party/downloads/lib.zip").is_ok());
        assert!(fetch("http://github.com/lib.zip", sha, "lib.zip").is_err(), "not https");
        assert!(fetch("https://github.com@evil.example/lib.zip", sha, "lib.zip").is_err());
        assert!(fetch("https://github.com/lib.zip", "abc", "lib.zip").is_err());
        assert!(fetch("https://github.com/lib.zip", sha, "../lib.zip").is_err());
        assert!(with("[[fetch]]\nname = \"lib\"\nurl = \"https://github.com/lib.zip\"\nto = \"lib.zip\"").is_err(), "no sha256");
    }

    #[test]
    fn checks_steps() {
        let step = |body: &str| with(&format!("[requires]\npython = \"3.10\"\n[[step]]\nname = \"x\"\n{body}"));
        assert!(step("run = [\"{python}\", \"tools/a.py\"]").is_ok());
        assert!(step("").is_err(), "neither run nor copy");
        assert!(step("run = [\"a\"]\ncopy = { from = \"a\", to = \"b\" }").is_err(), "both");
        assert!(step("run = [\"{pyton}\"]").unwrap_err().contains("unknown {pyton}"));
        assert!(step("run = [\"{python\"]").is_err());
        assert!(step("run = [\"{input}\"]").unwrap_err().contains("[input]"));
        assert!(step("run = [\"{cmake}\"]").unwrap_err().contains("[requires]"));
        assert!(step("run = [\"a\"]\nmsvc_env = true").is_err());
        assert!(step("run = [\"a\", \"{{literal}}\"]").is_ok());
    }

    #[test]
    fn expands_placeholders() {
        let value = |name: &str| match name {
            "python" => Some(r"C:\Python 3\python.exe".to_string()),
            "input" => Some(r"D:\my roms\game.zip".to_string()),
            _ => None,
        };
        assert_eq!(expand("{python}", &value).unwrap(), r"C:\Python 3\python.exe");
        assert_eq!(expand("--rom={input}", &value).unwrap(), r"--rom=D:\my roms\game.zip");
        assert_eq!(expand("{{x}} plain", &value).unwrap(), "{x} plain");
        assert!(expand("{cmake}", &value).is_err());
    }
}
