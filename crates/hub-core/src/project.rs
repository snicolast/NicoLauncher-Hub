//! What can be done with one project on this PC: where it stands, and the
//! commands for its scripts.

use crate::news;
use crate::registry::{is_plain_relative, Project};
use crate::Result;
use serde::Serialize;
use std::path::Path;
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum State {
    /// Its source is not on this PC yet.
    NotInstalled,
    /// It needs the user's own file (a ROM) and none is chosen, or it is gone.
    NeedsInput,
    /// The source is here and it has not been built.
    NeedsBuild,
    /// Built: it can be started.
    Ready,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    Install,
    Build,
    Update,
}

pub fn is_installed(project: &Project, dir: &Path) -> bool {
    let s = &project.scripts;
    [&s.install, &s.build, &project.exe]
        .into_iter()
        .flatten()
        .any(|rel| dir.join(rel).is_file())
        || dir.join(crate::manifest::FILE).is_file()
}

pub fn is_built(project: &Project, dir: &Path) -> bool {
    project.exe.as_ref().is_some_and(|exe| dir.join(exe).is_file())
}

pub fn state(project: &Project, dir: Option<&Path>, input: Option<&Path>) -> State {
    let Some(dir) = dir.filter(|d| is_installed(project, d)) else {
        return State::NotInstalled;
    };
    if is_built(project, dir) {
        return State::Ready;
    }
    if project.input.is_some() && !input.is_some_and(|p| p.is_file()) {
        return State::NeedsInput;
    }
    State::NeedsBuild
}

/// The command for one of the project's own scripts, run in its folder. The
/// user's file, when there is one, is its only argument (as the scripts take
/// it today).
pub fn script_command(
    project: &Project,
    dir: &Path,
    which: Script,
    input: Option<&Path>,
) -> Result<Command> {
    let s = &project.scripts;
    let rel = match which {
        Script::Install => &s.install,
        Script::Build => &s.build,
        Script::Update => &s.update,
    }
    .as_ref()
    .ok_or_else(|| format!("{} has no script for this.", project.name))?;
    if !is_plain_relative(rel) {
        return Err(format!("Bad script path: {rel}"));
    }
    let path = dir.join(rel);
    if !path.is_file() {
        return Err(format!("{} is missing. Reinstall {}.", path.display(), project.name));
    }
    // A .bat is started through cmd.exe by Windows itself, and Rust quotes its
    // arguments for it (or refuses the ones it cannot quote safely)
    let mut cmd = if cfg!(windows) {
        Command::new(&path)
    } else {
        let mut c = Command::new("sh");
        c.arg(&path);
        c
    };
    cmd.current_dir(dir);
    if which != Script::Update {
        if let Some(input) = input.filter(|p| p.is_file()) {
            cmd.arg(input);
        }
    }
    Ok(cmd)
}

/// Starts the built program in its folder and leaves it running.
pub fn launch(project: &Project, dir: &Path) -> Result<()> {
    let exe = project.exe.as_ref().ok_or("This project has nothing to start.")?;
    let path = dir.join(exe);
    if !path.is_file() {
        return Err(format!("{} is not built yet.", project.name));
    }
    Command::new(&path)
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(|_| ())
        .map_err(|e| format!("Could not start {}: {e}", path.display()))
}

/// Puts a shortcut to the built program on the user's desktop, replacing one
/// of the same name. Windows only: Windows' own shell writes the .lnk, and is
/// asked where the desktop is, so it is found wherever the user keeps it
/// (OneDrive moves it). Its icon is `icon` (an .ico the user chose), or else
/// the program's own.
pub fn desktop_shortcut(project: &Project, dir: &Path, icon: Option<&Path>) -> Result<()> {
    let exe = project.exe.as_ref().ok_or("This project has nothing to start.")?;
    let path = dir.join(exe);
    if !path.is_file() {
        return Err(format!("{} is not built yet.", project.name));
    }
    if !cfg!(windows) {
        return Err("Desktop shortcuts are only made on Windows.".into());
    }
    write_shortcut(&shortcut_file_name(&project.name), &path, dir, icon.unwrap_or(&path), false)
        .map(|_| ())
        .map_err(|e| format!("Could not make the shortcut: {e}"))
}

