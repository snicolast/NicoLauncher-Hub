//! Pictures the user picked for a project: a cover (the library card, the
//! sidebar, the strip on Home), a background (the project's page, Home's
//! spotlight, the blurred picture behind the window) and a logo (shown over
//! them in place of the project's name).
//!
//! The picture is copied into the launcher's own `art` folder, made smaller and
//! saved as a JPEG (a logo as a PNG, to keep what is see-through), instead of pointing at the user's file: a file that is
//! moved, deleted or on a USB drive that is not plugged in would leave the card
//! empty, and a photo straight from a camera is heavy to draw. A portable
//! launcher keeps the folder next to its exe, like its settings.
//!
//! Settings keep only the file's name and which part of the picture to show;
//! the folder never holds anything else, so a name is all the window asks for.

use crate::{err, http_get, Result};
use image::{DynamicImage, ImageReader, Rgb, RgbImage};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

/// The folder next to the settings file.
pub const DIR: &str = "art";
/// Bigger files are refused before they are read.
pub const MAX_BYTES: u64 = 50 * 1024 * 1024;
/// A picture is made to fit in this, keeping its shape: enough for the
/// launcher's window on a 4K screen, where the backgrounds are drawn at most
/// that big.
pub const MAX_WIDTH: u32 = 2560;
pub const MAX_HEIGHT: u32 = 1600;
/// The kinds of file the picker offers.
pub const EXTENSIONS: [&str; 4] = ["jpg", "jpeg", "png", "webp"];
/// A logo is made to fit in this: it is never drawn bigger than a few hundred
/// pixels, twice that on a 4K screen.
pub const LOGO_MAX_WIDTH: u32 = 1600;
pub const LOGO_MAX_HEIGHT: u32 = 800;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Cover,
    Background,
    Logo,
}

impl Kind {
    fn word(self) -> &'static str {
        match self {
            Kind::Cover => "cover",
            Kind::Background => "background",
            Kind::Logo => "logo",
        }
    }

    fn extension(self) -> &'static str {
        match self {
            Kind::Logo => "png",
            _ => "jpg",
        }
    }
}

/// A picture the user picked, as the settings keep it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Art {
    /// The file's name in the `art` folder.
    pub file: String,
    /// The point of the picture that stays in sight when a frame cuts it, as
    /// percents of its width and height: [50, 50] is the middle.
    #[serde(default = "middle")]
    pub focus: [u8; 2],
}

fn middle() -> [u8; 2] {
    [50, 50]
}

impl Art {
    /// The same picture, with a focus that is inside it.
    pub fn clean(mut self) -> Art {
        self.focus = self.focus.map(|v| v.min(100));
        self
    }
}

/// The `art` folder of settings kept in `settings_dir`.
pub fn dir(settings_dir: &Path) -> PathBuf {
    settings_dir.join(DIR)
}

/// A name the launcher could have given a file it made: lower case letters,
/// digits and dashes, then `.jpg` (or `.png`, a logo). Nothing else is read from, or deleted in,
/// the folder, so no path from the window can reach another file.
pub fn valid_name(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".jpg").or_else(|| name.strip_suffix(".png")) else { return false };
    !stem.is_empty() && stem.len() <= 120 && stem.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// The picture in `path` (a file on the user's PC), copied into `dir`.
/// Returns the new file's name.
pub fn from_file(dir: &Path, id: &str, kind: Kind, path: &Path) -> Result<String> {
    let size = std::fs::metadata(path).map_err(err("Could not read the picture"))?.len();
    if size > MAX_BYTES {
        return Err(format!("{} is too big ({} MB). Choose a picture under {} MB.", path.display(), size / (1024 * 1024), MAX_BYTES / (1024 * 1024)));
    }
    let bytes = std::fs::read(path).map_err(err("Could not read the picture"))?;
    save(dir, id, kind, &bytes)
}

