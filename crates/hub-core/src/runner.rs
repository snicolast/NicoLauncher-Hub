//! Runs a command and hands its output over line by line, as it comes.

use std::io::{self, Read};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{mpsc, Arc};

/// Lets another thread stop a running command.
#[derive(Clone, Default)]
pub struct Cancel {
    pid: Arc<AtomicU32>,
    asked: Arc<AtomicBool>,
}

impl Cancel {
    pub fn new() -> Cancel {
        Cancel::default()
    }

    pub fn was_asked(&self) -> bool {
        self.asked.load(Ordering::SeqCst)
    }

    /// Stops the command and everything it started (a .bat starts compilers,
    /// installers...).
    pub fn cancel(&self) {
        self.asked.store(true, Ordering::SeqCst);
        let pid = self.pid.load(Ordering::SeqCst);
        if pid != 0 {
            kill_tree(pid);
        }
    }
}

#[cfg(windows)]
fn kill_tree(pid: u32) {
    let mut cmd = Command::new("taskkill");
    cmd.args(["/PID", &pid.to_string(), "/T", "/F"]).stdout(Stdio::null()).stderr(Stdio::null());
    no_window(&mut cmd);
    let _ = cmd.status();
}

#[cfg(not(windows))]
fn kill_tree(pid: u32) {
    // The command runs in its own process group (see `run`)
    let _ = Command::new("kill").args(["-KILL", "--", &format!("-{pid}")]).status();
}

/// No console window flashing up for a command started from the launcher.
#[cfg(windows)]
pub fn no_window(cmd: &mut Command) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    cmd.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
pub fn no_window(_cmd: &mut Command) {}

/// Runs `cmd` to the end and returns its exit code (-1 when it was killed).
/// stdout and stderr both go to `on_line`. Nothing can be typed into it: a
/// script that waits for a key gets none and carries on.
///
/// Its input is a pipe that is closed at once, not the null device: on
/// Windows NUL is a character device, like a console, so `pause` and `choice`
/// take it for a keyboard and wait on it for ever. A closed pipe reads as the
/// end of a file.
pub fn run(mut cmd: Command, cancel: &Cancel, on_line: &mut dyn FnMut(&str)) -> io::Result<i32> {
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    no_window(&mut cmd);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        cmd.process_group(0);
    }
    let mut child = cmd.spawn()?;
    drop(child.stdin.take());
    cancel.pid.store(child.id(), Ordering::SeqCst);
    if cancel.was_asked() {
        kill_tree(child.id());
    }

    let (tx, rx) = mpsc::channel::<String>();
    let mut readers = Vec::new();
    let out = child.stdout.take().map(|s| Box::new(s) as Box<dyn Read + Send>);
    let errs = child.stderr.take().map(|s| Box::new(s) as Box<dyn Read + Send>);
    for stream in [out, errs].into_iter().flatten() {
        let tx = tx.clone();
        readers.push(std::thread::spawn(move || read_lines(stream, &tx)));
    }
    drop(tx);
    for line in rx {
        on_line(&line);
    }
    for r in readers {
        let _ = r.join();
    }
    let status = child.wait()?;
    cancel.pid.store(0, Ordering::SeqCst);
    Ok(status.code().unwrap_or(-1))
}

/// Lines end at `\n` or at `\r` (progress bars redraw with `\r`). Bytes that
/// are not UTF-8 (a console code page) are replaced, never an error.
fn read_lines(mut stream: Box<dyn Read + Send>, tx: &mpsc::Sender<String>) {
    let mut buf = [0u8; 8192];
    let mut line: Vec<u8> = Vec::new();
    let send = |line: &mut Vec<u8>| {
        if !line.is_empty() {
            let _ = tx.send(String::from_utf8_lossy(line).trim_end().to_string());
            line.clear();
        }
    };
    loop {
        match stream.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                for &b in &buf[..n] {
                    if b == b'\n' || b == b'\r' {
                        send(&mut line);
                    } else {
                        line.push(b);
                    }
                }
            }
        }
    }
    send(&mut line);
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    fn sh(script: &str) -> Command {
        let mut c = Command::new("sh");
        c.args(["-c", script]);
        c
    }

    #[test]
    fn streams_both_outputs_and_returns_the_exit_code() {
        let mut lines = Vec::new();
        let code = run(sh("echo one; echo two >&2; printf 'a\\rb\\n'; exit 3"), &Cancel::new(), &mut |l| {
            lines.push(l.to_string())
        })
        .unwrap();
        assert_eq!(code, 3);
        lines.sort();
        assert_eq!(lines, ["a", "b", "one", "two"]);
    }

    #[test]
    fn a_script_waiting_for_a_key_does_not_hang() {
        let code = run(sh("read x; echo after"), &Cancel::new(), &mut |_| {}).unwrap();
        assert_eq!(code, 0);
    }

    #[test]
    fn cancel_stops_it_and_what_it_started() {
        let cancel = Cancel::new();
        let c2 = cancel.clone();
        let started = std::time::Instant::now();
        let killer = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(300));
            c2.cancel();
        });
        let code = run(sh("sleep 30 & sleep 30"), &cancel, &mut |_| {}).unwrap();
        killer.join().unwrap();
        assert_eq!(code, -1);
        assert!(cancel.was_asked());
        assert!(started.elapsed() < std::time::Duration::from_secs(10));
    }
}

