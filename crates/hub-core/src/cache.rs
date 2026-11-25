//! What was last read from GitHub and YouTube (the news and the latest
//! video), kept on disk so the next start shows it at once, before the
//! network answers. It is only a stand-in: the fresh copy replaces it as soon
//! as it comes, and a missing or damaged file just means nothing to show yet.

use crate::news::Entry;
use crate::video::Video;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Cache {
    /// The launcher's own news (the Hub's news.txt).
    pub launcher_news: Vec<Entry>,
    /// Each project's news on GitHub, by project id.
    pub news: HashMap<String, Vec<Entry>>,
    pub latest_video: Option<Video>,
}

/// Next to the settings: AppData, or the launcher's folder when portable.
pub fn file_next_to(config: &Path) -> PathBuf {
    config.with_file_name("cache.json")
}

pub fn load(path: &Path) -> Cache {
    std::fs::read(path).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default()
}

/// Written aside and then moved over the old file, so a launcher closed
/// halfway never leaves half a file. Nothing is lost if it cannot be written.
pub fn save(path: &Path, cache: &Cache) {
    let Ok(bytes) = serde_json::to_vec(cache) else { return };
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let tmp = path.with_extension("json.tmp");
    if std::fs::write(&tmp, bytes).is_ok() && std::fs::rename(&tmp, path).is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::news;

    fn dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("hub-cache-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn what_is_saved_comes_back() {
        let d = dir("roundtrip");
        let path = file_next_to(&d.join("config.json"));
        let mut cache = Cache { launcher_news: news::parse("[2] 2026-11-25\n- Launcher 0.3.23: faster news\n"), ..Cache::default() };
        cache.news.insert("segarally2".into(), news::parse("[30] 2026-11-24\n- a fix\n"));
        save(&path, &cache);
        assert_eq!(load(&path), cache);
        assert!(!path.with_extension("json.tmp").exists());
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn a_missing_or_damaged_file_is_empty() {
        let d = dir("damaged");
        let path = file_next_to(&d.join("config.json"));
        assert_eq!(load(&path), Cache::default());
        std::fs::create_dir_all(&d).unwrap();
        std::fs::write(&path, b"{ not json").unwrap();
        assert_eq!(load(&path), Cache::default());
        let _ = std::fs::remove_dir_all(&d);
    }
}