/// The picture at `url` (one of the project's screenshots), copied into `dir`.
pub fn from_url(dir: &Path, id: &str, kind: Kind, url: &str) -> Result<String> {
    let mut bytes = Vec::new();
    http_get(url)?.into_reader().take(MAX_BYTES + 1).read_to_end(&mut bytes).map_err(err("Could not download the picture"))?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err(format!("{url} is too big."));
    }
    save(dir, id, kind, &bytes)
}

/// Reads the picture, makes it fit, and writes it to `dir` (a JPEG, or a PNG
/// for a logo) under a name of its own: the window shows a new picture at
/// once, without one it had loaded before under the same name.
pub fn save(dir: &Path, id: &str, kind: Kind, bytes: &[u8]) -> Result<String> {
    let jpeg = match kind {
        Kind::Logo => prepare_logo(bytes)?,
        _ => prepare(bytes)?,
    };
    std::fs::create_dir_all(dir).map_err(err("Could not create the pictures folder"))?;
    let name = new_name(id, kind);
    let tmp = dir.join(format!("{name}.tmp"));
    std::fs::write(&tmp, jpeg).map_err(err("Could not save the picture"))?;
    std::fs::rename(&tmp, dir.join(&name)).map_err(err("Could not save the picture"))?;
    Ok(name)
}

fn new_name(id: &str, kind: Kind) -> String {
    static COUNT: AtomicU32 = AtomicU32::new(0);
    let id: String = id.chars().map(|c| c.to_ascii_lowercase()).filter(|c| c.is_ascii_alphanumeric() || *c == '-').take(40).collect();
    let millis = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
    format!("{id}-{}-{millis}-{}.{}", kind.word(), COUNT.fetch_add(1, Ordering::Relaxed), kind.extension())
}

/// The picture as the JPEG that is kept: at most MAX_WIDTH x MAX_HEIGHT, and
/// a transparent one laid on the launcher's own dark blue.
pub fn prepare(bytes: &[u8]) -> Result<Vec<u8>> {
    let picture = decode(bytes)?;
    let picture = if picture.width() > MAX_WIDTH || picture.height() > MAX_HEIGHT {
        picture.resize(MAX_WIDTH, MAX_HEIGHT, image::imageops::FilterType::CatmullRom)
    } else {
        picture
    };
    let rgb = flatten(picture);
    let mut out = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 90).encode_image(&rgb).map_err(err("Could not save the picture"))?;
    Ok(out)
}

/// A logo as the PNG that is kept: what is see-through stays so, the empty
/// margin around it is cut off (so it lines up with the text beside it), and
/// it is made to fit in LOGO_MAX_WIDTH x LOGO_MAX_HEIGHT.
pub fn prepare_logo(bytes: &[u8]) -> Result<Vec<u8>> {
    let picture = decode(bytes)?.into_rgba8();
    let (w, h) = picture.dimensions();
    let mut found: Option<(u32, u32, u32, u32)> = None;
    for (x, y, p) in picture.enumerate_pixels() {
        if p.0[3] > 8 {
            let b = found.get_or_insert((x, y, x, y));
            *b = (b.0.min(x), b.1.min(y), b.2.max(x), b.3.max(y));
        }
    }
    let (x0, y0, x1, y1) = found.ok_or("That picture is all see-through: there is nothing in it to show.")?;
    let picture = if (x0, y0, x1, y1) != (0, 0, w - 1, h - 1) {
        image::imageops::crop_imm(&picture, x0, y0, x1 - x0 + 1, y1 - y0 + 1).to_image()
    } else {
        picture
    };
    let picture = DynamicImage::ImageRgba8(picture);
    let picture = if picture.width() > LOGO_MAX_WIDTH || picture.height() > LOGO_MAX_HEIGHT {
        picture.resize(LOGO_MAX_WIDTH, LOGO_MAX_HEIGHT, image::imageops::FilterType::CatmullRom)
    } else {
        picture
    };
    let mut out = Vec::new();
    picture.write_to(&mut Cursor::new(&mut out), image::ImageFormat::Png).map_err(err("Could not save the picture"))?;
    Ok(out)
}

