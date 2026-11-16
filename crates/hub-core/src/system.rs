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

/// The Windows version from the registry, the graphics cards from DirectX:
/// what any game asks, and nothing started for it.
#[cfg(windows)]
pub fn info() -> SystemInfo {
    let os = windows_registry::LOCAL_MACHINE
        .open(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion")
        .map(|key| {
            let get = |name: &str| key.get_string(name).unwrap_or_default();
            // DisplayVersion ("24H2") since 20H2; ReleaseId ("2004") before
            let release = Some(get("DisplayVersion")).filter(|r| !r.is_empty()).unwrap_or_else(|| get("ReleaseId"));
            describe_os(&get("ProductName"), &release, &get("CurrentBuildNumber"))
        })
        .unwrap_or_else(|_| UNKNOWN.to_string());
    SystemInfo { os, gpu: describe_gpus(&adapters()) }
}

/// Every graphics card with its driver version, as Windows numbers it
/// (32.0.15.7652). Windows' own software renderer is left out.
#[cfg(windows)]
fn adapters() -> Vec<(String, String)> {
    use windows::core::Interface;
    use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIDevice, IDXGIFactory1, DXGI_ADAPTER_FLAG_SOFTWARE};

    let mut out = Vec::new();
    // SAFETY: plain DirectX queries; every object is released when dropped
    unsafe {
        let Ok(factory) = CreateDXGIFactory1::<IDXGIFactory1>() else {
            return out;
        };
        let mut i = 0;
        while let Ok(adapter) = factory.EnumAdapters1(i) {
            i += 1;
            let Ok(desc) = adapter.GetDesc1() else { continue };
            if desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32 != 0 {
                continue;
            }
            let len = desc.Description.iter().position(|&c| c == 0).unwrap_or(desc.Description.len());
            let name = String::from_utf16_lossy(&desc.Description[..len]);
            let driver = adapter.CheckInterfaceSupport(&IDXGIDevice::IID).map(driver_number).unwrap_or_default();
            out.push((name, driver));
        }
    }
    out
}

/// DirectX gives the driver version as four 16-bit numbers in one.
#[cfg(windows)]
fn driver_number(v: i64) -> String {
    let v = v as u64;
    format!("{}.{}.{}.{}", v >> 48, (v >> 32) & 0xffff, (v >> 16) & 0xffff, v & 0xffff)
}

#[cfg(not(windows))]
pub fn info() -> SystemInfo {
    SystemInfo { os: std::env::consts::OS.to_string(), gpu: UNKNOWN.to_string() }
}

const UNKNOWN: &str = "unknown";

/// "Windows 11 Home 24H2 (build 26100)". The registry still names Windows 11
/// "Windows 10": build 22000 and later are Windows 11.
pub fn describe_os(product: &str, release: &str, build: &str) -> String {
    let product = product.trim();
    let product = product.strip_prefix("Microsoft ").unwrap_or(product);
    if product.is_empty() {
        return UNKNOWN.to_string();
    }
    let mut s = match build.trim().parse::<u32>() {
        Ok(b) if b >= 22000 => product.replacen("Windows 10", "Windows 11", 1),
        _ => product.to_string(),
    };
    if !release.trim().is_empty() {
        s += &format!(" {}", release.trim());
    }
    if !build.trim().is_empty() {
        s += &format!(" (build {})", build.trim());
    }
    s
}

/// The cards one after another: "NVIDIA GeForce RTX 4070 · driver 576.52".
pub fn describe_gpus(adapters: &[(String, String)]) -> String {
    let gpus: Vec<String> = adapters
        .iter()
        .map(|(name, driver)| (name.trim(), driver.trim()))
        // Remote desktop and fallback adapters say nothing about the PC
        .filter(|(name, _)| {
            !name.is_empty() && !name.starts_with("Microsoft Basic") && !name.starts_with("Microsoft Remote")
        })
        .map(|(name, driver)| {
            let driver = driver_version(name, driver);
            if driver.is_empty() { name.to_string() } else { format!("{name} · driver {driver}") }
        })
        .collect();
    if gpus.is_empty() { UNKNOWN.to_string() } else { gpus.join(" + ") }
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

    fn cards(list: &[(&str, &str)]) -> Vec<(String, String)> {
        list.iter().map(|(n, d)| (n.to_string(), d.to_string())).collect()
    }

    #[test]
    fn windows_and_cards() {
        assert_eq!(describe_os("Windows 10 Home", "24H2", "26100"), "Windows 11 Home 24H2 (build 26100)");
        let gpus = cards(&[
            ("Intel(R) UHD Graphics 770", "31.0.101.5186"),
            ("NVIDIA GeForce RTX 4070", "32.0.15.7652"),
            ("Microsoft Basic Display Adapter", "10.0.26100.1"),
        ]);
        assert_eq!(
            describe_gpus(&gpus),
            "Intel(R) UHD Graphics 770 · driver 31.0.101.5186 + NVIDIA GeForce RTX 4070 · driver 576.52"
        );
    }

    #[test]
    fn missing_pieces() {
        assert_eq!(describe_os("", "", ""), "unknown");
        assert_eq!(describe_gpus(&[]), "unknown");
        assert_eq!(describe_os("Windows 10 Pro", "", "19045"), "Windows 10 Pro (build 19045)");
        assert_eq!(describe_os("Windows 10 Pro", "2004", ""), "Windows 10 Pro 2004");
        assert_eq!(describe_gpus(&cards(&[("AMD Radeon RX 7800 XT", "")])), "AMD Radeon RX 7800 XT");
    }

    #[test]
    fn nvidia_driver_numbers() {
        assert_eq!(driver_version("NVIDIA GeForce GTX 1060", "31.0.15.3623"), "536.23");
        assert_eq!(driver_version("NVIDIA GeForce RTX 3080", "32.0.15.6094"), "560.94");
        assert_eq!(driver_version("AMD Radeon", "31.0.24033.1003"), "31.0.24033.1003");
    }
}
