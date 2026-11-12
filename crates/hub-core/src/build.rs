//! Builds a project from its `hub.toml`: the downloads, then the steps. This
//! is what each project's `build.bat` and `get_deps.bat` do, done by the
//! launcher.
//!
//! The tools are either the launcher's own build kit (`kit`), downloaded
//! into the launcher's folder, or looked for on the PC (`requirements`). One
//! that should be on the PC and is not stops the build, which says which:
//! installing those is still the project's `install.bat`.

use crate::kit::{self, Kit};
use crate::manifest::{expand, Fetch, Manifest, Step};
use crate::requirements::{at_least, find_cmake, find_python, find_visual_studio};
use crate::runner::{self, Cancel};
use crate::{agent, err, http_error, Result};
use sha2::{Digest, Sha256};
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

/// What the steps of one manifest run with.
#[derive(Debug, Default)]
pub struct Tools {
    pub python: Option<PathBuf>,
    pub cmake: Option<PathBuf>,
    /// The whole environment after Visual Studio's `vcvars64.bat`.
    pub msvc_env: Option<Vec<(OsString, OsString)>>,
    /// With tools from the build kit: the environment every step runs in.
    pub kit_env: Option<Vec<(OsString, OsString)>>,
    /// The user's own file, when one is chosen.
    pub input: Option<PathBuf>,
}

impl Tools {
    /// Looks for what the manifest asks for: in the build kit under
    /// `kit_root` for the tools it names from it, on this PC for the rest. An
    /// error names everything that is missing, not only the first.
    pub fn find(manifest: &Manifest, input: Option<&Path>, kit_root: &Path) -> Result<Tools> {
        let r = &manifest.requires;
        let mut missing = Vec::new();
        let mut tools = Tools { input: input.filter(|p| p.is_file()).map(Path::to_path_buf), ..Default::default() };
        let kit = Kit::builtin();
        let from_kit = kit.select(&r.kit)?;
        for t in &from_kit {
            if !t.is_installed(kit_root) {
                missing.push(format!("{} {} of the build kit", t.name, t.version));
            }
        }
        if !from_kit.is_empty() {
            tools.kit_env = Some(kit::environment(&from_kit, kit_root));
        }
        // The kit's own python or cmake, with the version the project asks for
        let in_kit = |what: &str, min: Option<&str>| -> Result<Option<PathBuf>> {
            let Some(tool) = from_kit.iter().find(|t| t.provides.contains_key(what)) else { return Ok(None) };
            match min {
                Some(min) if !at_least(&tool.version, min) => Err(format!(
                    "This project needs {} {min} or later and the build kit of this launcher has {}. Update the launcher.",
                    tool.name, tool.version
                )),
                _ => Ok(tool.program(kit_root, what)),
            }
        };
        tools.python = in_kit("python", r.python.as_deref())?;
        if let (None, Some(min)) = (&tools.python, &r.python) {
            match find_python(min) {
                Some(p) => tools.python = Some(PathBuf::from(p.path)),
                None => missing.push(format!("Python {min} or later")),
            }
        }
        let vs = if r.msvc { find_visual_studio() } else { None };
        if r.msvc {
            match &vs {
                Some(dir) => tools.msvc_env = Some(msvc_environment(dir)?),
                None => missing.push("Visual Studio 2022 with the x64 C++ tools and CMake".to_string()),
            }
        }
        tools.cmake = in_kit("cmake", r.cmake.as_deref())?;
        if tools.cmake.is_none() && (r.cmake.is_some() || r.msvc) {
            match find_cmake(r.cmake.as_deref(), vs.as_deref()) {
                Some(path) => tools.cmake = Some(path),
                None => missing.push(format!("CMake {} or later", r.cmake.as_deref().unwrap_or("3"))),
            }
        }
        if missing.is_empty() {
            Ok(tools)
        } else {
            Err(format!("Not found on this PC: {}.", missing.join("; ")))
        }
    }