/// The picture in `bytes`, of any kind the launcher reads.
fn decode(bytes: &[u8]) -> Result<DynamicImage> {
    let mut reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format().map_err(err("Could not read the picture"))?;
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(20_000);
    limits.max_image_height = Some(20_000);
    reader.limits(limits);
    let picture = reader.decode().map_err(|e| match e {
        image::ImageError::Unsupported(_) => "That is not a picture the launcher can read. Choose a JPG, PNG or WebP file.".to_string(),
        e => format!("Could not read the picture: {e}"),
    })?;
    Ok(picture)
}

/// The window's background colour (#0a0d16) under what is see-through.
fn flatten(picture: DynamicImage) -> RgbImage {
    if !picture.color().has_alpha() {
        return picture.into_rgb8();
    }
    const UNDER: [u32; 3] = [0x0a, 0x0d, 0x16];
    let rgba = picture.into_rgba8();
    RgbImage::from_fn(rgba.width(), rgba.height(), |x, y| {
        let p = rgba.get_pixel(x, y).0;
        let a = p[3] as u32;
        Rgb(std::array::from_fn(|i| ((p[i] as u32 * a + UNDER[i] * (255 - a) + 127) / 255) as u8))
    })
}

/// The bytes of a picture in `dir`, for the window. None for a name the
/// launcher would not have given, or a file that is not there.
pub fn read(dir: &Path, name: &str) -> Option<Vec<u8>> {
    if !valid_name(name) {
        return None;
    }
    std::fs::read(dir.join(name)).ok()
}

