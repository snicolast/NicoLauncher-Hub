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

/// The certificates HTTPS trusts: Mozilla's list, built in, plus whatever the
/// system has. Only the system's list failed on some PCs (issue: "invalid peer
/// certificate: UnknownIssuer"): Windows adds a root to its list only when one
/// of its own programs first needs it, so GitHub's could be missing. The
/// system's list is still read for what Mozilla does not have: antivirus and
/// company proxies that check HTTPS put their own certificate there.
fn tls_config() -> std::sync::Arc<rustls::ClientConfig> {
    static CONFIG: std::sync::OnceLock<std::sync::Arc<rustls::ClientConfig>> = std::sync::OnceLock::new();
    CONFIG
        .get_or_init(|| {
            let mut roots = rustls::RootCertStore { roots: webpki_roots::TLS_SERVER_ROOTS.to_vec() };
            if let Ok(system) = rustls_native_certs::load_native_certs() {
                roots.add_parsable_certificates(system);
            }
            let config = rustls::ClientConfig::builder_with_provider(rustls::crypto::ring::default_provider().into())
                .with_protocol_versions(&[&rustls::version::TLS12, &rustls::version::TLS13])
                .expect("ring supports TLS 1.2 and 1.3")
                .with_root_certificates(roots)
                .with_no_client_auth();
            std::sync::Arc::new(config)
        })
        .clone()
}

pub(crate) fn agent(read_timeout_secs: u64) -> ureq::Agent {
    ureq::AgentBuilder::new()
        .tls_config(tls_config())
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
