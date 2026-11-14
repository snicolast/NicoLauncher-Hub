//! Updates an installed project to the latest version of its source: what the
//! projects' `tools\update.ps1` does, done by the launcher. Source code only.
//!
//! It never touches what is not part of the repository: the user's own file,
//! saves, the built program, downloaded libraries. A repository file the user
//! changed is copied to `out\update_backup\<date>\` before it is replaced.
//!
//! It keeps the same record as `update.ps1` (`version.txt` and `manifest.txt`
//! in the project's update folder), so either one can do the next update.

use crate::registry::{is_plain_relative, Project};
use crate::source::{download, unpack, write_update_state, zip_url};
use crate::{build::sha256_of, err, Result};
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Default, PartialEq)]
pub struct Report {
    pub added: usize,
    pub updated: usize,
    pub removed: usize,
    /// Files changed on this PC that were copied aside before being replaced
    /// or removed.
    pub backed_up: Vec<String>,
    /// False on the first update of a folder with no record: then every file
    /// that differs is backed up, because an old version and a change made
    /// here cannot be told apart.
    pub had_record: bool,
}

#[derive(Debug, PartialEq)]
pub enum Outcome {
    UpToDate { version: String },
    Updated { version: String, report: Report },
}

/// The commit the installed copy was made from, when there is a record.
pub fn installed_version(state_dir: &Path) -> Option<String> {
    let text = fs::read_to_string(state_dir.join("version.txt")).ok()?;
    let first = text.lines().next()?.trim().to_lowercase();
    (!first.is_empty()).then_some(first)
}

/// `manifest.txt`: `<sha256> <path>` per line. Keyed by the path in lower case
/// with `/` (Windows does not tell `Tools\A.py` from `tools\a.py`); the value
/// keeps the path as written.
fn read_record(state_dir: &Path) -> HashMap<String, (String, String)> {
    let text = fs::read_to_string(state_dir.join("manifest.txt")).unwrap_or_default();
    text.lines()
        .filter_map(|line| line.trim_end().split_once(' '))
        .map(|(hash, rel)| (rel.replace('\\', "/"), hash.to_lowercase()))
        .filter(|(rel, _)| is_plain_relative(rel))
        .map(|(rel, hash)| (rel.to_lowercase(), (rel, hash)))
        .collect()
}

fn copy_to(from: &Path, to: &Path) -> Result<()> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).map_err(err("Could not create a folder"))?;
    }
    fs::copy(from, to).map(|_| ()).map_err(|e| format!("Could not write {}: {e}", to.display()))
}

/// Brings `dest` to the version unpacked in `src`. `files` is every file of
/// that version with its SHA256 (what `source::unpack` returns).
pub fn apply(src: &Path, files: &[(String, String)], dest: &Path, state_dir: &Path, version: &str, backup: &Path) -> Result<Report> {
    let old = read_record(state_dir);
    let mut report = Report { had_record: !old.is_empty(), ..Default::default() };
    let save = |rel: &str, report: &mut Report| -> Result<()> {
        copy_to(&dest.join(rel), &backup.join(rel))?;
        report.backed_up.push(rel.to_string());
        Ok(())
    };
    for (rel, hash) in files {
        let target = dest.join(rel);
        if target.is_file() {
            let local = sha256_of(&target)?;
            if &local == hash {
                continue;
            }
            // Changed here since the last update, or no record of it: keep a copy
            if old.get(&rel.to_lowercase()).is_none_or(|(_, recorded)| recorded != &local) {
                save(rel, &mut report)?;
            }
            report.updated += 1;
        } else {
            report.added += 1;
        }
        copy_to(&src.join(rel), &target)?;
    }
    // Files the previous version had and this one does not: removed (a copy
    // kept first if they were changed here), and the folders they leave empty
    let new: std::collections::HashSet<String> = files.iter().map(|(rel, _)| rel.to_lowercase()).collect();
    for (key, (rel, recorded)) in &old {
        let path = dest.join(rel);
        if new.contains(key) || !path.is_file() {
            continue;
        }
        if &sha256_of(&path)? != recorded {
            save(rel, &mut report)?;
        }
        fs::remove_file(&path).map_err(|e| format!("Could not remove {}: {e}", path.display()))?;
        report.removed += 1;
        let mut dir = path.parent();
        while let Some(d) = dir.filter(|d| *d != dest && d.starts_with(dest)) {
            // Fails, and stops here, as soon as a folder is not empty
            if fs::remove_dir(d).is_err() {
                break;
            }
            dir = d.parent();
        }
    }
    write_update_state(state_dir, version, files)?;
    report.backed_up.sort();
    Ok(report)
}

/// `2026-11-11_093000` (UTC), for the backup folder's name.
pub(crate) fn timestamp() -> String {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let (days, rest) = (secs / 86400, secs % 86400);
    // Days since 1970 to a date (Howard Hinnant's civil_from_days)
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let (day, month) = (doy - (153 * mp + 2) / 5 + 1, if mp < 10 { mp + 3 } else { mp - 9 });
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}_{:02}{:02}{:02}", rest / 3600, rest % 3600 / 60, rest % 60)
}