    fn value(&self, name: &str) -> Option<String> {
        let path = match name {
            "python" => self.python.clone(),
            "cmake" => self.cmake.clone(),
            // Next to the cmake in use, so both are the same version
            "ctest" => self.cmake.as_ref().map(|c| {
                let name = if c.extension().is_some() { "ctest.exe" } else { "ctest" };
                if c.parent().is_some_and(|p| !p.as_os_str().is_empty()) { c.with_file_name(name) } else { PathBuf::from(name) }
            }),
            "input" => self.input.clone(),
            _ => None,
        };
        path.map(|p| p.to_string_lossy().into_owned())
    }
}

/// The environment `vcvars64.bat` leaves: a small script calls it and prints
/// what is set, and every `run` step with `msvc_env` gets exactly that.
#[cfg(windows)]
fn msvc_environment(vs: &Path) -> Result<Vec<(OsString, OsString)>> {
    let vcvars = vs.join(r"VC\Auxiliary\Build\vcvars64.bat");
    let script = std::env::temp_dir().join(format!("nicolauncher-vcvars-{}.bat", std::process::id()));
    // UTF-8 output, so a user folder with accents survives
    let text = format!("@echo off\r\nchcp 65001 >nul\r\ncall \"{}\" >nul 2>&1\r\nif errorlevel 1 exit /b 1\r\necho ---ENV---\r\nset\r\n", vcvars.display());
    fs::write(&script, text).map_err(err("Could not write a temporary file"))?;
    let mut cmd = Command::new(&script);
    cmd.stdin(std::process::Stdio::null());
    runner::no_window(&mut cmd);
    let out = cmd.output();
    let _ = fs::remove_file(&script);
    let out = out.map_err(err("Could not start Visual Studio's vcvars64.bat"))?;
    let text = String::from_utf8_lossy(&out.stdout);
    let env = text.split_once("---ENV---").map(|(_, env)| parse_environment(env)).unwrap_or_default();
    if !out.status.success() || !env.iter().any(|(k, _)| k.eq_ignore_ascii_case("INCLUDE")) {
        return Err(format!(
            "Visual Studio's setup ({}) failed. Check that the \"Windows 11 SDK\" component is installed.",
            vcvars.display()
        ));
    }
    Ok(env)
}

#[cfg(not(windows))]
fn msvc_environment(_vs: &Path) -> Result<Vec<(OsString, OsString)>> {
    Err("Visual Studio is only on Windows.".into())
}

/// The output of `set`: one `NAME=value` per line. cmd's own hidden variables
/// (`=C:=C:\...`) start with `=` and are left out.
#[cfg_attr(not(windows), allow(dead_code))]
fn parse_environment(text: &str) -> Vec<(OsString, OsString)> {
    text.lines()
        .filter_map(|line| line.trim_end_matches('\r').split_once('='))
        .filter(|(name, _)| !name.is_empty())
        .map(|(name, value)| (name.into(), value.into()))
        .collect()
}