/// The same checks against real .bat files, as the launcher runs the
/// projects' scripts on Windows. Every one runs under a deadline: a script
/// that hangs must fail its test with what it printed, not hang the run.
#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::Mutex;
    use std::time::{Duration, Instant};

    fn bat(name: &str, body: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nl bat {}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(name);
        std::fs::write(&path, body.replace('\n', "\r\n")).unwrap();
        path
    }

    /// Runs `cmd` on another thread and gives up after `secs`: then it is
    /// cancelled and the test fails, saying whether cancelling freed it.
    fn run_within(secs: u64, cmd: Command, cancel: &Cancel) -> (i32, Vec<String>) {
        let lines = Arc::new(Mutex::new(Vec::new()));
        let (tx, rx) = mpsc::channel();
        let (lines2, cancel2) = (lines.clone(), cancel.clone());
        std::thread::spawn(move || {
            let code = run(cmd, &cancel2, &mut |l| lines2.lock().unwrap().push(l.to_string()));
            let _ = tx.send(code);
        });
        match rx.recv_timeout(Duration::from_secs(secs)) {
            Ok(code) => (code.unwrap(), lines.lock().unwrap().clone()),
            Err(_) => {
                cancel.cancel();
                let freed = rx.recv_timeout(Duration::from_secs(15)).is_ok();
                panic!(
                    "still running after {secs}s (cancel freed it: {freed}); output so far: {:?}",
                    lines.lock().unwrap()
                );
            }
        }
    }

    #[test]
    fn a_bat_gets_its_argument_and_its_exit_code_comes_back() {
        let path = bat("args.bat", "@echo off\necho one\necho [%~1] 1>&2\nexit /b 3\n");
        let mut cmd = Command::new(&path);
        cmd.arg(r"D:\my roms\own file.zip");
        let (code, mut lines) = run_within(30, cmd, &Cancel::new());
        assert_eq!(code, 3);
        lines.sort();
        assert_eq!(lines, [r"[D:\my roms\own file.zip]", "one"]);
    }

    /// install.bat and build.bat end on `pause` when they think a person
    /// started them: with nothing to read it must carry on.
    #[test]
    fn pause_does_not_hang() {
        let path = bat("pause.bat", "@echo off\necho before\npause\necho after\nexit /b 0\n");
        let (code, lines) = run_within(30, Command::new(&path), &Cancel::new());
        assert_eq!(code, 0, "{lines:?}");
        assert!(lines.iter().any(|l| l == "before") && lines.iter().any(|l| l == "after"), "{lines:?}");
    }

    /// install.bat asks "Start the game now" with `choice`: no answer must end
    /// the question, and must not count as yes.
    #[test]
    fn choice_does_not_hang_or_say_yes() {
        let path = bat(
            "choice.bat",
            "@echo off\necho before\nchoice /c YN /m \"Start now\"\nif not errorlevel 2 echo WOULD START\necho after\nexit /b 0\n",
        );
        let (code, lines) = run_within(30, Command::new(&path), &Cancel::new());
        assert_eq!(code, 0, "{lines:?}");
        // choice gives up ("The file is either empty...") and leaves its prompt
        // without a line end, so the next echo lands on the same line
        assert!(lines.iter().any(|l| l.ends_with("after")), "{lines:?}");
        assert!(!lines.iter().any(|l| l.contains("WOULD START")), "{lines:?}");
    }

    #[test]
    fn cancel_stops_a_bat_and_what_it_started() {
        let path = bat("long.bat", "@echo off\necho started\nping -n 60 127.0.0.1 >nul\necho not reached\n");
        let cancel = Cancel::new();
        let c2 = cancel.clone();
        let started = Instant::now();
        let killer = std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(2));
            c2.cancel();
        });
        let (code, lines) = run_within(40, Command::new(&path), &cancel);
        killer.join().unwrap();
        assert_ne!(code, 0);
        assert!(started.elapsed() < Duration::from_secs(30), "{:?}", started.elapsed());
        assert!(!lines.iter().any(|l| l.contains("not reached")), "{lines:?}");
    }
}
