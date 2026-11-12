//! What a project needs on the PC before it can be built, and whether it is
//! there: the checks the projects' `find_python.bat` and `find_vs.bat` do,
//! done once here for every project.
//!
//! This only looks. Installing what is missing is still the projects'
//! `install.bat` (winget) until the launcher takes that over too.

use crate::runner::no_window;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// What a project asks for: `requires` in `registry.json` and `[requires]` in a
/// project's `hub.toml`. Unknown keys are ignored, so a registry that asks for
/// something new still loads in an older launcher.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Requires {
    /// Minimum Python, as `"3.10"`.
    #[serde(default)]
    pub python: Option<String>,
    /// Minimum CMake, as `"3.27"`.
    #[serde(default)]
    pub cmake: Option<String>,
    /// Visual Studio 2022 or later (Build Tools or any edition) with the x64
    /// C++ tools, its CMake, and a Windows SDK.
    #[serde(default)]
    pub msvc: bool,
    /// Tools of the launcher's own build kit (`kit.json`), by name. The
    /// launcher downloads them into its own folder; a `python` or `cmake`
    /// named here is used instead of one looked for on the PC.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub kit: Vec<String>,
}

impl Requires {
    pub fn is_empty(&self) -> bool {
        *self == Requires::default()
    }
}

impl Requires {
    pub fn validate(&self) -> Result<(), String> {
        for v in [&self.python, &self.cmake].into_iter().flatten() {
            if parse_version(v).is_none() {
                return Err(format!("Bad version in the requirements: {v:?}"));
            }
        }
        Ok(())
    }

    /// The most demanding of two: for one report that covers several projects.
    pub fn merge(&self, other: &Requires) -> Requires {
        let newer = |a: &Option<String>, b: &Option<String>| match (a, b) {
            (Some(a), Some(b)) => Some(if parse_version(a) >= parse_version(b) { a.clone() } else { b.clone() }),
            (a, b) => a.clone().or_else(|| b.clone()),
        };
        Requires {
            python: newer(&self.python, &other.python),
            cmake: newer(&self.cmake, &other.cmake),
            msvc: self.msvc || other.msvc,
            kit: {
                let mut kit = self.kit.clone();
                kit.extend(other.kit.iter().filter(|t| !self.kit.contains(t)).cloned());
                kit
            },
        }
    }
}

/// `"3.12.4"` or `"3.10"` as numbers. Anything after the numbers (`"3.13.0rc1"`,
/// `"3.29.5-msvc4"`) is ignored.
pub fn parse_version(text: &str) -> Option<Vec<u32>> {
    let mut parts = Vec::new();
    for piece in text.trim().split('.') {
        let digits: String = piece.chars().take_while(|c| c.is_ascii_digit()).collect();
        if digits.is_empty() {
            break;
        }
        parts.push(digits.parse().ok()?);
        if digits.len() != piece.len() {
            break;
        }
    }
    if parts.is_empty() { None } else { Some(parts) }
}