/// Gives the shortcut on the desktop its new icon (see `desktop_shortcut`),
/// when there is one: none is made. Ok(false) when there was none.
pub fn refresh_shortcut(project: &Project, dir: &Path, icon: Option<&Path>) -> Result<bool> {
    let Some(path) = project.exe.as_ref().map(|e| dir.join(e)).filter(|p| p.is_file()) else { return Ok(false) };
    if !cfg!(windows) {
        return Ok(false);
    }
    write_shortcut(&shortcut_file_name(&project.name), &path, dir, icon.unwrap_or(&path), true)
        .map_err(|e| format!("Could not change the shortcut: {e}"))
}

/// The shortcut's file name: the project's name with what Windows does not
/// allow in a file name made spaces.
fn shortcut_file_name(name: &str) -> String {
    let clean: String = name
        .chars()
        .map(|c| if c.is_control() || r#"<>:"/\|?*"#.contains(c) { ' ' } else { c })
        .collect();
    format!("{}.lnk", clean.trim())
}

/// On a thread of its own, so the shell's objects never meet the window's:
/// the thread starts and ends its own use of COM.
/// Ok(false), and nothing written, when `only_if_there` and the desktop has
/// no shortcut of that name.
#[cfg(windows)]
fn write_shortcut(file_name: &str, exe: &Path, dir: &Path, icon: &Path, only_if_there: bool) -> std::result::Result<bool, String> {
    use std::os::windows::ffi::OsStringExt;
    use windows::core::{Interface, HSTRING};
    use windows::Win32::System::Com::{
        CoCreateInstance, CoInitializeEx, CoTaskMemFree, CoUninitialize, IPersistFile, CLSCTX_INPROC_SERVER,
        COINIT_APARTMENTTHREADED,
    };
    use windows::Win32::UI::Shell::{FOLDERID_Desktop, IShellLinkW, SHGetKnownFolderPath, ShellLink, KF_FLAG_DEFAULT};

    let (file_name, exe, dir, icon) = (file_name.to_owned(), exe.to_owned(), dir.to_owned(), icon.to_owned());
    let work = move || -> std::result::Result<bool, String> {
        let write = || -> windows::core::Result<bool> {
            // SAFETY: COM is started on this thread below and ends after this
            // runs; the string Windows gives for the desktop is freed once read
            unsafe {
                let p = SHGetKnownFolderPath(&FOLDERID_Desktop, KF_FLAG_DEFAULT, None)?;
                let desktop = std::ffi::OsString::from_wide(p.as_wide());
                CoTaskMemFree(Some(p.0 as *const _));
                let lnk = Path::new(&desktop).join(&file_name);
                if only_if_there && !lnk.is_file() {
                    return Ok(false);
                }
                let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
                link.SetPath(&HSTRING::from(exe.as_path()))?;
                link.SetWorkingDirectory(&HSTRING::from(dir.as_path()))?;
                link.SetIconLocation(&HSTRING::from(icon.as_path()), 0)?;
                link.cast::<IPersistFile>()?.Save(&HSTRING::from(lnk.as_path()), true)?;
                Ok(true)
            }
        };
        // SAFETY: a new thread, so COM is started here once and ended once
        unsafe {
            CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok().map_err(|e| e.message())?;
            let done = write();
            CoUninitialize();
            done.map_err(|e| e.message())
        }
    };
    std::thread::spawn(work).join().unwrap_or_else(|_| Err("unknown error".into()))
}

#[cfg(not(windows))]
fn write_shortcut(_file_name: &str, _exe: &Path, _dir: &Path, _icon: &Path, _only_if_there: bool) -> std::result::Result<bool, String> {
    Err("Desktop shortcuts are only made on Windows.".into())
}

/// Whether the copy of the game in this folder is running. Only this copy:
/// the same game started from another folder (a developer's own build, say)
/// is none of the launcher's business. Windows keeps the file of a running
/// program closed to writing, so asking to write to it tells, and no list of
/// processes is needed.
pub fn is_running(project: &Project, dir: &Path) -> bool {
    let Some(exe) = project.exe.as_ref().map(|e| dir.join(e)) else {
        return false;
    };
    if !cfg!(windows) || !exe.is_file() {
        return false;
    }
    // 32: ERROR_SHARING_VIOLATION. Anything else (read-only file, no
    // permission) is not a running game
    std::fs::OpenOptions::new().write(true).open(&exe).is_err_and(|e| e.raw_os_error() == Some(32))
}

/// Takes a project off the disk: its folder with everything in it (the built
/// game, what was extracted from the user's file, saved games). Ok(false) and
/// nothing deleted when the folder is a git working copy (somebody's own
/// clone, pointed at with "use a folder I already have") or is already gone:
/// the launcher then only forgets it. A folder that does not look like the
/// project is refused, whatever the settings say.
pub fn uninstall(project: &Project, dir: &Path) -> Result<bool> {
    if !dir.is_dir() || dir.join(".git").exists() {
        return Ok(false);
    }
    if !is_installed(project, dir) {
        return Err(format!("{} does not look like a folder of {}: nothing was deleted.", dir.display(), project.name));
    }
    std::fs::remove_dir_all(dir).map_err(|e| {
        format!("Could not delete all of {}: {e}. Close anything that is using it and try again.", dir.display())
    })?;
    Ok(true)
}

/// The news entries the installed copy has.
pub fn local_news(project: &Project, dir: &Path) -> Vec<news::Entry> {
    project
        .news
        .as_ref()
        .and_then(|file| std::fs::read_to_string(dir.join(file)).ok())
        .map(|text| news::parse(&text))
        .unwrap_or_default()
}

/// Which version of the project is installed, for the user to see and quote.
/// The projects have no version number of their own: each entry of their
/// `news.txt` is one, so the newest entry the installed copy has is its
/// version. The commit says exactly which code it is.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Version {
    /// Number of the newest entry in the installed `news.txt`.
    pub number: Option<u32>,
    /// That entry's title, usually its date.
    pub title: Option<String>,
    /// The commit the installed copy was made from, in full.
    pub commit: Option<String>,
}

