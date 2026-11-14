//! NicoLauncher engine. No UI here: the Tauri app (and later a command line)
//! call into this crate.
//!
//! What it never does: download, search for or link to game ROMs, and upload
//! or share anything it builds. A project's own file (its ROM) is only ever a
//! path on the user's disk, picked by the user and handed to the project's
//! own scripts.

pub mod achievements;
pub mod announcements;
pub mod build;
pub mod config;
pub mod kit;
pub mod manifest;
pub mod news;
pub mod project;
pub mod registry;
pub mod requirements;
pub mod runner;
pub mod source;
pub mod system;
pub mod update;
pub mod video;

/// Sent with every HTTP request. Generic on purpose: nothing about the user.
pub const USER_AGENT: &str = "NicoLauncher";

pub type Result<T> = std::result::Result<T, String>;

pub(crate) fn err<E: std::fmt::Display>(what: &str) -> impl FnOnce(E) -> String + '_ {
    move |e| format!("{what}: {e}")
}

pub(crate) fn agent(read_timeout_secs: u64) -> ureq::Agent {
    ureq::AgentBuilder::new()
        .timeout_connect(std::time::Duration::from_secs(20))
        .timeout_read(std::time::Duration::from_secs(read_timeout_secs))
        .user_agent(USER_AGENT)
        .try_proxy_from_env(true)
        .build()
}

pub(crate) fn http_error(url: &str, e: ureq::Error) -> String {
    match e {
        ureq::Error::Status(403 | 429, _) if url.contains("api.github.com") => {
            "GitHub refused the request: too many checks from this connection in the last hour. Try again later.".to_string()
        }
        ureq::Error::Status(code, _) => format!("{url} answered {code}"),
        other => format!("Could not reach {url}: {other}"),
    }
}

pub(crate) fn http_get(url: &str) -> Result<ureq::Response> {
    agent(60).get(url).call().map_err(|e| http_error(url, e))
}