pub fn sha256_of(path: &Path) -> Result<String> {
    let mut file = File::open(path).map_err(err("Could not read a file"))?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 256 * 1024];
    loop {
        let n = file.read(&mut buf).map_err(err("Could not read a file"))?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

#[cfg(test)]
pub(crate) fn sha256_of_bytes(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect()
}

fn download(agent: &ureq::Agent, url: &str, to: &Path, cancel: &Cancel, log: &mut dyn FnMut(&str)) -> Result<()> {
    const MB: u64 = 1024 * 1024;
    let resp = agent.get(url).call().map_err(|e| http_error(url, e))?;
    let size = resp.header("Content-Length").and_then(|v| v.parse::<u64>().ok());
    let mut reader = resp.into_reader();
    let mut file = File::create(to).map_err(err("Could not write the download"))?;
    let mut buf = vec![0u8; 256 * 1024];
    // A large download says how far it is, every 32 MB
    let (mut done, mut next_report) = (0u64, 32 * MB);
    loop {
        if cancel.was_asked() {
            return Err("Cancelled.".into());
        }
        let n = reader.read(&mut buf).map_err(err("The download was cut off"))?;
        if n == 0 {
            return Ok(());
        }
        file.write_all(&buf[..n]).map_err(err("Could not write the download"))?;
        done += n as u64;
        if done >= next_report {
            match size {
                Some(size) => log(&format!("  {} of {} MB", done / MB, size / MB)),
                None => log(&format!("  {} MB", done / MB)),
            }
            next_report += 32 * MB;
        }
    }
}

/// One `[[fetch]]`: the file is at `to` with the right SHA256 when this
/// returns Ok. A good copy already there is kept; a bad one is replaced. The
/// download goes to a `.part` file first, so one that is cut off or is not
/// the expected file (a proxy or antivirus page) never takes the place.
pub fn fetch(f: &Fetch, dir: &Path, cancel: &Cancel, log: &mut dyn FnMut(&str)) -> Result<()> {
    fetch_with(&agent(60), f, dir, cancel, log)
}

fn fetch_with(agent: &ureq::Agent, f: &Fetch, dir: &Path, cancel: &Cancel, log: &mut dyn FnMut(&str)) -> Result<()> {
    let to = dir.join(&f.to);
    let wanted = f.sha256.to_lowercase();
    if to.is_file() {
        if sha256_of(&to)? == wanted {
            return Ok(());
        }
        log(&format!("{} is incomplete or damaged: downloading it again", f.to));
    }
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).map_err(err("Could not create a folder"))?;
    }
    let part = to.with_file_name(format!("{}.part", to.file_name().unwrap_or_default().to_string_lossy()));
    log(&format!("Downloading {}", f.name));
    let mut last = String::new();
    for attempt in 1..=3 {
        if attempt > 1 {
            log(&format!("  {last} Trying again ({attempt}/3)"));
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
        let got = download(agent, &f.url, &part, cancel, log).and_then(|()| sha256_of(&part));
        if cancel.was_asked() {
            let _ = fs::remove_file(&part);
            return Err("Cancelled.".into());
        }
        match got {
            Ok(sha) if sha == wanted => {
                return fs::rename(&part, &to).map_err(err("Could not move the download into place"));
            }
            Ok(_) => {
                // Not a connection problem: the same again would give the same
                let _ = fs::remove_file(&part);
                return Err(format!(
                    "{} was downloaded but its SHA256 is not the expected one (a proxy or antivirus page instead of the file?). Nothing was kept.",
                    f.name
                ));
            }
            Err(e) => last = e,
        }
    }
    let _ = fs::remove_file(&part);
    Err(format!(
        "{last} You can also save {} with a browser as {} and build again.",
        f.url,
        to.display()
    ))
}

fn all_exist(dir: &Path, files: &[String]) -> bool {
    !files.is_empty() && files.iter().all(|f| dir.join(f).exists())
}

fn run_step(step: &Step, dir: &Path, tools: &Tools, cancel: &Cancel, log: &mut dyn FnMut(&str)) -> Result<()> {
    if let Some(copy) = &step.copy {
        let (from, to) = (dir.join(&copy.from), dir.join(&copy.to));
        if !from.is_file() {
            return Err(format!("{} is not there.", copy.from));
        }
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).map_err(err("Could not create a folder"))?;
        }
        return fs::copy(&from, &to).map(|_| ()).map_err(|e| format!("Could not copy {} to {}: {e}", copy.from, copy.to));
    }
    let value = |name: &str| tools.value(name);
    let mut args = Vec::new();
    for arg in &step.run {
        args.push(expand(arg, &value).map_err(|e| {
            if arg.contains("{input}") && tools.input.is_none() {
                "This step needs your own file: choose it first.".to_string()
            } else {
                e
            }
        })?);
    }
    // A program given as a path inside the project is started from there,
    // whatever folder the launcher itself runs in
    let program = Path::new(&args[0]);
    let in_project = program.is_relative() && program.components().count() > 1;
    let mut cmd = Command::new(if in_project { dir.join(program) } else { program.to_path_buf() });
    cmd.args(&args[1..]).current_dir(dir);
    if step.msvc_env {
        let env = tools.msvc_env.as_ref().ok_or("The Visual Studio environment is not set up.")?;
        cmd.env_clear().envs(env.iter().map(|(k, v)| (k, v)));
    } else if let Some(env) = &tools.kit_env {
        cmd.env_clear().envs(env.iter().map(|(k, v)| (k, v)));
    }
    let code = runner::run(cmd, cancel, log).map_err(|e| format!("Could not start {}: {e}", args[0]))?;
    if code == 0 { Ok(()) } else { Err(format!("It ended with an error (code {code}).")) }
}

