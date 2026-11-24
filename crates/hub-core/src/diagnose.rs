//! What went wrong, for someone who has never built a program: a task that
//! failed is matched against the problems users have already run into
//! (issues on GitHub), so the launcher can say in plain words what happened
//! and offer the fix, instead of only showing the log. A failure that matches
//! none of them is left as it was: the log and "Report this problem".

use crate::kit::Kit;
use crate::Result;
use serde::Serialize;
use std::path::Path;

/// A known cause of a failed install, update or build. The interface words
/// it and picks the button; the log stays the same.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Diagnosis {
    /// A file of the build kit is gone (an antivirus removed it, most often):
    /// those tools are downloaded again. `tools`: their folders in the kit
    /// (`llvm-mingw-20251104`).
    KitDamaged { tools: Vec<String> },
    /// The kit's files disappear as soon as they are unpacked: getting them
    /// again does not help until the antivirus leaves the folder alone.
    KitBlocked,
    /// The user's file is not the one the project needs. `detail`: what the
    /// project's own check said about it, in its words.
    WrongInput { detail: Vec<String> },
    /// Compiled files of an earlier version do not fit the new code (they
    /// were not rebuilt): building from scratch fixes it.
    StaleBuild,
    /// The disk is full.
    DiskFull,
    /// Windows refused a file: the game, a window in its folder or an
    /// antivirus has it open, or the folder may not be written to.
    Locked,
    /// The download did not get through: no connection, cut off, or blocked.
    Network,
}

/// Lines with one of these: the disk is full (Windows, POSIX tools, Python).
const DISK_FULL: &[&str] = &["os error 112", "not enough space on the disk", "no space left on device", "errno 28"];
/// A file or program that should be there is not.
const NOT_FOUND: &[&str] = &["file not found", "no such file", "cannot find", "cannot open", "unable to find", "not recognized"];
/// Windows would not let a file be written, moved or deleted.
const LOCKED: &[&str] = &["os error 32", "os error 5)", "being used by another process", "access is denied", "permission denied"];
/// The linker found code that calls something no compiled file has.
const UNDEFINED: &[&str] = &["undefined symbol:", "undefined reference to"];
/// Words of the launcher's own downloads when they fail (lib.rs, build.rs).
const NETWORK: &[&str] = &["could not reach ", "the download was cut off", "timed out", "unknownissuer", "dns failed", "connection reset", "connection refused"];

/// How far back from the end of the log to look: the cause is near the
/// failure, and a long build logs thousands of lines that came out fine.
const TAIL: usize = 400;

/// The known problem in a failed task's log (`error`: the message it ended
/// with), or None. First match wins: a full disk breaks everything after it,
/// so it is looked for before what it can look like.
pub fn diagnose(log: &[String], error: &str) -> Option<Diagnosis> {
    let tail: Vec<String> = log[log.len().saturating_sub(TAIL)..].iter().chain(std::iter::once(&error.to_string())).map(|l| l.to_lowercase()).collect();
    let has = |words: &[&str], line: &str| words.iter().any(|w| line.contains(w));
    let any = |words: &[&str]| tail.iter().any(|l| has(words, l));

    if any(DISK_FULL) {
        return Some(Diagnosis::DiskFull);
    }
    if any(&["disappeared right after it was unpacked"]) {
        return Some(Diagnosis::KitBlocked);
    }
    // A missing file inside one of the kit's tools: the line itself names it
    // (clang: "C:/.../kit/llvm-mingw-20251104/include/...: fatal error: ...
    // file not found")
    let kit = Kit::builtin();
    let mut tools: Vec<String> = Vec::new();
    for line in tail.iter().filter(|l| has(NOT_FOUND, l)) {
        for t in &kit.tools {
            let dir = format!("{}-{}", t.name, t.version);
            if line.contains(&dir.to_lowercase()) && !tools.contains(&dir) {
                tools.push(dir);
            }
        }
    }
    if !tools.is_empty() {
        return Some(Diagnosis::KitDamaged { tools });
    }
    if let Some(detail) = wrong_input(log, error) {
        return Some(Diagnosis::WrongInput { detail });
    }
    if any(LOCKED) {
        return Some(Diagnosis::Locked);
    }
    if any(UNDEFINED) {
        return Some(Diagnosis::StaleBuild);
    }
    if has(NETWORK, &error.to_lowercase()) || tail.iter().rev().take(5).any(|l| has(NETWORK, l)) {
        return Some(Diagnosis::Network);
    }
    None
}

