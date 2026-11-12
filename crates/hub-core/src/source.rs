//! Getting a project's source code from GitHub: the same zip the "Download ZIP"
//! button gives, for one exact commit. Source code only: nothing built is ever
//! downloaded.

use crate::{agent, err, http_error, registry::Project, Result};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{self, Read, Seek, Write};
use std::path::{Component, Path, PathBuf};

/// Downloads the project's source into `dest`, which must not exist yet or be
/// empty. Everything is unpacked next to it first and moved into place at the
/// end, so a download that fails leaves no half-installed folder.
pub fn install(project: &Project, dest: &Path, log: &mut dyn FnMut(&str)) -> Result<()> {
    if dir_has_files(dest) {
        return Err(format!("{} already has files in it.", dest.display()));
    }
    let parent = dest.parent().ok_or("Bad install folder")?;
    fs::create_dir_all(parent).map_err(err("Could not create the install folder"))?;

    let tag = format!("{}-{}", project.id, std::process::id());
    let zip_path = parent.join(format!(".{tag}.zip.part"));
    let staging = parent.join(format!(".{tag}.installing"));
    let result = (|| {
        log(&format!("Downloading the source of {}...", project.name));
        download(&zip_url(project), &zip_path, log)?;
        log("Unpacking...");
        let _ = fs::remove_dir_all(&staging);
        let zip = File::open(&zip_path).map_err(err("Could not open the download"))?;
        let (files, commit) = unpack(zip, &staging)?;
        match (&commit, &project.update_state) {
            (Some(sha), Some(state)) => {
                log(&format!("Version {}", &sha[..7]));
                write_update_state(&staging.join(state), sha, &files)?;
            }
            _ => {}
        }
        if dest.exists() {
            fs::remove_dir(dest).map_err(err("Could not replace the install folder"))?;
        }
        fs::rename(&staging, dest).map_err(err("Could not move the files into place"))?;
        log(&format!("{} files in {}", files.len(), dest.display()));
        Ok(())
    })();
    let _ = fs::remove_file(&zip_path);
    if result.is_err() {
        let _ = fs::remove_dir_all(&staging);
    }
    result
}

/// The branch as a zip, straight from the download host: no GitHub API, so no
/// hourly limit on a shared connection.
pub(crate) fn zip_url(project: &Project) -> String {
    format!("https://codeload.github.com/{}/zip/refs/heads/{}", project.repo, project.branch)
}

fn dir_has_files(dir: &Path) -> bool {
    fs::read_dir(dir).map(|mut d| d.next().is_some()).unwrap_or(false)
}

pub(crate) fn download(url: &str, to: &Path, log: &mut dyn FnMut(&str)) -> Result<()> {
    // No overall limit: only a connection that goes quiet for a minute fails
    let resp = agent(60).get(url).call().map_err(|e| http_error(url, e))?;
    let mut reader = resp.into_reader();
    let mut file = File::create(to).map_err(err("Could not write the download"))?;
    let mut buf = vec![0u8; 256 * 1024];
    let (mut total, mut next_report) = (0u64, 0u64);
    loop {
        let n = reader.read(&mut buf).map_err(err("The download was cut off"))?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n]).map_err(err("Could not write the download"))?;
        total += n as u64;
        if total >= next_report {
            log(&format!("  {} MB", total / (1024 * 1024)));
            next_report = total + 16 * 1024 * 1024;
        }
    }
    Ok(())
}

/// Unpacks a GitHub source zip into `out`, without the top folder GitHub wraps
/// everything in. Returns each file's path (relative, `/` separated) and SHA256,
/// and the commit the zip was made from (GitHub writes it as the zip's comment).
pub fn unpack<R: Read + Seek>(zip: R, out: &Path) -> Result<(Vec<(String, String)>, Option<String>)> {
    let mut archive = zip::ZipArchive::new(zip).map_err(err("The download is not a zip"))?;
    let commit = std::str::from_utf8(archive.comment())
        .ok()
        .map(|c| c.trim().to_lowercase())
        .filter(|c| c.len() == 40 && c.chars().all(|ch| ch.is_ascii_hexdigit()));
    let mut files = Vec::new();
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(err("Bad zip entry"))?;
        // None for a name that would land outside the folder (`..`, absolute)
        let name = entry
            .enclosed_name()
            .ok_or_else(|| format!("Unsafe path in the zip: {}", entry.name()))?;
        let rel: PathBuf = name.components().skip(1).collect();
        if rel.as_os_str().is_empty() {
            continue;
        }
        if rel.components().any(|c| !matches!(c, Component::Normal(_))) {
            return Err(format!("Unsafe path in the zip: {}", entry.name()));
        }
        let target = out.join(&rel);
        if entry.is_dir() {
            fs::create_dir_all(&target).map_err(err("Could not create a folder"))?;
            continue;
        }
        if let Some(dir) = target.parent() {
            fs::create_dir_all(dir).map_err(err("Could not create a folder"))?;
        }
        let mut file = File::create(&target).map_err(err("Could not write a file"))?;
        let mut hasher = Sha256::new();
        let mut tee = Tee { file: &mut file, hasher: &mut hasher };
        io::copy(&mut entry, &mut tee).map_err(err("Could not unpack a file"))?;
        let rel_text = rel
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .join("/");
        files.push((rel_text, hex(&hasher.finalize())));
    }
    if files.is_empty() {
        return Err("The download has no files in it".into());
    }
    Ok((files, commit))
}