/// Downloads the tools of the build kit the manifest names that are not
/// under `kit_root` yet. Nothing to do for a manifest that names none.
pub fn install_kit(manifest: &Manifest, kit_root: &Path, cancel: &Cancel, log: &mut dyn FnMut(&str)) -> Result<()> {
    let kit = Kit::builtin();
    kit::install(&kit.select(&manifest.requires.kit)?, kit_root, cancel, log)
}

/// The downloads, then the steps, in the project's folder. Each one is
/// announced as `[n/total] name`. Returns the warnings: the optional steps
/// that failed.
pub fn run(manifest: &Manifest, dir: &Path, tools: &Tools, cancel: &Cancel, log: &mut dyn FnMut(&str)) -> Result<Vec<String>> {
    let downloads = usize::from(!manifest.fetches.is_empty());
    let total = manifest.steps.len() + downloads;
    if downloads == 1 {
        log(&format!("[1/{total}] Getting the libraries"));
        for f in &manifest.fetches {
            fetch(f, dir, cancel, log)?;
        }
    }
    let mut warnings = Vec::new();
    for (i, step) in manifest.steps.iter().enumerate() {
        if cancel.was_asked() {
            return Err("Cancelled.".into());
        }
        log(&format!("[{}/{total}] {}", i + 1 + downloads, step.name));
        if all_exist(dir, &step.skip_if_exists) {
            log("  Already done.");
            continue;
        }
        match run_step(step, dir, tools, cancel, log) {
            Ok(()) => {}
            Err(_) if cancel.was_asked() => return Err("Cancelled.".into()),
            Err(e) if step.optional => {
                log(&format!("  Warning: {e}"));
                warnings.push(format!("{}: {e}", step.name));
            }
            Err(e) => return Err(format!("{}: {e}", step.name)),
        }
    }
    if !dir.join(&manifest.exe).is_file() {
        return Err(format!("The steps ended but {} is not there.", manifest.exe));
    }
    Ok(warnings)
}

