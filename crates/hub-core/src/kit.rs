//! The build kit: the compiler and the build tools, downloaded by the
//! launcher into a folder of its own and used only from there. A project that
//! asks for it (`kit` in `[requires]`) builds the same on every PC: nothing
//! is installed on the system, and nothing already on it (another compiler,
//! another Python, a variable some installer left) takes part in the build.
//!
//! What is in it is pinned in `kit.json`, each download with its SHA256. Only
//! tools whose licenses allow passing them on are listed; each comes straight
//! from its own project's releases. Nothing of any game is in the kit: a
//! project is still built on the user's PC, from the user's own file.

use crate::build::fetch;
use crate::manifest::Fetch;
use crate::runner::Cancel;
use crate::{err, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    pub name: String,
    pub version: String,
    pub about: String,
    pub license: String,
    /// A `.zip` or a `.tar.gz`.
    pub url: String,
    pub sha256: String,
    /// Leading folders of the archive to drop.
    #[serde(default)]
    pub strip: usize,
    /// What is not unpacked: paths (after `strip`) that start with one of these.
    #[serde(default)]
    pub skip: Vec<String>,
    /// Folders of the tool that go in PATH (`""` is the tool's own folder).
    #[serde(default)]
    pub path: Vec<String>,
    /// The programs a manifest reaches by placeholder: `python`, `cmake`.
    #[serde(default)]
    pub provides: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Kit {
    pub tools: Vec<Tool>,
}

/// Written last into a tool's folder: one without it was cut off half way.
const MARKER: &str = ".nicolauncher-kit";

impl Kit {
    /// The kit this launcher was built with.
    pub fn builtin() -> Kit {
        Kit::parse(include_str!("../../../kit.json")).expect("kit.json is checked by the tests")
    }

    pub fn parse(text: &str) -> Result<Kit> {
        let kit: Kit = serde_json::from_str(text).map_err(err("Bad kit.json"))?;
        for (i, t) in kit.tools.iter().enumerate() {
            let plain = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || "-._".contains(c));
            if !plain(&t.name) || !plain(&t.version) {
                return Err(format!("Bad kit.json: bad name or version in {:?}", t.name));
            }
            if kit.tools[..i].iter().any(|o| o.name == t.name) {
                return Err(format!("Bad kit.json: {} is listed twice", t.name));
            }
            if !t.url.starts_with("https://") || kind(&t.url).is_none() {
                return Err(format!("Bad kit.json: {} needs an https .zip or .tar.gz", t.name));
            }
            if t.sha256.len() != 64 || !t.sha256.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(format!("Bad kit.json: bad SHA256 for {}", t.name));
            }
            for p in t.path.iter().chain(t.provides.values()) {
                if Path::new(p).components().any(|c| !matches!(c, Component::Normal(_))) {
                    return Err(format!("Bad kit.json: bad path {p:?} in {}", t.name));
                }
            }
            if t.provides.keys().any(|k| k != "python" && k != "cmake") {
                return Err(format!("Bad kit.json: {} provides something unknown", t.name));
            }
        }
        Ok(kit)
    }

    pub fn tool(&self, name: &str) -> Option<&Tool> {
        self.tools.iter().find(|t| t.name == name)
    }

    /// The tools a project names, in the kit's own order. An unknown name is
    /// from a project written for a newer launcher.
    pub fn select(&self, names: &[String]) -> Result<Vec<&Tool>> {
        if let Some(unknown) = names.iter().find(|n| self.tool(n).is_none()) {
            return Err(format!("This project needs {unknown:?} from the build kit, which this version of the launcher does not have. Update the launcher."));
        }
        Ok(self.tools.iter().filter(|t| names.contains(&t.name)).collect())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Zip,
    TarGz,
}

fn kind(url: &str) -> Option<Kind> {
    let file = url.rsplit('/').next().unwrap_or("").to_lowercase();
    if file.ends_with(".zip") {
        Some(Kind::Zip)
    } else if file.ends_with(".tar.gz") || file.ends_with(".tgz") {
        Some(Kind::TarGz)
    } else {
        None
    }
}