struct Tee<'a> {
    file: &'a mut File,
    hasher: &'a mut Sha256,
}

impl Write for Tee<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.file.write(buf)?;
        self.hasher.update(&buf[..n]);
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// What the projects' own updater (`tools\update.ps1`) keeps after an update:
/// `version.txt` with the commit and `manifest.txt` with `<sha256> <path>` per
/// file. Written at install so its first update knows which files are untouched
/// and does not back every one of them up.
pub(crate) fn write_update_state(dir: &Path, sha: &str, files: &[(String, String)]) -> Result<()> {
    fs::create_dir_all(dir).map_err(err("Could not write the version record"))?;
    let mut lines: Vec<String> = files
        .iter()
        .map(|(rel, hash)| format!("{hash} {}", rel.replace('/', "\\")))
        .collect();
    lines.sort_by_key(|l| l[65..].to_lowercase());
    let mut text = lines.join("\r\n");
    text.push_str("\r\n");
    fs::write(dir.join("manifest.txt"), text).map_err(err("Could not write the version record"))?;
    fs::write(dir.join("version.txt"), format!("{sha}\r\n"))
        .map_err(err("Could not write the version record"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use zip::write::SimpleFileOptions;

    fn zip_of(entries: &[(&str, &[u8])]) -> Cursor<Vec<u8>> {
        let mut w = zip::ZipWriter::new(Cursor::new(Vec::new()));
        w.set_comment("0123456789abcdef0123456789abcdef01234567");
        for (name, data) in entries {
            if name.ends_with('/') {
                w.add_directory(*name, SimpleFileOptions::default()).unwrap();
            } else {
                w.start_file(*name, SimpleFileOptions::default()).unwrap();
                w.write_all(data).unwrap();
            }
        }
        Cursor::new(w.finish().unwrap().into_inner())
    }

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nl-{name}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn unpacks_without_the_top_folder_and_hashes() {
        let out = temp("unpack");
        let zip = zip_of(&[
            ("Repo-abc123/", b""),
            ("Repo-abc123/build.bat", b"@echo off\r\n"),
            ("Repo-abc123/tools/", b""),
            ("Repo-abc123/tools/x.py", b"print(1)\n"),
        ]);
        let (files, commit) = unpack(zip, &out).unwrap();
        assert_eq!(commit.as_deref(), Some("0123456789abcdef0123456789abcdef01234567"));
        assert_eq!(fs::read(out.join("build.bat")).unwrap(), b"@echo off\r\n");
        assert_eq!(fs::read(out.join("tools/x.py")).unwrap(), b"print(1)\n");
        assert!(!out.join("Repo-abc123").exists());
        let names: Vec<&str> = files.iter().map(|f| f.0.as_str()).collect();
        assert_eq!(names, ["build.bat", "tools/x.py"]);
        // sha256 of "print(1)\n"
        assert_eq!(files[1].1, hex(&Sha256::digest(b"print(1)\n")));
        assert_eq!(files[1].1.len(), 64);

        write_update_state(&out.join("save/update"), &"a".repeat(40), &files).unwrap();
        let manifest = fs::read_to_string(out.join("save/update/manifest.txt")).unwrap();
        assert!(manifest.ends_with(" tools\\x.py\r\n"), "{manifest:?}");
        assert_eq!(manifest.lines().count(), 2);
        let version = fs::read_to_string(out.join("save/update/version.txt")).unwrap();
        assert_eq!(version.trim(), "a".repeat(40));
        fs::remove_dir_all(&out).unwrap();
    }

    #[test]
    fn refuses_paths_that_leave_the_folder() {
        let out = temp("unsafe");
        let zip = zip_of(&[("Repo-abc/ok.txt", b"ok"), ("Repo-abc/../../evil.txt", b"x")]);
        assert!(unpack(zip, &out).is_err());
        assert!(!out.parent().unwrap().join("evil.txt").exists());
        let _ = fs::remove_dir_all(&out);
    }

    #[test]
    fn refuses_an_empty_or_broken_download() {
        let out = temp("empty");
        assert!(unpack(zip_of(&[("Repo-abc/", b"")]), &out).is_err());
        assert!(unpack(Cursor::new(b"<html>proxy page</html>".to_vec()), &out).is_err());
        let _ = fs::remove_dir_all(&out);
    }
}

#[cfg(test)]
mod network_tests {
    use super::*;
    use crate::project::{self, State};
    use crate::registry::Registry;

    /// Real download from GitHub: `cargo test -p hub-core -- --ignored`
    #[test]
    #[ignore = "needs the network"]
    fn installs_a_real_project() {
        let reg = Registry::bundled();
        let p = reg.find("lostworld").unwrap();
        let dest = std::env::temp_dir().join(format!("nl-real-{}", std::process::id())).join(&p.id);
        let _ = fs::remove_dir_all(dest.parent().unwrap());
        install(p, &dest, &mut |l| println!("{l}")).unwrap();
        assert_eq!(project::state(p, Some(&dest), None), State::NeedsInput);
        assert!(dest.join("save/update/version.txt").is_file());
        assert!(!crate::project::local_news(p, &dest).is_empty());
        // Nothing left behind next to it, and a second install is refused
        assert_eq!(fs::read_dir(dest.parent().unwrap()).unwrap().count(), 1);
        assert!(install(p, &dest, &mut |_| {}).is_err());
        assert!(!crate::news::fetch(p).unwrap().is_empty());
        fs::remove_dir_all(dest.parent().unwrap()).unwrap();
    }
}