/// The step that reads the user's file failed: every project names it
/// "Extracting the ROM" in its hub.toml, and its script says what is wrong
/// with the file. Its lines are the detail, without the one that tells
/// people who build by hand to run build.bat again.
fn wrong_input(log: &[String], error: &str) -> Option<Vec<String>> {
    const STEP: &str = "Extracting the ROM";
    let failed = |l: &str| l.starts_with(&format!("{STEP}: It ended with an error"));
    if !failed(error) && !log.iter().rev().take(3).any(|l| failed(l)) {
        return None;
    }
    let start = log.iter().rposition(|l| l.starts_with('[') && l.ends_with(&format!("] {STEP}")))?;
    let detail = log[start + 1..]
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty() && !failed(l) && !l.contains("build.bat"))
        .take(8)
        .map(String::from)
        .collect();
    Some(detail)
}

/// What the button of a diagnosis does before the task runs again.
#[derive(Debug, Clone, serde::Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Fix {
    /// These tools of the kit (their folders, as `KitDamaged` names them)
    /// are deleted, so the task downloads them again whole.
    RepairKit { tools: Vec<String> },
    /// The compiled files of the project (`build/kit`, where its hub.toml
    /// builds with the kit, and `build/cmake`, where build.bat builds) are
    /// deleted, so everything is compiled again. Nothing else: saved games,
    /// settings and what was extracted from the user's file stay.
    CleanBuild,
}

/// Folders a clean build deletes, relative to the project's folder.
pub const BUILD_DIRS: [&str; 2] = ["build/kit", "build/cmake"];