pub fn at_least(found: &str, minimum: &str) -> bool {
    match (parse_version(found), parse_version(minimum)) {
        (Some(f), Some(m)) => f >= m,
        _ => false,
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Check {
    pub id: &'static str,
    pub ok: bool,
    /// The version found, when there is one.
    pub version: Option<String>,
    /// Where it is, or what is missing.
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Python {
    /// The command that starts it: `["py", "-3"]`, `["python"]` or a full path.
    pub command: Vec<String>,
    pub version: String,
    pub path: String,
}

fn quiet(program: &str) -> Command {
    let mut cmd = Command::new(program);
    cmd.stdin(Stdio::null()).stderr(Stdio::null());
    no_window(&mut cmd);
    cmd
}

/// Runs a candidate and asks it what it is. `python` in PATH can be an older
/// Python or the Microsoft Store alias (which only opens the Store), so every
/// candidate is run, never trusted by name.
fn probe_python(command: &[String]) -> Option<Python> {
    let out = quiet(&command[0])
        .args(&command[1..])
        .args(["-c", "import sys; print(sys.version.split()[0]); print(sys.executable)"])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    let mut lines = text.lines();
    let version = lines.next()?.trim().to_string();
    let path = lines.next().unwrap_or("").trim().to_string();
    parse_version(&version)?;
    Some(Python { command: command.to_vec(), version, path })
}

fn python_candidates() -> Vec<Vec<String>> {
    let s = |parts: &[&str]| parts.iter().map(|p| p.to_string()).collect::<Vec<_>>();
    let mut list = vec![s(&["py", "-3"]), s(&["python"])];
    if cfg!(not(windows)) {
        list.push(s(&["python3"]));
    }
    // The python.org installs, in PATH or not: per user (what install.bat
    // makes) and for all users
    for var in ["LOCALAPPDATA", "ProgramFiles"] {
        let Some(base) = std::env::var_os(var) else { continue };
        let base = PathBuf::from(base);
        let base = if var == "LOCALAPPDATA" { base.join("Programs").join("Python") } else { base };
        let Ok(entries) = std::fs::read_dir(&base) else { continue };
        let mut dirs: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("Python3")))
            .collect();
        dirs.sort();
        for dir in dirs.into_iter().rev() {
            let exe = dir.join("python.exe");
            if exe.is_file() {
                list.push(vec![exe.to_string_lossy().into_owned()]);
            }
        }
    }
    list
}

/// The first Python that is at least `minimum`.
pub fn find_python(minimum: &str) -> Option<Python> {
    python_candidates()
        .iter()
        .filter_map(|c| probe_python(c))
        .find(|p| at_least(&p.version, minimum))
}

/// A Visual Studio folder with the x64 C++ tools and its own CMake.
pub fn is_vs_with_cpp(dir: &Path) -> bool {
    dir.join("VC/Auxiliary/Build/vcvars64.bat").is_file() && vs_cmake(dir).is_file()
}

pub fn vs_cmake(dir: &Path) -> PathBuf {
    dir.join("Common7/IDE/CommonExtensions/Microsoft/CMake/CMake/bin/cmake.exe")
}

/// The newest Windows SDK under a Windows Kits folder that has the x64 headers
/// and libraries the build needs. The SDK is its own component in the Visual
/// Studio Installer: the C++ tools do not bring it.
pub fn find_windows_sdk(kits: &Path) -> Option<String> {
    let mut versions: Vec<String> = std::fs::read_dir(kits.join("Include"))
        .ok()?
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|v| v.starts_with("10."))
        .filter(|v| {
            kits.join("Include").join(v).join("um/windows.h").is_file()
                && kits.join("Include").join(v).join("ucrt/stdio.h").is_file()
                && kits.join("Lib").join(v).join("um/x64/kernel32.lib").is_file()
                && kits.join("Lib").join(v).join("ucrt/x64/ucrt.lib").is_file()
        })
        .collect();
    versions.sort_by_key(|v| parse_version(v));
    versions.pop()
}

