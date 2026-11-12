//! The author's latest video, for the home page: read from the channel's
//! public feed on YouTube. Only its id, title and date are kept.

use crate::{err, http_get, Result};
use serde::Serialize;

/// snicolast's channel (youtube.com/@snicolast).
pub const FEED_URL: &str = "https://www.youtube.com/feeds/videos.xml?channel_id=UCrmeVYYAwMFHdXGRVSP1rwg";

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Video {
    pub title: String,
    /// The day it was published, as YYYY-MM-DD.
    pub date: String,
    pub url: String,
    pub thumbnail: String,
}

fn between<'a>(text: &'a str, open: &str, close: &str) -> Option<&'a str> {
    let start = text.find(open)? + open.len();
    let end = text[start..].find(close)? + start;
    Some(&text[start..end])
}

fn unescape(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&#39;", "'")
        .replace("&amp;", "&")
}

/// The newest video of a channel feed. Shorts are passed over when there is
/// a regular video, as they show poorly as a wide picture. The id is checked
/// and the addresses are built here, so nothing from the feed is opened as is.
pub fn parse(feed: &str) -> Option<Video> {
    let mut first = None;
    for entry in feed.split("<entry>").skip(1) {
        let Some(id) = between(entry, "<yt:videoId>", "</yt:videoId>") else { continue };
        if id.len() != 11 || !id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
            continue;
        }
        let Some(title) = between(entry, "<title>", "</title>") else { continue };
        let date = between(entry, "<published>", "</published>").and_then(|d| d.get(..10)).unwrap_or("");
        let video = Video {
            title: unescape(title.trim()),
            date: date.to_string(),
            url: format!("https://www.youtube.com/watch?v={id}"),
            thumbnail: format!("https://i.ytimg.com/vi/{id}/mqdefault.jpg"),
        };
        if !entry.contains("youtube.com/shorts/") {
            return Some(video);
        }
        first.get_or_insert(video);
    }
    first
}

pub fn fetch_latest() -> Result<Option<Video>> {
    Ok(parse(&http_get(FEED_URL)?.into_string().map_err(err("Bad video feed"))?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, title: &str, link: &str) -> String {
        format!(
            "<entry><id>yt:video:{id}</id><yt:videoId>{id}</yt:videoId><title>{title}</title>\
             <link rel=\"alternate\" href=\"{link}\"/><published>2026-11-10T08:00:00+00:00</published></entry>"
        )
    }

    #[test]
    fn takes_the_newest_regular_video() {
        let feed = format!(
            "<feed><title>snicolast</title>{}{}{}</feed>",
            entry("aaaaaaaaaaa", "A short", "https://www.youtube.com/shorts/aaaaaaaaaaa"),
            entry("bbbbbbbb_-b", "Rally &amp; &quot;more&quot;", "https://www.youtube.com/watch?v=bbbbbbbb_-b"),
            entry("ccccccccccc", "Older", "https://www.youtube.com/watch?v=ccccccccccc"),
        );
        let v = parse(&feed).unwrap();
        assert_eq!(v.title, "Rally & \"more\"");
        assert_eq!(v.date, "2026-11-10");
        assert_eq!(v.url, "https://www.youtube.com/watch?v=bbbbbbbb_-b");
        assert_eq!(v.thumbnail, "https://i.ytimg.com/vi/bbbbbbbb_-b/mqdefault.jpg");
    }

    #[test]
    fn a_short_is_better_than_nothing_and_bad_ids_are_refused() {
        let feed = format!(
            "<feed>{}{}</feed>",
            entry("bad\"id/../x", "Nope", "https://www.youtube.com/watch?v=x"),
            entry("aaaaaaaaaaa", "A short", "https://www.youtube.com/shorts/aaaaaaaaaaa"),
        );
        assert_eq!(parse(&feed).unwrap().title, "A short");
        assert_eq!(parse("<feed></feed>"), None);
    }
}
