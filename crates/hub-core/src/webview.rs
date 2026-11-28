//! The launcher's window is drawn by Microsoft Edge WebView2. When Windows
//! says it is not there, Tauri's own message sends people to Microsoft's page
//! with several downloads, two of which install nothing, so installing "it"
//! again and again changes nothing. This message names the right download,
//! the usual reasons it still fails, and what to send when asking for help.

/// The Evergreen Bootstrapper: the small download that installs WebView2 for
/// this PC. The installer uses the same one when WebView2 is missing.
pub const DOWNLOAD: &str = "https://go.microsoft.com/fwlink/p/?LinkId=2124703";

/// The message, and whether it offers to open [`DOWNLOAD`]. `os` describes
/// the Windows version, `error` is what Windows answered.
pub fn missing_message(os: &str, error: &str, wine: bool) -> (String, bool) {
    let details = format!("Details: {os} · {}", error.trim());
    if wine {
        return (
            format!(
                "NicoLauncher needs Microsoft Edge WebView2 to show its window, and it is not available here.\n\n\
                 This looks like Wine or Proton (Linux, Steam Deck), where WebView2 does not work. \
                 NicoLauncher runs on Windows 10 or 11 for now.\n\n{details}"
            ),
            false,
        );
    }
    (
        format!(
            "NicoLauncher needs Microsoft Edge WebView2 to show its window, and Windows says it is not installed for this user.\n\n\
             1. Press Yes to download the Evergreen Bootstrapper from Microsoft, and run it. \
             Not the \"Fixed Version\" or the SDK: those install nothing.\n\
             2. Start NicoLauncher again.\n\n\
             Still the same? Some trimmed-down Windows versions (Tiny11, AtlasOS, scripts that remove Edge) \
             block WebView2. NicoLauncher needs Windows 10 or 11, 64-bit.\n\n\
             {details}\n\n\
             Open the download now?"
        ),
        true,
    )
}

/// Shown instead of the window when WebView2 is missing. Returns when the
/// user has closed the message; the launcher then quits.
#[cfg(windows)]
pub fn explain_missing(error: &str) {
    use windows::core::{w, HSTRING, PCWSTR};
    use windows::Win32::UI::Shell::ShellExecuteW;
    use windows::Win32::UI::WindowsAndMessaging::{
        MessageBoxW, IDYES, MB_ICONERROR, MB_OK, MB_SETFOREGROUND, MB_YESNO, SW_SHOWNORMAL,
    };

    let (text, offer) = missing_message(&crate::system::os(), error, wine());
    let buttons = if offer { MB_YESNO } else { MB_OK };
    let answer = unsafe { MessageBoxW(None, &HSTRING::from(text), w!("NicoLauncher"), buttons | MB_ICONERROR | MB_SETFOREGROUND) };
    if offer && answer == IDYES {
        unsafe { ShellExecuteW(None, w!("open"), &HSTRING::from(DOWNLOAD), PCWSTR::null(), PCWSTR::null(), SW_SHOWNORMAL) };
    }
}

/// Wine's own ntdll has a function Windows' does not
#[cfg(windows)]
fn wine() -> bool {
    use windows::core::{s, w};
    use windows::Win32::System::LibraryLoader::{GetModuleHandleW, GetProcAddress};
    unsafe { GetModuleHandleW(w!("ntdll.dll")).ok().and_then(|ntdll| GetProcAddress(ntdll, s!("wine_get_version"))).is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_gets_the_right_download_and_the_details() {
        let (text, offer) = missing_message("Windows 10 Pro 22H2 (build 19045)", "The system cannot find the file specified. (0x80070002)\n", false);
        assert!(offer);
        assert!(text.contains("Evergreen Bootstrapper"));
        assert!(text.contains("Fixed Version"));
        assert!(text.ends_with("Open the download now?"));
        assert!(text.contains("Details: Windows 10 Pro 22H2 (build 19045) · The system cannot find the file specified. (0x80070002)\n"));
    }

    #[test]
    fn wine_is_told_it_is_not_supported() {
        let (text, offer) = missing_message("unknown", "0x80004005", true);
        assert!(!offer);
        assert!(text.contains("Wine or Proton"));
        assert!(!text.contains("Open the download"));
        assert!(text.ends_with("Details: unknown · 0x80004005"));
    }
}