/// `%LOCALAPPDATA%\NicoLauncher\kit` (`~/.local/share/...` elsewhere): on
/// this PC only, never in a roaming profile.
pub fn default_root() -> PathBuf {
    let base = std::env::var_os("LOCALAPPDATA")
        .or_else(|| std::env::var_os("XDG_DATA_HOME"))
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local").join("share")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("NicoLauncher").join("kit")
}

impl Tool {
    /// A folder per version: a newer kit never unpacks over an older one.
    pub fn dir(&self, root: &Path) -> PathBuf {
        root.join(format!("{}-{}", self.name, self.version))
    }

    pub fn is_installed(&self, root: &Path) -> bool {
        fs::read_to_string(self.dir(root).join(MARKER)).is_ok_and(|m| m.trim().eq_ignore_ascii_case(&self.sha256))
    }

    pub fn program(&self, root: &Path, what: &str) -> Option<PathBuf> {
        self.provides.get(what).map(|rel| self.dir(root).join(rel))
    }
}

/// The path inside the tool's folder for an archive entry, or None for one
/// that is not unpacked. An entry that would land outside is an error.
fn place(tool: &Tool, name: &Path) -> Result<Option<PathBuf>> {
    let mut parts = Vec::new();
    for c in name.components() {
        match c {
            Component::Normal(p) => parts.push(p.to_string_lossy().into_owned()),
            Component::CurDir => {}
            _ => return Err(format!("Unsafe path in the {} archive: {}", tool.name, name.display())),
        }
    }
    if parts.len() <= tool.strip {
        return Ok(None);
    }
    let rel = parts[tool.strip..].join("/");
    if tool.skip.iter().any(|s| rel.starts_with(s.as_str()) || format!("{rel}/") == *s) {
        return Ok(None);
    }
    Ok(Some(parts[tool.strip..].iter().collect()))
}

fn write_file(to: &Path, from: &mut dyn io::Read) -> Result<()> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).map_err(err("Could not create a folder"))?;
    }
    let mut file = File::create(to).map_err(|e| format!("Could not write {}: {e}", to.display()))?;
    io::copy(from, &mut file).map(|_| ()).map_err(|e| format!("Could not write {}: {e}", to.display()))
}

fn unpack(tool: &Tool, archive: &Path, out: &Path, cancel: &Cancel) -> Result<()> {
    let cancelled = || if cancel.was_asked() { Err("Cancelled.".to_string()) } else { Ok(()) };
    let file = File::open(archive).map_err(err("Could not read the download"))?;
    match kind(&tool.url) {
        Some(Kind::Zip) => {
            let mut zip = zip::ZipArchive::new(io::BufReader::new(file)).map_err(err("The download is not a zip"))?;
            for i in 0..zip.len() {
                cancelled()?;
                let mut entry = zip.by_index(i).map_err(err("Bad zip entry"))?;
                let name = entry.enclosed_name().ok_or_else(|| format!("Unsafe path in the {} archive: {}", tool.name, entry.name()))?;
                let Some(rel) = place(tool, &name)? else { continue };
                if entry.is_dir() {
                    fs::create_dir_all(out.join(rel)).map_err(err("Could not create a folder"))?;
                } else {
                    write_file(&out.join(rel), &mut entry)?;
                }
            }
        }
        Some(Kind::TarGz) => {
            let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(io::BufReader::new(file)));
            for entry in tar.entries().map_err(err("The download is not a tar.gz"))? {
                cancelled()?;
                let mut entry = entry.map_err(err("Bad tar entry"))?;
                let name = entry.path().map_err(err("Bad tar entry"))?.into_owned();
                let Some(rel) = place(tool, &name)? else { continue };
                let kind = entry.header().entry_type();
                if kind.is_dir() {
                    fs::create_dir_all(out.join(rel)).map_err(err("Could not create a folder"))?;
                } else if kind.is_file() {
                    write_file(&out.join(rel), &mut entry)?;
                } else {
                    // Links could point anywhere: no tool of the kit has them
                    return Err(format!("The {} archive has a link ({}), which is not unpacked.", tool.name, name.display()));
                }
            }
        }
        None => return Err(format!("{} is neither a .zip nor a .tar.gz.", tool.url)),
    }
    Ok(())
}

