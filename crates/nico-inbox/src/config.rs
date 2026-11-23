//! config.toml, next to the exe (or the path given with --config). It holds
//! the only secret of the mailbox, the ntfy topic: it stays on the server.

use serde::Deserialize;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Where launchers send to. Tailscale Funnel publishes this one.
    #[serde(default = "default_public")]
    pub public: String,
    /// The panel to read the messages. Only for your own machines: Tailscale
    /// serve (without funnel) points at it.
    #[serde(default = "default_panel")]
    pub panel: String,
    /// Messages, blocked senders and the log. Relative to the config file.
    #[serde(default = "default_data")]
    pub data: PathBuf,
    /// How hard the launcher's sum is (see pow.rs). 20 takes a PC well under a
    /// second; each one more doubles it.
    #[serde(default = "default_bits")]
    pub pow_bits: u32,
    /// Use the first address of X-Forwarded-For as the sender's address. Right
    /// behind Tailscale or another proxy on the same machine; wrong if the
    /// public port is reachable directly.
    #[serde(default = "yes")]
    pub trust_forwarded: bool,
    #[serde(default)]
    pub limits: Limits,
    #[serde(default)]
    pub ntfy: Option<Ntfy>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limits {
    /// Messages one sender (address or launcher) may send in an hour, and in a day.
    pub per_hour: u32,
    pub per_day: u32,
    /// Messages kept from everyone together in a day. Past it, launchers are
    /// told to try later (they keep the message and send it later).
    pub global_per_day: u32,
}

impl Default for Limits {
    fn default() -> Self {
        Limits { per_hour: 5, per_day: 20, global_per_day: 300 }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ntfy {
    /// The topic's full address, e.g. https://ntfy.sh/<long random name>.
    /// Anyone who knows it can read the notices: keep it to yourself.
    pub url: String,
    /// An access token, for a reserved topic or your own ntfy server.
    #[serde(default)]
    pub token: Option<String>,
    /// The whole message in the notice. Off: only its kind, game and title.
    #[serde(default)]
    pub full_text: bool,
    /// Opened when you tap the notice: the panel's address in your tailnet.
    #[serde(default)]
    pub click: Option<String>,
}

fn default_public() -> String {
    "127.0.0.1:8787".into()
}
fn default_panel() -> String {
    "127.0.0.1:8788".into()
}
fn default_data() -> PathBuf {
    "data".into()
}
fn default_bits() -> u32 {
    20
}
fn yes() -> bool {
    true
}

impl Config {
    pub fn load(path: &Path) -> Result<Config, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        let mut config: Config = toml::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
        if config.data.is_relative() {
            config.data = path.parent().unwrap_or(Path::new(".")).join(&config.data);
        }
        Ok(config)
    }
}
