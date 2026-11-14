//! Where the launcher keeps its own files: in the user's profile when it was
//! installed, or all in its own folder when it runs portable.
//!
//! Portable means a `portable.txt` (or a `config.json`) next to the exe. Then
//! the settings, the build kit and the window's data stay in that folder and
//! nothing is written to AppData, so the folder can be moved, or carried on a
//! USB drive, as a whole.

use crate::{err, Result};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

/// The file that makes a copy of the launcher portable.
pub const MARKER: &str = "portable.txt";
/// The launcher's exe inside the portable zip.
pub const EXE_IN_ZIP: &str = "NicoLauncher.exe";
/// What the updater asks for when it runs portable: the zip, not the installer.
pub const UPDATE_TARGET: &str = "windows-x86_64-portable";

#[derive(Debug, Clone, PartialEq)]
pub struct Places {
    /// The folder the launcher runs portable from, if it does.
    pub portable: Option<PathBuf>,
    pub config: PathBuf,
    /// The build kit's tools.
    pub kit: PathBuf,
    /// The window's own data (WebView2). None: where Tauri puts it, in
    /// `%LOCALAPPDATA%`.
    pub webview: Option<PathBuf>,
}

impl Places {
    /// For the exe that is running.
    pub fn here() -> Places {
        let exe_dir = std::env::current_exe().ok().and_then(|e| e.parent().map(Path::to_path_buf));
        Places::for_exe_dir(exe_dir.as_deref())
    }

    pub fn for_exe_dir(exe_dir: Option<&Path>) -> Places {
        match exe_dir.filter(|dir| is_portable(dir)) {
            Some(dir) => Places {
                portable: Some(dir.to_path_buf()),
                config: dir.join("config.json"),
                kit: dir.join("kit"),
                webview: Some(dir.join("webview")),
            },
            None => Places {
                portable: None,
                config: crate::config::Config::default_path(),
                kit: crate::kit::default_root(),
                webview: None,
            },
        }
    }
}

fn is_portable(dir: &Path) -> bool {
    dir.join(MARKER).is_file() || dir.join("config.json").is_file()
}

/// Why the build kit cannot be unpacked in this folder, if it cannot: the
/// same characters that stop a project's folder stop the tools themselves.
pub fn kit_problem(kit: &Path) -> Option<String> {
    crate::config::folder_problem(kit)?;
    let folder = kit.parent().unwrap_or(kit);
    Some(format!(
        "The launcher's folder, {}, has [ ] or ; in its name, and the build tools cannot work from a folder like that. Move the launcher to a folder without them.",
        folder.display()
    ))
}

/// The new exe from a downloaded portable zip.
pub fn exe_from_zip(zip: &[u8]) -> Result<Vec<u8>> {
    let mut archive = zip::ZipArchive::new(io::Cursor::new(zip)).map_err(err("The download is not a zip"))?;
    let mut file = archive.by_name(EXE_IN_ZIP).map_err(err("The download has no launcher in it"))?;
    let mut exe = Vec::new();
    file.read_to_end(&mut exe).map_err(err("Could not read the download"))?;
    Ok(exe)
}

/// The name the exe that is replaced moves to: Windows lets a running exe be
/// renamed, not overwritten. Deleted on the next start.
pub fn old_exe(exe: &Path) -> PathBuf {
    let mut name = exe.file_name().unwrap_or_default().to_os_string();
    name.push(".old");
    exe.with_file_name(name)
}

/// Puts the new exe in place of the running one. The old one is kept aside
/// until the next start, and put back if the new one cannot be written.
pub fn replace_exe(exe: &Path, new: &[u8]) -> Result<()> {
    let old = old_exe(exe);
    let _ = std::fs::remove_file(&old);
    std::fs::rename(exe, &old).map_err(|e| format!("Could not update {}: {e}", exe.display()))?;
    if let Err(e) = std::fs::write(exe, new) {
        let _ = std::fs::remove_file(exe);
        let _ = std::fs::rename(&old, exe);
        return Err(format!("Could not write the new version to {}: {e}", exe.display()));
    }
    Ok(())
}

/// Deletes the exe an update left aside, once the old process has let go of it.
pub fn remove_old_exe(exe: &Path) {
    let old = old_exe(exe);
    for _ in 0..20 {
        if !old.exists() || std::fs::remove_file(&old).is_ok() {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nl-portable-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn portable_only_with_its_marker() {
        let dir = temp("marker");
        let installed = Places::for_exe_dir(Some(&dir));
        assert_eq!(installed.portable, None);
        assert_eq!(installed.config, crate::config::Config::default_path());
        assert_eq!(installed.webview, None);

        std::fs::write(dir.join(MARKER), "").unwrap();
        let portable = Places::for_exe_dir(Some(&dir));
        assert_eq!(portable.portable.as_deref(), Some(dir.as_path()));
        assert_eq!(portable.config, dir.join("config.json"));
        assert_eq!(portable.kit, dir.join("kit"));
        assert_eq!(portable.webview, Some(dir.join("webview")));

        // A config.json next to the exe is enough too
        std::fs::remove_file(dir.join(MARKER)).unwrap();
        std::fs::write(dir.join("config.json"), "{}").unwrap();
        assert!(Places::for_exe_dir(Some(&dir)).portable.is_some());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_folder_the_kit_cannot_use() {
        assert!(kit_problem(Path::new("/tools/NicoLauncher/kit")).is_none());
        let problem = kit_problem(Path::new("/tools/[Portable]/NicoLauncher/kit")).unwrap();
        assert!(problem.contains("[Portable]/NicoLauncher,") && problem.contains("Move the launcher"), "{problem}");
    }

    #[test]
    fn the_exe_comes_out_of_the_zip() {
        let mut w = zip::ZipWriter::new(io::Cursor::new(Vec::new()));
        w.start_file(MARKER, zip::write::SimpleFileOptions::default()).unwrap();
        w.write_all(b"portable").unwrap();
        w.start_file(EXE_IN_ZIP, zip::write::SimpleFileOptions::default()).unwrap();
        w.write_all(b"MZ new").unwrap();
        let zip = w.finish().unwrap().into_inner();
        assert_eq!(exe_from_zip(&zip).unwrap(), b"MZ new");
        assert!(exe_from_zip(b"not a zip").is_err());
    }

    #[test]
    fn the_exe_is_swapped_and_the_old_one_removed() {
        let dir = temp("swap");
        let exe = dir.join("NicoLauncher.exe");
        std::fs::write(&exe, "old").unwrap();
        replace_exe(&exe, b"new").unwrap();
        assert_eq!(std::fs::read(&exe).unwrap(), b"new");
        assert_eq!(std::fs::read(old_exe(&exe)).unwrap(), b"old");
        assert_eq!(old_exe(&exe), dir.join("NicoLauncher.exe.old"));
        remove_old_exe(&exe);
        assert!(!old_exe(&exe).exists());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