fn install_one(tool: &Tool, root: &Path, cancel: &Cancel, log: &mut dyn FnMut(&str)) -> Result<()> {
    let file = tool.url.rsplit('/').next().unwrap_or("download").replace("%2B", "+");
    let download = Fetch {
        name: format!("{} {}", tool.name, tool.version),
        url: tool.url.clone(),
        sha256: tool.sha256.clone(),
        to: format!("downloads/{file}"),
    };
    fetch(&download, root, cancel, log)?;
    let archive = root.join(&download.to);
    let dir = tool.dir(root);
    // Unpacked beside it and moved into place whole: a cancelled or failed
    // unpack never leaves something that looks like the tool
    let tmp = root.join(format!("{}-{}.tmp", tool.name, tool.version));
    for old in [&tmp, &dir] {
        if old.exists() {
            fs::remove_dir_all(old).map_err(|e| format!("Could not remove {}: {e}", old.display()))?;
        }
    }
    log(&format!("Unpacking {} {}", tool.name, tool.version));
    let done = unpack(tool, &archive, &tmp, cancel)
        .and_then(|()| fs::write(tmp.join(MARKER), &tool.sha256).map_err(err("Could not write a file")))
        .and_then(|()| fs::rename(&tmp, &dir).map_err(|e| format!("Could not move {} into place: {e}", tool.name)));
    if let Err(e) = done {
        let _ = fs::remove_dir_all(&tmp);
        return Err(e);
    }
    let _ = fs::remove_file(&archive);
    // Other versions of the same tool, left by an older launcher
    let prefix = format!("{}-", tool.name);
    for entry in fs::read_dir(root).into_iter().flatten().flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        let other_version = name.strip_prefix(&prefix).is_some_and(|v| v.starts_with(|c: char| c.is_ascii_digit()));
        if other_version && entry.path() != dir && entry.path().is_dir() {
            let _ = fs::remove_dir_all(entry.path());
        }
    }
    Ok(())
}

/// Downloads and unpacks the tools that are not there yet. What is already
/// installed is not touched, so a second call costs nothing.
pub fn install(tools: &[&Tool], root: &Path, cancel: &Cancel, log: &mut dyn FnMut(&str)) -> Result<()> {
    let missing: Vec<&&Tool> = tools.iter().filter(|t| !t.is_installed(root)).collect();
    if missing.is_empty() {
        return Ok(());
    }
    log(&format!("Getting the build tools into {} (once)", root.display()));
    for tool in missing {
        if cancel.was_asked() {
            return Err("Cancelled.".into());
        }
        install_one(tool, root, cancel, log)?;
    }
    Ok(())
}

/// What a build step may inherit from the launcher's own environment. PATH is
/// not here: it is made new. Neither is anything a compiler, CMake or Python
/// reads to change what they do (CC, CFLAGS, INCLUDE, LIB, CMAKE_*, PYTHON*).
const INHERITED: &[&str] = &[
    "SystemRoot", "SystemDrive", "windir", "ComSpec", "PATHEXT", "TEMP", "TMP", "USERPROFILE", "USERNAME", "APPDATA",
    "LOCALAPPDATA", "ProgramData", "ProgramFiles", "ProgramFiles(x86)", "ProgramW6432", "CommonProgramFiles",
    "HOMEDRIVE", "HOMEPATH", "NUMBER_OF_PROCESSORS", "PROCESSOR_ARCHITECTURE", "OS", "HTTP_PROXY", "HTTPS_PROXY",
    "NO_PROXY", "HOME", "LANG", "TMPDIR",
];