/// Deletes the pictures in `dir` that the settings no longer name: replaced,
/// put back to the default, or picked and then cancelled. Only files with a
/// name the launcher gives, and the half-written ones it leaves when it stops
/// while saving.
pub fn prune(dir: &Path, keep: &[&str]) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(name) = name.to_str() else { continue };
        let ours = valid_name(name) || name.strip_suffix(".tmp").is_some_and(valid_name);
        if ours && !keep.contains(&name) && entry.path().is_file() {
            let _ = std::fs::remove_file(entry.path());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(width: u32, height: u32, alpha: u8) -> Vec<u8> {
        let img = image::RgbaImage::from_pixel(width, height, image::Rgba([200, 100, 50, alpha]));
        let mut out = Vec::new();
        DynamicImage::ImageRgba8(img).write_to(&mut Cursor::new(&mut out), image::ImageFormat::Png).unwrap();
        out
    }

    fn temp(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("nicolauncher-art-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn names_the_window_may_ask_for() {
        assert!(valid_name("segarally2-cover-1763251200000-0.jpg"));
        assert!(!valid_name("../config.json"));
        assert!(!valid_name("..\\x.jpg"));
        assert!(!valid_name("a/b.jpg"));
        assert!(!valid_name("C:.jpg"));
        assert!(!valid_name(".jpg"));
        assert!(!valid_name("Cover.jpg"));
        assert!(valid_name("segarally2-logo-1763251200000-0.png"));
        assert!(!valid_name("cover.gif"));
        assert!(!valid_name(&format!("{}.jpg", "a".repeat(121))));
    }

    #[test]
    fn a_big_picture_is_made_to_fit_keeping_its_shape() {
        let jpeg = prepare(&png(4000, 1000, 255)).unwrap();
        let back = image::load_from_memory(&jpeg).unwrap();
        assert_eq!((back.width(), back.height()), (MAX_WIDTH, 640));
        let tall = image::load_from_memory(&prepare(&png(1000, 3200, 255)).unwrap()).unwrap();
        assert_eq!((tall.width(), tall.height()), (500, MAX_HEIGHT));
        let small = image::load_from_memory(&prepare(&png(300, 200, 255)).unwrap()).unwrap();
        assert_eq!((small.width(), small.height()), (300, 200), "a small one is not made bigger");
    }

    #[test]
    fn what_is_see_through_lies_on_the_window_colour() {
        let back = image::load_from_memory(&prepare(&png(16, 16, 0)).unwrap()).unwrap().into_rgb8();
        let p = back.get_pixel(8, 8).0;
        assert!(p[0] < 24 && p[1] < 26 && p[2] < 36, "{p:?}");
    }

    #[test]
    fn a_logo_keeps_what_is_see_through_and_loses_its_empty_margin() {
        // A 100x40 picture, see-through but for a 30x10 block
        let mut img = image::RgbaImage::from_pixel(100, 40, image::Rgba([0, 0, 0, 0]));
        for y in 12..22 {
            for x in 20..50 {
                img.put_pixel(x, y, image::Rgba([255, 255, 255, 255]));
            }
        }
        img.put_pixel(0, 0, image::Rgba([255, 0, 0, 3])); // too faint to count
        let mut png = Vec::new();
        DynamicImage::ImageRgba8(img).write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png).unwrap();
        let back = image::load_from_memory(&prepare_logo(&png).unwrap()).unwrap();
        assert_eq!((back.width(), back.height()), (30, 10));
        assert!(back.color().has_alpha());
        let big = image::load_from_memory(&prepare_logo(&png_bytes(4000, 1000, 255)).unwrap()).unwrap();
        assert_eq!((big.width(), big.height()), (LOGO_MAX_WIDTH, 400));
        assert!(prepare_logo(&png_bytes(10, 10, 0)).unwrap_err().contains("see-through"));
        let dir = temp("logo");
        let name = save(&dir, "segarally2", Kind::Logo, &png).unwrap();
        assert!(name.starts_with("segarally2-logo-") && name.ends_with(".png") && valid_name(&name), "{name}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn png_bytes(width: u32, height: u32, alpha: u8) -> Vec<u8> {
        png(width, height, alpha)
    }

    #[test]
    fn what_is_not_a_picture_is_refused() {
        let e = prepare(b"PK\x03\x04 not a picture at all").unwrap_err();
        assert!(e.contains("JPG, PNG or WebP") || e.contains("Could not read"), "{e}");
    }

    #[test]
    fn saved_read_and_pruned() {
        let dir = temp("prune");
        let a = save(&dir, "SegaRally2", Kind::Cover, &png(40, 30, 255)).unwrap();
        let b = save(&dir, "segarally2", Kind::Background, &png(40, 30, 255)).unwrap();
        assert_ne!(a, b);
        assert!(a.starts_with("segarally2-cover-") && valid_name(&a), "{a}");
        assert!(read(&dir, &a).is_some());
        assert!(read(&dir, "../x.jpg").is_none());
        std::fs::write(dir.join("notes.txt"), "mine").unwrap();
        std::fs::write(dir.join("old-cover-1-0.jpg.tmp"), "half").unwrap();
        prune(&dir, &[a.as_str()]);
        assert!(dir.join(&a).is_file());
        assert!(!dir.join(&b).exists());
        assert!(!dir.join("old-cover-1-0.jpg.tmp").exists());
        assert!(dir.join("notes.txt").is_file(), "only what the launcher made is deleted");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_too_big_file_is_not_read() {
        let dir = temp("big");
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("huge.png");
        let f = std::fs::File::create(&file).unwrap();
        f.set_len(MAX_BYTES + 1).unwrap();
        let e = from_file(&dir.join("art"), "x", Kind::Cover, &file).unwrap_err();
        assert!(e.contains("too big"), "{e}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn focus_is_kept_inside_the_picture() {
        let art: Art = serde_json::from_str(r#"{ "file": "a.jpg" }"#).unwrap();
        assert_eq!(art.focus, [50, 50]);
        assert_eq!(Art { file: "a.jpg".into(), focus: [250, 30] }.clean().focus, [100, 30]);
    }
}