/// The project's manifest, when its folder has one.
pub fn manifest_in(dir: &Path) -> Result<Option<Manifest>> {
    let path = dir.join(crate::manifest::FILE);
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(err("Could not read hub.toml"))?;
    Manifest::parse(&text).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader};
    use std::net::TcpListener;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nl-build-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Serves `bodies` in order, one per connection, on a local port.
    fn serve(bodies: Vec<Vec<u8>>) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://127.0.0.1:{}/lib.zip", listener.local_addr().unwrap().port());
        std::thread::spawn(move || {
            for body in bodies {
                let Ok((mut stream, _)) = listener.accept() else { return };
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut line = String::new();
                while reader.read_line(&mut line).is_ok_and(|n| n > 2) {
                    line.clear();
                }
                let head = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
                let _ = stream.write_all(head.as_bytes());
                let _ = stream.write_all(&body);
            }
        });
        url
    }

    /// Straight to the local test server, whatever proxy the machine has set.
    fn direct() -> ureq::Agent {
        ureq::AgentBuilder::new().try_proxy_from_env(false).build()
    }

    fn sha(bytes: &[u8]) -> String {
        sha256_of_bytes(bytes)
    }

    fn lines() -> (std::rc::Rc<std::cell::RefCell<Vec<String>>>, impl FnMut(&str)) {
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = seen.clone();
        (seen, move |l: &str| sink.borrow_mut().push(l.to_string()))
    }

    #[test]
    fn downloads_checks_and_keeps_a_good_copy() {
        let dir = temp("fetch");
        let body = b"the library".to_vec();
        // One body only: a second request would hang the test, so the second
        // fetch proves the good copy is not downloaded again
        let f = Fetch { name: "lib".into(), url: serve(vec![body.clone()]), sha256: sha(&body).to_uppercase(), to: "third_party/downloads/lib.zip".into() };
        let (_, mut log) = lines();
        fetch_with(&direct(), &f, &dir, &Cancel::new(), &mut log).unwrap();
        assert_eq!(fs::read(dir.join(&f.to)).unwrap(), body);
        assert!(!dir.join("third_party/downloads/lib.zip.part").exists());
        fetch_with(&direct(), &f, &dir, &Cancel::new(), &mut log).unwrap();
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn refuses_a_download_that_is_not_the_expected_file() {
        let dir = temp("badsha");
        let f = Fetch { name: "lib".into(), url: serve(vec![b"<html>blocked by proxy</html>".to_vec()]), sha256: sha(b"the library"), to: "lib.zip".into() };
        let (_, mut log) = lines();
        let e = fetch_with(&direct(), &f, &dir, &Cancel::new(), &mut log).unwrap_err();
        assert!(e.contains("SHA256"), "{e}");
        assert!(fs::read_dir(&dir).unwrap().next().is_none(), "nothing kept");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn replaces_a_damaged_copy() {
        let dir = temp("damaged");
        let body = b"the library".to_vec();
        fs::write(dir.join("lib.zip"), b"half of it").unwrap();
        let f = Fetch { name: "lib".into(), url: serve(vec![body.clone()]), sha256: sha(&body), to: "lib.zip".into() };
        let (seen, mut log) = lines();
        fetch_with(&direct(), &f, &dir, &Cancel::new(), &mut log).unwrap();
        assert_eq!(fs::read(dir.join("lib.zip")).unwrap(), body);
        assert!(seen.borrow()[0].contains("damaged"));
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn reads_the_output_of_set() {
        let env = parse_environment("=C:=C:\\Users\r\nINCLUDE=C:\\VS\\include;C:\\Kits\\um\r\nA=b=c\r\n\r\n");
        assert_eq!(env.len(), 2);
        assert_eq!(env[0], ("INCLUDE".into(), "C:\\VS\\include;C:\\Kits\\um".into()));
        assert_eq!(env[1], ("A".into(), "b=c".into()));
    }

    #[test]
    fn ctest_is_the_one_next_to_cmake() {
        let t = |cmake: &str| Tools { cmake: Some(PathBuf::from(cmake)), ..Default::default() }.value("ctest").unwrap();
        assert_eq!(t("cmake"), "ctest");
        assert_eq!(PathBuf::from(t("/opt/cmake/bin/cmake")), PathBuf::from("/opt/cmake/bin/ctest"));
        assert!(t("tools/cmake.exe").ends_with("ctest.exe"));
    }

    fn kit_manifest(requires: &str) -> Manifest {
        Manifest::parse(&format!("schema = 1\nexe = \"a.exe\"\n[requires]\n{requires}")).unwrap()
    }

    /// As after `kit::install`: the tool's folder with its marker.
    fn pretend_installed(root: &Path, name: &str) {
        let kit = Kit::builtin();
        let tool = kit.tool(name).unwrap();
        fs::create_dir_all(tool.dir(root)).unwrap();
        fs::write(tool.dir(root).join(".nicolauncher-kit"), &tool.sha256).unwrap();
    }

    #[test]
    fn takes_python_and_cmake_from_the_kit() {
        let root = temp("kit-tools");
        let m = kit_manifest("kit = [\"python\", \"cmake\", \"ninja\"]\npython = \"3.10\"\ncmake = \"3.27\"");
        let e = Tools::find(&m, None, &root).unwrap_err();
        assert!(e.contains("python") && e.contains("cmake") && e.contains("ninja") && e.contains("build kit"), "{e}");
        for name in ["python", "cmake", "ninja"] {
            pretend_installed(&root, name);
        }
        let tools = Tools::find(&m, None, &root).unwrap();
        assert!(tools.python.as_ref().unwrap().starts_with(&root) && tools.python.as_ref().unwrap().ends_with("python.exe"));
        assert!(tools.cmake.as_ref().unwrap().starts_with(&root));
        assert!(tools.value("ctest").unwrap().ends_with("ctest.exe"), "the kit's ctest too");
        let env = tools.kit_env.unwrap();
        let path = env.iter().find(|(k, _)| k == "PATH").unwrap().1.to_string_lossy().into_owned();
        assert!(path.contains("ninja-") && path.contains("python-") && path.contains("cmake-"), "{path}");

        // A project that asks for more than this launcher's kit has
        let e = Tools::find(&kit_manifest("kit = [\"python\"]\npython = \"99.0\""), None, &root).unwrap_err();
        assert!(e.contains("Update the launcher"), "{e}");
        let e = Tools::find(&kit_manifest("kit = [\"zig\"]"), None, &root).unwrap_err();
        assert!(e.contains("Update the launcher"), "{e}");
        fs::remove_dir_all(&root).unwrap();
    }

    /// The whole kit for real: downloaded, unpacked, and a C++ program built
    /// with its clang, CMake, Ninja and Python and nothing else from the PC.
    /// About 250 MB of downloads, so only when asked for (the Windows CI does).
    #[cfg(windows)]
    #[test]
    #[ignore = "downloads the build kit"]
    fn builds_with_the_real_kit() {
        let dir = temp("real-kit");
        let root = dir.join("kit");
        fs::write(dir.join("hello.cpp"), "#include <windows.h>\n#include <iostream>\n#include <string>\nint main() { std::string s = \"hello from the kit\"; std::cout << s << std::endl; return GetTickCount() ? 0 : 0; }\n").unwrap();
        fs::write(dir.join("CMakeLists.txt"), "cmake_minimum_required(VERSION 3.27)\nproject(hello CXX)\nset(CMAKE_CXX_STANDARD 20)\nadd_link_options(-static)\nadd_executable(hello hello.cpp)\nenable_testing()\nadd_test(NAME hello COMMAND hello)\n").unwrap();
        let m = Manifest::parse(
            r#"
schema = 1
exe = "hello.exe"
[requires]
kit = ["llvm-mingw", "cmake", "ninja", "python"]
python = "3.10"
cmake = "3.27"
[[step]]
name = "Python"
run = ["{python}", "-c", "import sys, zlib, zipfile, struct; open('made.txt', 'w').write(sys.executable)"]
[[step]]
name = "Configure"
run = ["{cmake}", "-S", ".", "-B", "build", "-G", "Ninja", "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_CXX_COMPILER=clang++"]
[[step]]
name = "Compile"
run = ["{cmake}", "--build", "build"]
[[step]]
name = "Test"
run = ["{ctest}", "--test-dir", "build", "--output-on-failure"]
[[step]]
name = "Copy"
copy = { from = "build/hello.exe", to = "hello.exe" }
[[step]]
name = "Run"
run = ["./hello.exe"]
"#,
        )
        .unwrap();
        let (seen, mut log) = lines();
        install_kit(&m, &root, &Cancel::new(), &mut log).unwrap();
        let tools = Tools::find(&m, None, &root).unwrap();
        let result = run(&m, &dir, &tools, &Cancel::new(), &mut log);
        assert!(result.is_ok(), "{result:?}\n{:#?}", seen.borrow());
        let python = fs::read_to_string(dir.join("made.txt")).unwrap();
        assert!(Path::new(&python).starts_with(&root), "the kit's Python ran: {python}");
        let cache = fs::read_to_string(dir.join("build/CMakeCache.txt")).unwrap();
        let compiler = cache.lines().find(|l| l.starts_with("CMAKE_CXX_COMPILER:")).unwrap().replace('/', "\\").to_lowercase();
        // By the end of the path: Windows may spell the start of the temporary
        // folder in its short form (RUNNER~1) in one place and whole in another
        let kit_clang = format!("{}\\kit\\llvm-mingw-", dir.file_name().unwrap().to_string_lossy().to_lowercase());
        assert!(compiler.contains(&kit_clang) && compiler.ends_with("\\bin\\clang++.exe"), "the kit's compiler was used: {compiler}");
        assert!(seen.borrow().iter().any(|l| l == "hello from the kit"));
        // What was left out of the compiler's archive is not there
        let llvm = Kit::builtin().tool("llvm-mingw").unwrap().dir(&root);
        assert!(llvm.join("x86_64-w64-mingw32").is_dir() && !llvm.join("i686-w64-mingw32").exists());
        // A second time nothing is downloaded
        let (seen, mut log) = lines();
        install_kit(&m, &root, &Cancel::new(), &mut log).unwrap();
        assert!(seen.borrow().is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[cfg(unix)]
    mod steps {
        use super::*;

        fn manifest(steps: &str) -> Manifest {
            Manifest::parse(&format!("schema = 1\nexe = \"game.bin\"\n[input]\nlabel = \"rom.zip\"\n{steps}")).unwrap()
        }

        const BUILD: &str = r#"
[[step]]
name = "Extract"
run = ["sh", "-c", "mkdir -p data && cp \"$1\" data/rom.bin && echo extracted", "sh", "{input}"]
skip_if_exists = ["data/rom.bin"]
[[step]]
name = "Compile"
run = ["tools/compile.sh", "build/game.bin"]
[[step]]
name = "Copy"
copy = { from = "build/game.bin", to = "game.bin" }
[[step]]
name = "Copy extras"
copy = { from = "build/extras.dll", to = "extras.dll" }
optional = true
[[step]]
name = "Tests"
run = ["sh", "-c", "echo one test failed; exit 3"]
optional = true
"#;

        fn project(name: &str) -> PathBuf {
            use std::os::unix::fs::PermissionsExt;
            let dir = temp(name);
            fs::create_dir_all(dir.join("tools")).unwrap();
            let script = dir.join("tools/compile.sh");
            fs::write(&script, "#!/bin/sh\nmkdir -p build && cat data/rom.bin > \"$1\" && echo compiled in $(basename \"$PWD\")\n").unwrap();
            fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
            dir
        }

        #[test]
        fn builds_in_order_and_reports_optional_failures() {
            let dir = project("steps");
            let rom = dir.join("my rom.zip");
            fs::write(&rom, "rom bytes").unwrap();
            let tools = Tools { input: Some(rom), ..Default::default() };
            let (seen, mut log) = lines();
            let warnings = run(&manifest(BUILD), &dir, &tools, &Cancel::new(), &mut log).unwrap();
            assert_eq!(fs::read_to_string(dir.join("game.bin")).unwrap(), "rom bytes");
            assert_eq!(warnings.len(), 2, "{warnings:?}");
            assert!(warnings[1].starts_with("Tests: ") && warnings[1].contains("code 3"));
            let seen = seen.borrow();
            assert_eq!(seen[0], "[1/5] Extract");
            assert!(seen.contains(&"extracted".to_string()) && seen.contains(&"[5/5] Tests".to_string()));
            assert!(seen.iter().any(|l| l.starts_with("compiled in nl-build-steps")), "runs in the project's folder");
            drop(seen);

            // Again without the file: the step that used it is already done
            let (seen, mut log) = lines();
            run(&manifest(BUILD), &dir, &Tools::default(), &Cancel::new(), &mut log).unwrap();
            assert_eq!(seen.borrow()[1], "  Already done.");
            fs::remove_dir_all(&dir).unwrap();
        }

        #[test]
        fn a_kit_step_sees_only_the_kit_environment() {
            let dir = temp("kit-env");
            std::env::set_var("NL_TEST_LEAK", "from the PC");
            let env = vec![("PATH".into(), "/usr/bin:/bin".into()), ("NL_KIT".into(), "yes".into())];
            let tools = Tools { kit_env: Some(env), ..Default::default() };
            let m = manifest("[[step]]\nname = \"Env\"\nrun = [\"sh\", \"-c\", \"echo kit=$NL_KIT leak=$NL_TEST_LEAK; touch game.bin\"]");
            let (seen, mut log) = lines();
            run(&m, &dir, &tools, &Cancel::new(), &mut log).unwrap();
            assert!(seen.borrow().contains(&"kit=yes leak=".to_string()), "{:?}", seen.borrow());
            fs::remove_dir_all(&dir).unwrap();
        }

        #[test]
        fn stops_at_the_first_step_that_fails() {
            let dir = project("fails");
            let (seen, mut log) = lines();
            let e = run(&manifest(BUILD), &dir, &Tools::default(), &Cancel::new(), &mut log).unwrap_err();
            assert_eq!(e, "Extract: This step needs your own file: choose it first.");
            assert_eq!(seen.borrow().len(), 1);

            let m = manifest("[[step]]\nname = \"Compile\"\nrun = [\"sh\", \"-c\", \"exit 2\"]\n[[step]]\nname = \"Never\"\nrun = [\"sh\", \"-c\", \"touch never\"]");
            let (_, mut log) = lines();
            assert_eq!(run(&m, &dir, &Tools::default(), &Cancel::new(), &mut log).unwrap_err(), "Compile: It ended with an error (code 2).");
            assert!(!dir.join("never").exists());

            // Steps that all pass but leave no program are not a build
            let m = manifest("[[step]]\nname = \"Nothing\"\nrun = [\"sh\", \"-c\", \"true\"]");
            let (_, mut log) = lines();
            assert!(run(&m, &dir, &Tools::default(), &Cancel::new(), &mut log).unwrap_err().contains("game.bin"));
            fs::remove_dir_all(&dir).unwrap();
        }

        #[test]
        fn reads_the_manifest_of_a_folder() {
            let dir = temp("manifest");
            assert_eq!(manifest_in(&dir).unwrap(), None);
            fs::write(dir.join("hub.toml"), "schema = 1\nexe = \"a.exe\"").unwrap();
            assert_eq!(manifest_in(&dir).unwrap().unwrap().exe, "a.exe");
            fs::write(dir.join("hub.toml"), "schema = 1").unwrap();
            assert!(manifest_in(&dir).is_err());
            fs::remove_dir_all(&dir).unwrap();
        }
    }

    /// On the Windows CI runner: the real tools, the real vcvars64.bat, and a
    /// step that compiles with them.
    #[cfg(windows)]
    #[test]
    fn compiles_with_the_real_visual_studio() {
        let dir = temp("msvc");
        fs::write(dir.join("hello.c"), "#include <windows.h>\n#include <stdio.h>\nint main(void) { printf(\"hello\"); return 0; }\n").unwrap();
        let m = Manifest::parse(
            r#"
schema = 1
exe = "hello.exe"
[requires]
python = "3.10"
cmake = "3.20"
msvc = true
[[step]]
name = "Python"
run = ["{python}", "-c", "open('made.txt', 'w').write('x')"]
[[step]]
name = "CMake"
run = ["{cmake}", "--version"]
[[step]]
name = "Compile"
run = ["cl", "/nologo", "hello.c"]
msvc_env = true
"#,
        )
        .unwrap();
        let tools = Tools::find(&m, None, &dir).unwrap();
        let (seen, mut log) = lines();
        let result = run(&m, &dir, &tools, &Cancel::new(), &mut log);
        assert!(result.is_ok(), "{result:?}\n{:#?}", seen.borrow());
        assert!(dir.join("made.txt").is_file() && dir.join("hello.exe").is_file());
        assert!(seen.borrow().iter().any(|l| l.starts_with("cmake version")));
        fs::remove_dir_all(&dir).unwrap();
    }
}