/// The installed copy's version, from its news (see `local_news`) and the
/// commit the launcher or `update.bat` recorded, or the commit checked out
/// in a git working copy. None when none of them is there.
pub fn installed_version(project: &Project, dir: &Path, news: &[news::Entry]) -> Option<Version> {
    let newest = news.iter().max_by_key(|e| e.number);
    let commit = project
        .update_state
        .as_deref()
        .filter(|state| is_plain_relative(state))
        .and_then(|state| crate::update::installed_version(&dir.join(state)))
        .or_else(|| git_head(dir))
        .filter(|sha| sha.len() >= 7 && sha.chars().all(|c| c.is_ascii_hexdigit()));
    let version = Version { number: newest.map(|e| e.number), title: newest.map(|e| e.title.clone()), commit };
    (version.number.is_some() || version.commit.is_some()).then_some(version)
}

/// The commit checked out in a git working copy: `.git/HEAD`, followed to
/// its branch (a file of its own, or a line of `packed-refs`).
fn git_head(dir: &Path) -> Option<String> {
    let git = dir.join(".git");
    let head = std::fs::read_to_string(git.join("HEAD")).ok()?;
    let head = head.trim();
    let Some(name) = head.strip_prefix("ref:").map(str::trim) else { return Some(head.to_lowercase()) };
    if !name.starts_with("refs/") || !is_plain_relative(name) {
        return None;
    }
    if let Ok(sha) = std::fs::read_to_string(git.join(name)) {
        return Some(sha.trim().to_lowercase());
    }
    let packed = std::fs::read_to_string(git.join("packed-refs")).ok()?;
    packed
        .lines()
        .filter_map(|line| line.split_once(' '))
        .find(|(_, r)| r.trim() == name)
        .map(|(sha, _)| sha.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::Registry;
    use std::fs;

    #[test]
    fn shortcut_names_windows_allows() {
        assert_eq!(shortcut_file_name("Nico SR2"), "Nico SR2.lnk");
        assert_eq!(shortcut_file_name("Star Wars: Trilogy / Arcade?"), "Star Wars  Trilogy   Arcade.lnk");
        assert_eq!(shortcut_file_name("a<b>c\"d|e*f\\g\th"), "a b c d e f g h.lnk");
    }

    #[test]
    fn uninstall_deletes_only_what_the_launcher_may() {
        let reg = Registry::bundled();
        let p = reg.find("swtrilogy").unwrap();
        let base = std::env::temp_dir().join(format!("nl-uninstall-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);

        // Not there: nothing to do
        assert_eq!(uninstall(p, &base.join("gone")), Ok(false));

        // Some other folder the settings point at by mistake: refused, untouched
        let other = base.join("documents");
        fs::create_dir_all(&other).unwrap();
        fs::write(other.join("thesis.txt"), "x").unwrap();
        assert!(uninstall(p, &other).is_err());
        assert!(other.join("thesis.txt").is_file());

        // A git working copy: kept whole
        let clone = base.join("clone");
        fs::create_dir_all(clone.join(".git")).unwrap();
        fs::write(clone.join("build.bat"), "@echo off").unwrap();
        assert_eq!(uninstall(p, &clone), Ok(false));
        assert!(clone.join("build.bat").is_file());

        // What the launcher installed: gone, with what is inside
        let game = base.join("swtrilogy");
        fs::create_dir_all(game.join("save")).unwrap();
        fs::write(game.join("hub.toml"), "schema = 1").unwrap();
        fs::write(game.join("save").join("records.bin"), "x").unwrap();
        assert_eq!(uninstall(p, &game), Ok(true));
        assert!(!game.exists());
        assert!(other.join("thesis.txt").is_file());
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn follows_a_project_from_nothing_to_ready() {
        let reg = Registry::bundled();
        let p = reg.find("swtrilogy").unwrap();
        let dir = std::env::temp_dir().join(format!("nl-state-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        assert_eq!(state(p, None, None), State::NotInstalled);
        assert_eq!(state(p, Some(&dir), None), State::NotInstalled);

        fs::create_dir_all(&dir).unwrap();
        assert_eq!(state(p, Some(&dir), None), State::NotInstalled);
        fs::write(dir.join("install.bat"), "@echo off").unwrap();
        assert_eq!(state(p, Some(&dir), None), State::NeedsInput);
        assert_eq!(state(p, Some(&dir), Some(&dir.join("gone.zip"))), State::NeedsInput);

        let rom = dir.join("own.zip");
        fs::write(&rom, "x").unwrap();
        assert_eq!(state(p, Some(&dir), Some(&rom)), State::NeedsBuild);
        let cmd = script_command(p, &dir, Script::Install, Some(&rom)).unwrap();
        assert_eq!(cmd.get_args().last().unwrap(), rom.as_os_str());
        assert!(script_command(p, &dir, Script::Update, None).is_err(), "update.bat is not there");

        fs::write(dir.join("StarWarsTrilogy.exe"), "x").unwrap();
        assert_eq!(state(p, Some(&dir), None), State::Ready);

        fs::write(dir.join("news.txt"), "[3] t\n- a\n").unwrap();
        assert_eq!(news::latest(&local_news(p, &dir)), 3);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn tells_the_installed_version() {
        let reg = Registry::bundled();
        let p = reg.find("swtrilogy").unwrap();
        let dir = std::env::temp_dir().join(format!("nl-version-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        assert_eq!(installed_version(p, &dir, &[]), None);

        fs::write(dir.join("news.txt"), "[4] 14 November 2026\n- b\n[3] 13 November 2026\n- a\n").unwrap();
        let v = installed_version(p, &dir, &local_news(p, &dir)).unwrap();
        assert_eq!((v.number, v.title.as_deref(), v.commit), (Some(4), Some("14 November 2026"), None));

        // What the launcher or update.bat recorded
        let sha = "0123456789abcdef0123456789abcdef01234567";
        fs::create_dir_all(dir.join("save/update")).unwrap();
        fs::write(dir.join("save/update/version.txt"), format!("{}\r\n", sha.to_uppercase())).unwrap();
        assert_eq!(installed_version(p, &dir, &[]).unwrap().commit.as_deref(), Some(sha));
        fs::remove_dir_all(dir.join("save")).unwrap();

        // A git working copy: its branch, loose or packed
        let other = "89abcdef0123456789abcdef0123456789abcdef";
        fs::create_dir_all(dir.join(".git/refs/heads")).unwrap();
        fs::write(dir.join(".git/HEAD"), "ref: refs/heads/main\n").unwrap();
        fs::write(dir.join(".git/packed-refs"), format!("# pack-refs\n{other} refs/heads/main\n")).unwrap();
        assert_eq!(installed_version(p, &dir, &[]).unwrap().commit.as_deref(), Some(other));
        fs::write(dir.join(".git/refs/heads/main"), format!("{sha}\n")).unwrap();
        assert_eq!(installed_version(p, &dir, &[]).unwrap().commit.as_deref(), Some(sha));
        // Detached
        fs::write(dir.join(".git/HEAD"), other).unwrap();
        assert_eq!(installed_version(p, &dir, &[]).unwrap().commit.as_deref(), Some(other));
        fs::remove_dir_all(&dir).unwrap();
    }
}