/// The environment of a step built with the kit: the kit's folders and the
/// system's own in PATH, and nothing else from the PC's PATH.
pub fn environment(tools: &[&Tool], root: &Path) -> Vec<(OsString, OsString)> {
    let mut env: Vec<(OsString, OsString)> = std::env::vars_os()
        .filter(|(name, _)| INHERITED.iter().any(|k| name.to_string_lossy().eq_ignore_ascii_case(k)))
        .collect();
    let mut path: Vec<PathBuf> = tools.iter().flat_map(|t| t.path.iter().map(|p| t.dir(root).join(p))).collect();
    if cfg!(windows) {
        let system = PathBuf::from(std::env::var_os("SystemRoot").unwrap_or_else(|| r"C:\Windows".into()));
        path.push(system.join("System32"));
        path.push(system);
    } else {
        path.extend(["/usr/bin", "/bin"].map(PathBuf::from));
    }
    env.push(("PATH".into(), std::env::join_paths(path).unwrap_or_default()));
    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nl-kit-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn tool(name: &str, url: &str) -> Tool {
        Tool {
            name: name.into(),
            version: "1.0".into(),
            about: String::new(),
            license: String::new(),
            url: url.into(),
            sha256: "0".repeat(64),
            strip: 1,
            skip: vec!["docs/".into()],
            path: vec!["bin".into()],
            provides: BTreeMap::new(),
        }
    }

    fn zip_of(entries: &[(&str, &str)]) -> Vec<u8> {
        let mut zip = zip::ZipWriter::new(io::Cursor::new(Vec::new()));
        for (name, body) in entries {
            zip.start_file(*name, zip::write::SimpleFileOptions::default()).unwrap();
            zip.write_all(body.as_bytes()).unwrap();
        }
        zip.finish().unwrap().into_inner()
    }

    fn tar_gz_of(entries: &[(&str, &str)]) -> Vec<u8> {
        let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast()));
        for (name, body) in entries {
            let mut header = tar::Header::new_gnu();
            header.set_size(body.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            tar.append_data(&mut header, name, body.as_bytes()).unwrap();
        }
        tar.into_inner().unwrap().finish().unwrap()
    }

    #[test]
    fn the_kit_of_this_launcher_is_valid() {
        let kit = Kit::builtin();
        for name in ["llvm-mingw", "cmake", "ninja", "python"] {
            assert!(kit.tool(name).is_some(), "{name}");
        }
        let root = Path::new("kit");
        assert!(kit.tool("python").unwrap().program(root, "python").unwrap().ends_with("python-3.12.16/python.exe"));
        assert!(kit.tool("cmake").unwrap().program(root, "cmake").unwrap().ends_with("bin/cmake.exe"));
        let names = |tools: Vec<&Tool>| tools.iter().map(|t| t.name.clone()).collect::<Vec<_>>();
        assert_eq!(names(kit.select(&["python".into(), "llvm-mingw".into()]).unwrap()), ["llvm-mingw", "python"]);
        assert!(kit.select(&["zig".into()]).unwrap_err().contains("Update the launcher"));
    }

    #[test]
    fn refuses_a_bad_kit() {
        let with = |field: &str| {
            let mut t = serde_json::json!({"name": "a", "version": "1", "about": "", "license": "", "url": "https://x/a.zip", "sha256": "0".repeat(64)});
            let (k, v) = field.split_once('=').unwrap();
            t[k] = serde_json::from_str(v).unwrap();
            Kit::parse(&serde_json::json!({ "tools": [t] }).to_string())
        };
        assert!(with("strip=0").is_ok());
        assert!(with("url=\"http://x/a.zip\"").is_err(), "not https");
        assert!(with("url=\"https://x/a.exe\"").is_err(), "not an archive");
        assert!(with("sha256=\"abc\"").is_err());
        assert!(with("name=\"../a\"").is_err());
        assert!(with("path=[\"../bin\"]").is_err());
        assert!(with("provides={\"python\": \"/usr/bin/python\"}").is_err());
        assert!(with("provides={\"shell\": \"sh.exe\"}").is_err());
    }

    #[test]
    fn unpacks_a_zip_without_its_top_folder_or_what_is_skipped() {
        let dir = temp("zip");
        let t = tool("cc", "https://x/cc.zip");
        fs::write(dir.join("a.zip"), zip_of(&[("cc-1.0/bin/cc", "compiler"), ("cc-1.0/docs/a.html", "x"), ("cc-1.0/lib/x/y.a", "lib"), ("cc-1.0/README", "r")])).unwrap();
        unpack(&t, &dir.join("a.zip"), &dir.join("out"), &Cancel::new()).unwrap();
        assert_eq!(fs::read_to_string(dir.join("out/bin/cc")).unwrap(), "compiler");
        assert!(dir.join("out/lib/x/y.a").is_file() && dir.join("out/README").is_file());
        assert!(!dir.join("out/docs").exists() && !dir.join("out/cc-1.0").exists());
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn unpacks_a_tar_gz_and_refuses_links() {
        let dir = temp("tar");
        let t = tool("py", "https://x/py.tar.gz");
        fs::write(dir.join("a.tgz"), tar_gz_of(&[("python/python.exe", "py"), ("python/Lib/os.py", "os"), ("python/docs/x", "x")])).unwrap();
        unpack(&t, &dir.join("a.tgz"), &dir.join("out"), &Cancel::new()).unwrap();
        assert_eq!(fs::read_to_string(dir.join("out/Lib/os.py")).unwrap(), "os");
        assert!(dir.join("out/python.exe").is_file() && !dir.join("out/docs").exists());

        let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast()));
        let mut header = tar::Header::new_gnu();
        header.set_entry_type(tar::EntryType::Symlink);
        header.set_size(0);
        tar.append_link(&mut header, "python/evil", "/etc/passwd").unwrap();
        fs::write(dir.join("link.tgz"), tar.into_inner().unwrap().finish().unwrap()).unwrap();
        assert!(unpack(&t, &dir.join("link.tgz"), &dir.join("out2"), &Cancel::new()).unwrap_err().contains("link"));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn nothing_lands_outside_the_tool_folder() {
        let t = tool("cc", "https://x/cc.zip");
        assert!(place(&t, Path::new("cc-1.0/../../evil")).is_err());
        assert!(place(&t, Path::new("/abs/evil")).is_err());
        assert_eq!(place(&t, Path::new("cc-1.0")).unwrap(), None, "the top folder itself");
        assert_eq!(place(&t, Path::new("cc-1.0/docs")).unwrap(), None);
        assert_eq!(place(&t, Path::new("cc-1.0/docs2/a")).unwrap(), Some(PathBuf::from("docs2/a")));
    }

    #[test]
    fn installs_once_and_replaces_older_versions() {
        let root = temp("install");
        let body = zip_of(&[("cc-1.0/bin/cc", "compiler")]);
        let mut t = tool("cc", "https://x/cc.zip");
        t.sha256 = crate::build::sha256_of_bytes(&body);
        // The download is already there with the right SHA256, as after a
        // download by hand: nothing is asked of the network
        fs::create_dir_all(root.join("downloads")).unwrap();
        fs::write(root.join("downloads/cc.zip"), &body).unwrap();
        fs::create_dir_all(root.join("cc-0.9/bin")).unwrap();
        fs::create_dir_all(root.join("cc-tools-1.0")).unwrap();
        fs::create_dir_all(root.join("cc-1.0")).unwrap(); // cut off half way: no marker
        assert!(!t.is_installed(&root));
        let mut seen = Vec::new();
        install(&[&t], &root, &Cancel::new(), &mut |l| seen.push(l.to_string())).unwrap();
        assert!(t.is_installed(&root));
        assert_eq!(fs::read_to_string(root.join("cc-1.0/bin/cc")).unwrap(), "compiler");
        assert!(!root.join("cc-0.9").exists(), "the older version goes");
        assert!(root.join("cc-tools-1.0").exists(), "another tool stays");
        assert!(!root.join("downloads/cc.zip").exists() && !root.join("cc-1.0.tmp").exists());
        assert!(seen[0].starts_with("Getting the build tools"));

        // Again: nothing to do, not even a line of log
        seen.clear();
        install(&[&t], &root, &Cancel::new(), &mut |l| seen.push(l.to_string())).unwrap();
        assert!(seen.is_empty());
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn the_environment_has_the_kit_first_and_not_the_pc_path() {
        let mut t = tool("cc", "https://x/cc.zip");
        t.path = vec!["bin".into(), "".into()];
        let env = environment(&[&t], Path::new("/kit"));
        let path = env.iter().find(|(k, _)| k == "PATH").unwrap().1.clone();
        let dirs: Vec<PathBuf> = std::env::split_paths(&path).collect();
        assert_eq!(dirs[0], Path::new("/kit/cc-1.0/bin"));
        assert_eq!(dirs[1], Path::new("/kit/cc-1.0/"));
        assert_eq!(dirs.len(), 4, "the kit and the system's two folders: {dirs:?}");
        assert!(env.iter().all(|(k, _)| !k.to_string_lossy().starts_with("CARGO")), "nothing else is inherited");
    }
}
