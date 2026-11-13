//! What a problem report says about the PC: the Windows version and the
//! graphics card. Read only when the Feedback page asks, shown to the user,
//! and sent nowhere by the launcher: the user sends the report on GitHub.

use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SystemInfo {
    /// "Windows 11 Home 24H2 (build 26100)"
    pub os: String,
    /// "NVIDIA GeForce RTX 4070 · driver 576.52"; several cards one after another
    pub gpu: String,
}

/// One PowerShell call: the edition from WMI (the registry still says
/// "Windows 10" on Windows 11), the feature update ("24H2") from the
/// registry, and every display adapter with its driver.
/// No double quotes in it, so it reaches PowerShell as written whatever the
/// quoting of the command line; UTF-8 out, for names that are not ASCII.
#[cfg(windows)]
const SCRIPT: &str = "[Console]::OutputEncoding = [Text.Encoding]::UTF8; $t = [char]9; \
$o = Get-CimInstance Win32_OperatingSystem; \
$d = (Get-ItemProperty 'HKLM:\\SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion').DisplayVersion; \
'OS' + $t + $o.Caption + $t + $d + $t + $o.BuildNumber; \
Get-CimInstance Win32_VideoController | ForEach-Object { 'GPU' + $t + $_.Name + $t + $_.DriverVersion }";

#[cfg(windows)]
pub fn info() -> SystemInfo {
    let mut cmd = std::process::Command::new("powershell");
    cmd.args(["-NoProfile", "-NonInteractive", "-Command", SCRIPT]);
    crate::runner::no_window(&mut cmd);
    match cmd.output() {
        Ok(out) => parse(&String::from_utf8_lossy(&out.stdout)),
        Err(_) => parse(""),
    }
}

#[cfg(not(windows))]
pub fn info() -> SystemInfo {
    SystemInfo { os: std::env::consts::OS.to_string(), gpu: UNKNOWN.to_string() }
}

const UNKNOWN: &str = "unknown";

/// Reads the lines of `SCRIPT`: `OS<tab>caption<tab>display version<tab>build`
/// and `GPU<tab>name<tab>driver version`.
pub fn parse(text: &str) -> SystemInfo {
    let mut os = None;
    let mut gpus = Vec::new();
    for line in text.lines() {
        let f: Vec<&str> = line.trim_end_matches('\r').split('\t').map(str::trim).collect();
        match f.as_slice() {
            ["OS", caption, rest @ ..] if !caption.is_empty() => {
                let name = caption.strip_prefix("Microsoft ").unwrap_or(caption);
                let release = rest.first().copied().unwrap_or("");
                let build = rest.get(1).copied().unwrap_or("");
                let mut s = name.to_string();
                if !release.is_empty() {
                    s += &format!(" {release}");
                }
                if !build.is_empty() {
                    s += &format!(" (build {build})");
                }
                os = Some(s);
            }
            ["GPU", name, rest @ ..] if !name.is_empty() => {
                // Remote desktop and fallback adapters say nothing about the PC
                if name.starts_with("Microsoft Basic") || name.starts_with("Microsoft Remote") {
                    continue;
                }
                let driver = rest.first().map(|d| driver_version(name, d)).unwrap_or_default();
                gpus.push(if driver.is_empty() { name.to_string() } else { format!("{name} · driver {driver}") });
            }
            _ => {}
        }
    }
    SystemInfo {
        os: os.unwrap_or_else(|| UNKNOWN.to_string()),
        gpu: if gpus.is_empty() { UNKNOWN.to_string() } else { gpus.join(" + ") },
    }
}

/// Windows shows NVIDIA drivers as 32.0.15.7652; NVIDIA calls that one
/// 576.52, the number players know. Other makers keep Windows' number.
fn driver_version(name: &str, version: &str) -> String {
    if name.to_ascii_uppercase().contains("NVIDIA") {
        let digits: String = version.chars().filter(char::is_ascii_digit).collect();
        if digits.len() >= 5 {
            let last = &digits[digits.len() - 5..];
            let major = last[..3].trim_start_matches('0');
            return format!("{}.{}", if major.is_empty() { "0" } else { major }, &last[3..]);
        }
    }
    version.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_and_cards() {
        let out = "OS\tMicrosoft Windows 11 Home\t24H2\t26100\r\n\
                   GPU\tIntel(R) UHD Graphics 770\t31.0.101.5186\r\n\
                   GPU\tNVIDIA GeForce RTX 4070\t32.0.15.7652\r\n\
                   GPU\tMicrosoft Basic Display Adapter\t10.0.26100.1\r\n";
        let i = parse(out);
        assert_eq!(i.os, "Windows 11 Home 24H2 (build 26100)");
        assert_eq!(i.gpu, "Intel(R) UHD Graphics 770 · driver 31.0.101.5186 + NVIDIA GeForce RTX 4070 · driver 576.52");
    }

    #[test]
    fn missing_pieces() {
        assert_eq!(parse(""), SystemInfo { os: "unknown".into(), gpu: "unknown".into() });
        let i = parse("OS\tMicrosoft Windows 10 Pro\t\t19045\nGPU\tAMD Radeon RX 7800 XT\t\n");
        assert_eq!(i.os, "Windows 10 Pro (build 19045)");
        assert_eq!(i.gpu, "AMD Radeon RX 7800 XT");
    }

    #[test]
    fn nvidia_driver_numbers() {
        assert_eq!(driver_version("NVIDIA GeForce GTX 1060", "31.0.15.3623"), "536.23");
        assert_eq!(driver_version("NVIDIA GeForce RTX 3080", "32.0.15.6094"), "560.94");
        assert_eq!(driver_version("AMD Radeon", "31.0.24033.1003"), "31.0.24033.1003");
    }
}