/// Downloads the latest source of the project's branch and applies it to
/// `dir`. Nothing is changed when the installed version is already that one,
/// unless `force`. It does not rebuild: the caller does.
pub fn update(project: &Project, dir: &Path, force: bool, log: &mut dyn FnMut(&str)) -> Result<Outcome> {
    let state_rel = project.update_state.as_ref().ok_or_else(|| format!("{} keeps no version record.", project.name))?;
    let state_dir = dir.join(state_rel);
    let parent = dir.parent().ok_or("Bad install folder")?;
    let tag = format!("{}-{}", project.id, std::process::id());
    let zip_path = parent.join(format!(".{tag}.zip.part"));
    let staging = parent.join(format!(".{tag}.updating"));
    let result = (|| {
        log(&format!("Downloading the latest source of {}...", project.name));
        download(&zip_url(project), &zip_path, log)?;
        let _ = fs::remove_dir_all(&staging);
        let zip = File::open(&zip_path).map_err(err("Could not open the download"))?;
        let (files, commit) = unpack(zip, &staging)?;
        let version = commit.ok_or("The download does not say which version it is.")?;
        let current = installed_version(&state_dir);
        if current.as_deref() == Some(version.as_str()) && !force {
            log(&format!("Up to date (version {}).", &version[..7]));
            return Ok(Outcome::UpToDate { version });
        }
        match &current {
            Some(c) => log(&format!("New version {} (installed: {}).", &version[..7], &c[..c.len().min(7)])),
            None => log(&format!("Latest version: {}. The installed version is unknown (first update).", &version[..7])),
        }
        let backup: PathBuf = dir.join("out").join("update_backup").join(timestamp());
        let report = apply(&staging, &files, dir, &state_dir, &version, &backup)?;
        log(&format!("Files: {} new, {} updated, {} removed.", report.added, report.updated, report.removed));
        if !report.backed_up.is_empty() {
            let n = report.backed_up.len();
            if report.had_record {
                log(&format!("{n} file(s) you had changed were copied to {} before being replaced:", backup.display()));
            } else {
                log(&format!("First update: your own changes cannot be told from an older version, so the {n} file(s) replaced were copied to {}:", backup.display()));
            }
            for rel in report.backed_up.iter().take(15) {
                log(&format!("  {rel}"));
            }
            if n > 15 {
                log("  ...");
            }
        }
        Ok(Outcome::Updated { version, report })
    })();
    let _ = fs::remove_file(&zip_path);
    let _ = fs::remove_dir_all(&staging);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const V1: &str = "1111111111111111111111111111111111111111";
    const V2: &str = "2222222222222222222222222222222222222222";

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nl-update-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Writes a version's files into `root/name` and returns what `unpack` would.
    fn version(root: &Path, name: &str, files: &[(&str, &str)]) -> (PathBuf, Vec<(String, String)>) {
        let src = root.join(name);
        let mut list = Vec::new();
        for (rel, text) in files {
            fs::create_dir_all(src.join(rel).parent().unwrap()).unwrap();
            fs::write(src.join(rel), text).unwrap();
            list.push((rel.to_string(), sha256_of(&src.join(rel)).unwrap()));
        }
        (src, list)
    }

    fn read(path: PathBuf) -> String {
        fs::read_to_string(path).unwrap()
    }

    #[test]
    fn updates_only_the_repository_files_and_keeps_the_rest() {
        let root = temp("apply");
        let (dest, state, backup) = (root.join("game"), root.join("game/save/update"), root.join("game/out/update_backup/now"));
        let (src1, files1) = version(&root, "v1", &[("build.bat", "v1"), ("src/main.cpp", "v1"), ("src/old/gone.cpp", "v1"), ("tools/kept.py", "same")]);
        let r = apply(&src1, &files1, &dest, &state, V1, &backup).unwrap();
        assert_eq!((r.added, r.updated, r.removed, r.had_record), (4, 0, 0, false));
        assert_eq!(installed_version(&state).as_deref(), Some(V1));

        // What the user has that is not the repository's
        fs::create_dir_all(dest.join("data")).unwrap();
        fs::write(dest.join("data/crom.bin"), "rom data").unwrap();
        fs::write(dest.join("rom.zip"), "the rom").unwrap();
        fs::write(dest.join("Game.exe"), "built").unwrap();
        fs::write(dest.join("save/settings.ini"), "mine").unwrap();
        fs::write(dest.join("src/old/notes.txt"), "mine").unwrap();
        // and a repository file they edited
        fs::write(dest.join("src/main.cpp"), "edited by the user").unwrap();

        let (src2, files2) = version(&root, "v2", &[("build.bat", "v2"), ("src/main.cpp", "v2"), ("tools/kept.py", "same"), ("docs/new.md", "v2")]);
        let r = apply(&src2, &files2, &dest, &state, V2, &backup).unwrap();
        assert_eq!((r.added, r.updated, r.removed, r.had_record), (1, 2, 1, true));
        assert_eq!(r.backed_up, ["src/main.cpp"], "build.bat was untouched: no copy of it");
        assert_eq!(read(backup.join("src/main.cpp")), "edited by the user");
        assert_eq!(read(dest.join("src/main.cpp")), "v2");
        assert_eq!(read(dest.join("build.bat")), "v2");
        assert!(!dest.join("src/old/gone.cpp").exists());
        assert!(dest.join("src/old/notes.txt").is_file(), "a folder with the user's file in it stays");
        for (file, text) in [("data/crom.bin", "rom data"), ("rom.zip", "the rom"), ("Game.exe", "built"), ("save/settings.ini", "mine")] {
            assert_eq!(read(dest.join(file)), text);
        }
        assert_eq!(installed_version(&state).as_deref(), Some(V2));
        let record = read(state.join("manifest.txt"));
        assert!(record.contains(" docs\\new.md\r\n") && !record.contains("gone.cpp"), "the format update.ps1 reads");

        // The same version again changes nothing
        let r = apply(&src2, &files2, &dest, &state, V2, &backup).unwrap();
        assert_eq!((r.added, r.updated, r.removed, r.backed_up.len()), (0, 0, 0, 0));
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_first_update_with_no_record_backs_up_what_it_replaces() {
        let root = temp("first");
        let (dest, state, backup) = (root.join("game"), root.join("game/save/update"), root.join("backup"));
        fs::create_dir_all(dest.join("src")).unwrap();
        fs::write(dest.join("build.bat"), "from an old download").unwrap();
        fs::write(dest.join("src/same.cpp"), "same").unwrap();
        let (src, files) = version(&root, "v1", &[("build.bat", "v1"), ("src/same.cpp", "same")]);
        let r = apply(&src, &files, &dest, &state, V1, &backup).unwrap();
        assert_eq!((r.updated, r.had_record), (1, false));
        assert_eq!(r.backed_up, ["build.bat"]);
        assert_eq!(read(backup.join("build.bat")), "from an old download");
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn removes_emptied_folders_and_backs_up_an_edited_file_it_removes() {
        let root = temp("remove");
        let (dest, state, backup) = (root.join("game"), root.join("game/save/update"), root.join("backup"));
        let (src1, files1) = version(&root, "v1", &[("a.txt", "1"), ("deep/er/one.txt", "1"), ("deep/two.txt", "1")]);
        apply(&src1, &files1, &dest, &state, V1, &backup).unwrap();
        fs::write(dest.join("deep/two.txt"), "edited").unwrap();
        let (src2, files2) = version(&root, "v2", &[("a.txt", "1")]);
        let r = apply(&src2, &files2, &dest, &state, V2, &backup).unwrap();
        assert_eq!(r.removed, 2);
        assert_eq!(r.backed_up, ["deep/two.txt"]);
        assert!(!dest.join("deep").exists());
        assert!(dest.join("a.txt").is_file());
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_record_cannot_point_outside_the_folder() {
        let root = temp("outside");
        let (dest, state) = (root.join("game"), root.join("game/save/update"));
        fs::create_dir_all(&state).unwrap();
        fs::write(root.join("precious.txt"), "not the project's").unwrap();
        let hash = sha256_of(&root.join("precious.txt")).unwrap();
        fs::write(state.join("manifest.txt"), format!("{hash} ..\\precious.txt\r\n{hash} C:\\x.txt\r\n")).unwrap();
        let (src, files) = version(&root, "v1", &[("a.txt", "1")]);
        let r = apply(&src, &files, &dest, &state, V1, &root.join("backup")).unwrap();
        assert_eq!(r.removed, 0);
        assert!(root.join("precious.txt").is_file());
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn reads_the_record_update_ps1_writes() {
        let state = temp("record");
        fs::write(state.join("version.txt"), format!("{}\r\n", V1.to_uppercase())).unwrap();
        fs::write(state.join("manifest.txt"), "ABC123 Tools\\Update.ps1\r\ndef456 file with spaces.txt\r\n").unwrap();
        assert_eq!(installed_version(&state).as_deref(), Some(V1));
        let record = read_record(&state);
        assert_eq!(record["tools/update.ps1"], ("Tools/Update.ps1".to_string(), "abc123".to_string()));
        assert_eq!(record["file with spaces.txt"].1, "def456");
        assert_eq!(installed_version(&state.join("none")), None);
        fs::remove_dir_all(&state).unwrap();
    }

    #[test]
    fn names_the_backup_folder_by_date() {
        let t = timestamp();
        assert_eq!(t.len(), 17, "{t}");
        assert!(t.starts_with("20") && t.as_bytes()[10] == b'_');
    }
}