/// Visual Studio with the C++ tools: vswhere first, with `-all` (without it an
/// instance waiting for a Windows restart is left out, common right after
/// installing Build Tools), then the default folders.
#[cfg(windows)]
pub fn find_visual_studio() -> Option<PathBuf> {
    let x86 = PathBuf::from(std::env::var_os("ProgramFiles(x86)")?);
    let vswhere = x86.join("Microsoft Visual Studio/Installer/vswhere.exe");
    if vswhere.is_file() {
        let out = quiet(&vswhere.to_string_lossy())
            .args(["-all", "-latest", "-products", "*", "-requires"])
            .args(["Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "Microsoft.VisualStudio.Component.VC.CMake.Project"])
            .args(["-property", "installationPath"])
            .output()
            .ok();
        if let Some(out) = out {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                let dir = PathBuf::from(line.trim());
                if is_vs_with_cpp(&dir) {
                    return Some(dir);
                }
            }
        }
    }
    for root in [std::env::var_os("ProgramFiles").map(PathBuf::from), Some(x86)].into_iter().flatten() {
        let Ok(years) = std::fs::read_dir(root.join("Microsoft Visual Studio")) else { continue };
        for year in years.flatten() {
            let Ok(editions) = std::fs::read_dir(year.path()) else { continue };
            for edition in editions.flatten() {
                if is_vs_with_cpp(&edition.path()) {
                    return Some(edition.path());
                }
            }
        }
    }
    None
}

#[cfg(not(windows))]
pub fn find_visual_studio() -> Option<PathBuf> {
    None
}

/// The Windows Kits folder: from the registry, as vcvars64.bat reads it, the
/// default one otherwise.
#[cfg(windows)]
pub fn windows_kits_root() -> Option<PathBuf> {
    for key in [r"HKLM\SOFTWARE\Microsoft\Windows Kits\Installed Roots", r"HKLM\SOFTWARE\Wow6432Node\Microsoft\Windows Kits\Installed Roots"] {
        let Ok(out) = quiet("reg").args(["query", key, "/v", "KitsRoot10"]).output() else { continue };
        for line in String::from_utf8_lossy(&out.stdout).lines() {
            if let Some((_, value)) = line.split_once("REG_SZ") {
                let dir = PathBuf::from(value.trim());
                if dir.is_dir() {
                    return Some(dir);
                }
            }
        }
    }
    let default = PathBuf::from(std::env::var_os("ProgramFiles(x86)")?).join("Windows Kits/10");
    default.is_dir().then_some(default)
}

#[cfg(not(windows))]
pub fn windows_kits_root() -> Option<PathBuf> {
    None
}

/// `cmake version 3.29.5-msvc4` -> `3.29.5-msvc4`
pub fn cmake_version(cmake: &Path) -> Option<String> {
    let out = quiet(&cmake.to_string_lossy()).arg("--version").output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let version = text.lines().next()?.strip_prefix("cmake version ")?.trim().to_string();
    parse_version(&version).map(|_| version)
}

/// Every CMake that answers, with its version: Visual Studio's own first, then
/// the one in PATH (as build.bat does).
fn cmake_candidates(vs: Option<&Path>) -> Vec<(PathBuf, String)> {
    let candidates = vs.into_iter().map(vs_cmake).chain([PathBuf::from("cmake")]);
    candidates.filter_map(|c| cmake_version(&c).map(|v| (c, v))).collect()
}

/// The CMake to build with: the first one that is at least `minimum` (any,
/// when no minimum is asked).
pub fn find_cmake(minimum: Option<&str>, vs: Option<&Path>) -> Option<PathBuf> {
    cmake_candidates(vs).into_iter().find(|(_, v)| minimum.is_none_or(|m| at_least(v, m))).map(|(path, _)| path)
}

/// Looks for everything `requires` asks for. One entry per thing asked.
pub fn check(requires: &Requires) -> Vec<Check> {
    let mut report = Vec::new();
    if let Some(min) = &requires.python {
        report.push(match find_python(min) {
            Some(p) => Check { id: "python", ok: true, version: Some(p.version), detail: p.path },
            None => Check { id: "python", ok: false, version: None, detail: format!("Python {min} or later not found") },
        });
    }
    let vs = if requires.msvc { find_visual_studio() } else { None };
    if requires.msvc {
        report.push(match &vs {
            Some(dir) => Check { id: "msvc", ok: true, version: None, detail: dir.to_string_lossy().into_owned() },
            None => Check { id: "msvc", ok: false, version: None, detail: "Visual Studio 2022 with the x64 C++ tools and CMake not found".into() },
        });
        report.push(match windows_kits_root().and_then(|kits| find_windows_sdk(&kits)) {
            Some(v) => Check { id: "winsdk", ok: true, version: Some(v), detail: String::new() },
            None => Check { id: "winsdk", ok: false, version: None, detail: "Windows SDK not found".into() },
        });
    }
    if let Some(min) = &requires.cmake {
        let found = cmake_candidates(vs.as_deref());
        report.push(match found.iter().find(|(_, v)| at_least(v, min)) {
            Some((path, v)) => Check { id: "cmake", ok: true, version: Some(v.clone()), detail: path.to_string_lossy().into_owned() },
            None => match found.first() {
                Some((_, v)) => Check { id: "cmake", ok: false, version: Some(v.clone()), detail: format!("CMake {min} or later is needed") },
                None => Check { id: "cmake", ok: false, version: None, detail: format!("CMake {min} or later not found") },
            },
        });
    }
    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn compares_versions_as_numbers() {
        assert_eq!(parse_version("3.12.4"), Some(vec![3, 12, 4]));
        assert_eq!(parse_version("3.13.0rc1"), Some(vec![3, 13, 0]));
        assert_eq!(parse_version("3.29.5-msvc4"), Some(vec![3, 29, 5]));
        assert_eq!(parse_version("10.0.22621.0"), Some(vec![10, 0, 22621, 0]));
        assert_eq!(parse_version("abc"), None);
        assert!(at_least("3.10.0", "3.10"));
        assert!(at_least("3.12.4", "3.10"), "12 is more than 10, not less");
        assert!(!at_least("3.9.13", "3.10"));
        assert!(!at_least("2.7.18", "3.10"));
        assert!(!at_least("nonsense", "3.10"));
    }

    #[test]
    fn merges_to_the_most_demanding() {
        let a = Requires { python: Some("3.10".into()), kit: vec!["ninja".into()], ..Default::default() };
        let b = Requires { python: Some("3.9".into()), cmake: Some("3.27".into()), msvc: true, kit: vec!["cmake".into(), "ninja".into()] };
        let m = a.merge(&b);
        assert_eq!(m, Requires { python: Some("3.10".into()), cmake: Some("3.27".into()), msvc: true, kit: vec!["ninja".into(), "cmake".into()] });
        assert!(m.validate().is_ok());
        assert!(Requires { python: Some("new".into()), ..Default::default() }.validate().is_err());
    }

    #[test]
    fn finds_a_python_that_is_new_enough_and_refuses_an_impossible_one() {
        // Whatever runs these tests has a Python 3 (the projects need one too)
        if let Some(p) = find_python("3.0") {
            assert!(at_least(&p.version, "3.0"));
            assert!(!p.path.is_empty());
        }
        assert!(find_python("99.0").is_none());
    }

    #[test]
    fn picks_the_newest_complete_windows_sdk() {
        let kits = std::env::temp_dir().join(format!("nl-kits-{}", std::process::id()));
        let _ = fs::remove_dir_all(&kits);
        assert_eq!(find_windows_sdk(&kits), None);
        let add = |v: &str, complete: bool| {
            for f in ["Include/V/um/windows.h", "Include/V/ucrt/stdio.h", "Lib/V/um/x64/kernel32.lib", "Lib/V/ucrt/x64/ucrt.lib"] {
                if !complete && f.starts_with("Lib") {
                    continue;
                }
                let path = kits.join(f.replace('V', v));
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                fs::write(path, "").unwrap();
            }
        };
        add("10.0.26100.0", false); // headers only: not usable
        assert_eq!(find_windows_sdk(&kits), None);
        add("10.0.9999.0", true);
        add("10.0.22621.0", true);
        assert_eq!(find_windows_sdk(&kits).as_deref(), Some("10.0.22621.0"), "by number, not by text");
        fs::remove_dir_all(&kits).unwrap();
    }

    #[test]
    fn knows_a_visual_studio_folder_with_the_cpp_tools() {
        let vs = std::env::temp_dir().join(format!("nl-vs-{}", std::process::id()));
        let _ = fs::remove_dir_all(&vs);
        fs::create_dir_all(vs.join("VC/Auxiliary/Build")).unwrap();
        fs::write(vs.join("VC/Auxiliary/Build/vcvars64.bat"), "").unwrap();
        assert!(!is_vs_with_cpp(&vs), "no CMake component");
        fs::create_dir_all(vs_cmake(&vs).parent().unwrap()).unwrap();
        fs::write(vs_cmake(&vs), "").unwrap();
        assert!(is_vs_with_cpp(&vs));
        fs::remove_dir_all(&vs).unwrap();
    }

    #[test]
    fn reports_only_what_was_asked() {
        assert!(check(&Requires::default()).is_empty());
        let report = check(&Requires { python: Some("99.0".into()), ..Default::default() });
        assert_eq!(report.len(), 1);
        assert_eq!(report[0].id, "python");
        assert!(!report[0].ok);
    }

    /// The Windows CI runner has Visual Studio, a Windows SDK and CMake: the
    /// real detection must find them there.
    #[cfg(windows)]
    #[test]
    fn finds_the_real_tools_on_a_windows_build_machine() {
        let report = check(&Requires { python: Some("3.10".into()), cmake: Some("3.20".into()), msvc: true, ..Default::default() });
        let ids: Vec<&str> = report.iter().map(|c| c.id).collect();
        assert_eq!(ids, ["python", "msvc", "winsdk", "cmake"]);
        for c in &report {
            assert!(c.ok, "{report:#?}");
        }
    }
}