impl Fix {
    pub fn apply(&self, kit_root: &Path, dir: &Path, log: &mut dyn FnMut(&str)) -> Result<()> {
        let gone = |path: &Path, log: &mut dyn FnMut(&str)| -> Result<()> {
            if !path.is_dir() {
                return Ok(());
            }
            log(&format!("Deleting {}", path.display()));
            std::fs::remove_dir_all(path)
                .map_err(|e| format!("Could not delete {}: {e}. Close anything that is using it and try again.", path.display()))
        };
        match self {
            // Only folders of tools this launcher knows: never another path
            Fix::RepairKit { tools } => {
                for t in Kit::builtin().tools.iter().filter(|t| tools.contains(&format!("{}-{}", t.name, t.version))) {
                    gone(&t.dir(kit_root), log)?;
                }
            }
            Fix::CleanBuild => {
                for rel in BUILD_DIRS {
                    gone(&dir.join(rel), log)?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(text: &str) -> Vec<String> {
        text.lines().map(String::from).collect()
    }

    // The tail of the log in StarWarsTrilogyNative #107: a header of the
    // compiler gone from the kit
    #[test]
    fn a_file_gone_from_the_kit() {
        let log = lines(
            "[5/7] Compiling\n\
             In file included from C:/Users/x/AppData/Local/NicoLauncher/kit/llvm-mingw-20251104/include/c++/v1/__iterator/concepts.h:29:\n\
             C:/Users/x/AppData/Local/NicoLauncher/kit/llvm-mingw-20251104/include/c++/v1/__iterator/incrementable_traits.h:18:10: fatal error: '__type_traits/is_primary_template.h' file not found\n\
             1 error generated.\n\
             ninja: build stopped: subcommand failed.",
        );
        assert_eq!(
            diagnose(&log, "Compiling: It ended with an error (code 1)."),
            Some(Diagnosis::KitDamaged { tools: vec!["llvm-mingw-20251104".into()] })
        );
    }

    #[test]
    fn the_kit_removed_as_soon_as_it_is_unpacked() {
        let e = "3 file(s) of llvm-mingw disappeared right after it was unpacked (a, b, c). An antivirus is probably removing them: add C:\\kit as an exception and try again.";
        assert_eq!(diagnose(&[], e), Some(Diagnosis::KitBlocked));
    }

    // StarWarsTrilogyNative #115: another set than the one it needs
    #[test]
    fn another_rom() {
        let log = lines(
            "[1/7] Getting the libraries\n\
             Downloading SDL3\n\
             [2/7] Extracting the ROM\n\
             C:\\Users\\x\\Downloads\\swtrilogy.zip is not a complete Star Wars Trilogy ROM (swtrilgy, Rev. A).\n\
             41 of its 41 chips are missing: this is another game or another version.\n\
             The zip has 2 file(s): a.bin, b.bin\n\
             Get a current, complete swtrilgy.zip (Rev. A) and run build.bat again.",
        );
        let d = diagnose(&log, "Extracting the ROM: It ended with an error (code 1).");
        assert_eq!(
            d,
            Some(Diagnosis::WrongInput {
                detail: vec![
                    "C:\\Users\\x\\Downloads\\swtrilogy.zip is not a complete Star Wars Trilogy ROM (swtrilgy, Rev. A).".into(),
                    "41 of its 41 chips are missing: this is another game or another version.".into(),
                    "The zip has 2 file(s): a.bin, b.bin".into(),
                ]
            })
        );
    }

    // SegaRally2Native #82: objects of the previous version, not rebuilt
    #[test]
    fn old_compiled_files() {
        let log = lines(
            "[5/7] Compiling\n\
             ld.lld: error: undefined symbol: r3d_set_sr2_snowy(bool)\n\
             >>> referenced by CMakeFiles/SegaRally2.dir/src/runtime/runtime.cpp.obj:(rt_event(Ctx&))\n\
             clang-21: error: linker command failed with exit code 1 (use -v to see invocation)\n\
             ninja: build stopped: subcommand failed.",
        );
        assert_eq!(diagnose(&log, "Compiling: It ended with an error (code 1)."), Some(Diagnosis::StaleBuild));
    }

    // SegaRally2Native #72
    #[test]
    fn no_connection() {
        let log = lines("Downloading the source of Sega Rally 2 Native...");
        let e = "Could not reach https://codeload.github.com/snicolast/SegaRally2Native/zip/refs/heads/main: Connection Failed: tls connection init failed: invalid peer certificate: UnknownIssuer";
        assert_eq!(diagnose(&log, e), Some(Diagnosis::Network));
    }

    #[test]
    fn a_full_disk_first() {
        let log = lines("[5/7] Compiling\nLLVM ERROR: IO failure on output stream: No space left on device\nfatal error: 'x.h' file not found");
        assert_eq!(diagnose(&log, "Compiling: It ended with an error (code 1)."), Some(Diagnosis::DiskFull));
        assert_eq!(diagnose(&[], "Could not write data/crom.bin: There is not enough space on the disk. (os error 112)"), Some(Diagnosis::DiskFull));
    }

    #[test]
    fn a_file_windows_holds() {
        let e = "Could not delete all of D:\\Games\\segarally2: The process cannot access the file because it is being used by another process. (os error 32).";
        assert_eq!(diagnose(&[], e), Some(Diagnosis::Locked));
        assert_eq!(diagnose(&[], "Could not move the download into place: Access is denied. (os error 5)"), Some(Diagnosis::Locked));
    }

    #[test]
    fn an_unknown_failure_stays_unknown() {
        let log = lines("[5/7] Compiling\nsrc/game.cpp:12:3: error: use of undeclared identifier 'foo'\nninja: build stopped: subcommand failed.");
        assert_eq!(diagnose(&log, "Compiling: It ended with an error (code 1)."), None);
        assert_eq!(diagnose(&[], "Cancelled."), None);
    }

    // A header of the game's own source that is not found is a bug of the
    // game, not of the kit
    #[test]
    fn a_missing_file_outside_the_kit_is_not_the_kit() {
        let log = lines("D:/Games/swtrilogy/src/game.cpp:3:10: fatal error: 'gone.h' file not found");
        assert_eq!(diagnose(&log, "Compiling: It ended with an error (code 1)."), None);
    }

    #[test]
    fn only_the_end_of_a_long_log() {
        let mut log = vec!["could not reach the mirror, using the next one".to_string()];
        log.extend((0..TAIL).map(|i| format!("[{i}/999] Building CXX object x{i}.obj")));
        log.push("src/game.cpp:12:3: error: use of undeclared identifier 'foo'".into());
        assert_eq!(diagnose(&log, "Compiling: It ended with an error (code 1)."), None);
    }

    #[test]
    fn fixes_delete_only_their_folders() {
        let base = std::env::temp_dir().join(format!("nl-fix-{}", std::process::id()));
        let (kit, game) = (base.join("kit"), base.join("game"));
        let llvm = Kit::builtin().tools.iter().find(|t| t.name == "llvm-mingw").unwrap().clone();
        let llvm_dir = format!("{}-{}", llvm.name, llvm.version);
        for d in [kit.join(&llvm_dir), kit.join("cmake-0"), kit.join("elsewhere"), game.join("build/kit/x"), game.join("build/cmake"), game.join("build/gen"), game.join("save")] {
            std::fs::create_dir_all(&d).unwrap();
        }
        let mut log = |_: &str| {};
        Fix::RepairKit { tools: vec![llvm_dir.clone(), "elsewhere".into(), "../game".into()] }.apply(&kit, &game, &mut log).unwrap();
        assert!(!kit.join(&llvm_dir).exists());
        assert!(kit.join("elsewhere").exists() && kit.join("cmake-0").exists() && game.is_dir());
        Fix::CleanBuild.apply(&kit, &game, &mut log).unwrap();
        assert!(!game.join("build/kit").exists() && !game.join("build/cmake").exists());
        assert!(game.join("build/gen").exists() && game.join("save").exists());
        // Nothing left to delete is not an error
        Fix::CleanBuild.apply(&kit, &game, &mut log).unwrap();
        std::fs::remove_dir_all(&base).unwrap();
    }
}
