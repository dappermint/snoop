//! Colour schemes: two built in, the rest loaded from files.
//!
//! [`Palette`] describes *what* a colour is used for; a [`Theme`] here
//! decides *which* colours fill it. Keeping the schemes out of `theme.rs`
//! means that file can stay close to upstream: a merge that adds a palette
//! field touches `theme.rs`, and each scheme below gains one line rather
//! than the whole palette conflicting.
//!
//! A theme file is the flat half of Alacritty's colour config: one
//! `key = "#rrggbb"` per line, `#` starts a comment outside quotes, and
//! `[section]` headers are skipped. The key names are this palette's own
//! rather than Alacritty's, and an unrecognised one is reported and
//! skipped. Anything the file leaves out keeps the built-in value, so a
//! file that only sets `accent` is a valid theme.
//!
//! ```text
//! # ~/.config/snoop/themes/my-theme.toml
//! dark   = true
//! window = "#22212c"
//! accent = "#9580ff"   # the colour buttons and progress use
//! ```

use std::path::{Path, PathBuf};

use egui::Color32;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::theme::Palette;

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb(
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    )
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    /// The original open-source Dracula, MIT licensed.
    #[default]
    Dracula,
    /// Upstream Fastpotify's green.
    Spotify,
    /// `<name>.toml` in the themes directory.
    Custom(String),
}

impl Theme {
    /// The schemes that need no file on disk.
    pub const BUILT_IN: [Theme; 2] = [Self::Dracula, Self::Spotify];

    pub fn name(&self) -> &str {
        match self {
            Self::Dracula => "dracula",
            Self::Spotify => "spotify",
            Self::Custom(name) => name,
        }
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Dracula => "Dracula",
            Self::Spotify => "Spotify",
            Self::Custom(name) => name,
        }
    }

    /// Never fails: a name with no built-in and no file yet is still worth
    /// keeping in settings.json, so someone can write the file afterwards
    /// rather than finding their choice silently reset.
    pub fn from_name(name: &str) -> Self {
        match name {
            "dracula" => Self::Dracula,
            "spotify" => Self::Spotify,
            other => Self::Custom(other.to_owned()),
        }
    }

    /// The palette to draw with, falling back to the default scheme when a
    /// custom theme is missing or unreadable.
    pub fn palette(&self, dark: bool, themes_dir: &Path) -> Palette {
        let base = if dark {
            Palette::dark()
        } else {
            Palette::light()
        };
        match self {
            Self::Spotify => base,
            Self::Dracula => dracula(base),
            Self::Custom(name) => match self.file(themes_dir) {
                None => {
                    log::warn!("theme name {name:?} is not a usable file name");
                    Theme::default().palette(dark, themes_dir)
                }
                Some(path) => match std::fs::read_to_string(&path) {
                    Err(error) => {
                        log::warn!("cannot read theme {}: {error}", path.display());
                        Theme::default().palette(dark, themes_dir)
                    }
                    Ok(text) => match parse(&text, base) {
                        Err(error) => {
                            log::warn!("theme {} is not usable: {error}", path.display());
                            Theme::default().palette(dark, themes_dir)
                        }
                        Ok(loaded) => {
                            for key in &loaded.unknown_keys {
                                log::warn!("theme {} sets unknown key {key:?}", path.display());
                            }
                            loaded.palette
                        }
                    },
                },
            },
        }
    }

    /// The file this scheme reads, or `None` when it has none or the name
    /// could escape the themes directory.
    pub fn file(&self, themes_dir: &Path) -> Option<PathBuf> {
        let Self::Custom(name) = self else {
            return None;
        };
        // The name reaches here from settings.json, which a user edits by
        // hand, so it is untrusted input on its way to a path.
        let unsafe_name = name.is_empty()
            || name.contains(['/', '\\'])
            || name.contains("..")
            || Path::new(name).components().count() != 1;
        (!unsafe_name).then(|| themes_dir.join(format!("{name}.toml")))
    }

    /// Every `<name>.toml` in the themes directory, sorted, so the settings
    /// page can list them without touching the disk while drawing.
    pub fn discover(themes_dir: &Path) -> Vec<Theme> {
        let Ok(entries) = std::fs::read_dir(themes_dir) else {
            return Vec::new();
        };
        let mut found: Vec<Theme> = entries
            .flatten()
            .filter(|entry| entry.path().extension().is_some_and(|ext| ext == "toml"))
            .filter_map(|entry| {
                let stem = entry.path().file_stem()?.to_str()?.to_owned();
                match Theme::from_name(&stem) {
                    Theme::Custom(name) => Some(Theme::Custom(name)),
                    // A file called dracula.toml would otherwise appear
                    // twice and shadow the built-in in a confusing way.
                    _ => None,
                }
            })
            .collect();
        found.sort_by(|a, b| a.name().cmp(b.name()));
        found
    }
}

impl Serialize for Theme {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.name())
    }
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_name(&String::deserialize(deserializer)?))
    }
}

/// A parsed theme file: the palette it describes, and the keys it set that
/// mean nothing, which are worth telling someone about rather than dropping.
#[derive(Debug)]
pub struct Loaded {
    pub palette: Palette,
    pub unknown_keys: Vec<String>,
}

/// Reads `key = value` lines over `base`.
pub fn parse(text: &str, base: Palette) -> Result<Loaded, String> {
    let mut palette = base;
    let mut unknown_keys = Vec::new();

    for (index, raw) in text.lines().enumerate() {
        let line = strip_comment(raw).trim();
        if line.is_empty() || line.starts_with('[') {
            continue;
        }
        let number = index + 1;
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("line {number}: expected `key = value`"));
        };
        let (key, value) = (key.trim(), value.trim());

        if key == "dark" {
            palette.dark = match value {
                "true" => true,
                "false" => false,
                other => {
                    return Err(format!(
                        "line {number}: dark is true or false, not {other:?}"
                    ));
                }
            };
            continue;
        }

        let Some(slot) = slot(&mut palette, key) else {
            unknown_keys.push(key.to_owned());
            continue;
        };
        // An unquoted `#9580ff` looks like a comment and leaves nothing
        // behind, which is worth naming rather than reporting as empty.
        if value.is_empty() {
            return Err(format!(
                "line {number}: {key} has no value (quote it, as \"#rrggbb\")"
            ));
        }
        *slot = parse_color(value)
            .ok_or_else(|| format!("line {number}: {key} is not a colour: {value}"))?;
    }

    Ok(Loaded {
        palette,
        unknown_keys,
    })
}

/// `#` opens a comment, but only outside the quotes a colour lives in.
fn strip_comment(line: &str) -> &str {
    let mut quoted = false;
    for (index, character) in line.char_indices() {
        match character {
            '"' => quoted = !quoted,
            '#' if !quoted => return &line[..index],
            _ => {}
        }
    }
    line
}

fn parse_color(value: &str) -> Option<Color32> {
    let value = value.trim().trim_matches('"').trim();
    let digits = value
        .strip_prefix('#')
        .or_else(|| value.strip_prefix("0x"))
        .unwrap_or(value);
    if !digits.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let byte = |at: usize| u8::from_str_radix(&digits[at..at + 2], 16).ok();
    match digits.len() {
        6 => Some(Color32::from_rgb(byte(0)?, byte(2)?, byte(4)?)),
        8 => Some(Color32::from_rgba_unmultiplied(
            byte(0)?,
            byte(2)?,
            byte(4)?,
            byte(6)?,
        )),
        _ => None,
    }
}

/// The palette field a theme file key names.
fn slot<'p>(palette: &'p mut Palette, key: &str) -> Option<&'p mut Color32> {
    Some(match key {
        "window" => &mut palette.window,
        "panel" => &mut palette.panel,
        "surface" => &mut palette.surface,
        "surface_hover" => &mut palette.surface_hover,
        "surface_active" => &mut palette.surface_active,
        "outline" => &mut palette.outline,
        "text" => &mut palette.text,
        "secondary" => &mut palette.secondary,
        "dim" => &mut palette.dim,
        "accent" => &mut palette.accent,
        "accent_hover" => &mut palette.accent_hover,
        "on_accent" => &mut palette.on_accent,
        "danger" => &mut palette.danger,
        "warning" => &mut palette.warning,
        "overlay" => &mut palette.overlay,
        "shadow" => &mut palette.shadow,
        _ => return None,
    })
}

/// The original open-source Dracula, which is MIT licensed and so can ship
/// in the binary. Dracula Pro is a paid product: it belongs in a file of
/// your own rather than in this repository.
fn dracula(base: Palette) -> Palette {
    if !base.dark {
        // Dracula is a dark scheme with no official light counterpart, so
        // the light mode keeps its accent and nothing else.
        return Palette {
            accent: rgb(0xbd93f9),
            accent_hover: rgb(0xd0aeff),
            ..base
        };
    }
    Palette {
        dark: true,
        window: rgb(0x282a36),
        panel: rgb(0x21222c),
        surface: rgb(0x343746),
        surface_hover: rgb(0x424450),
        surface_active: rgb(0x4d4f68),
        outline: rgb(0x44475a),
        text: rgb(0xf8f8f2),
        secondary: rgb(0xc8c9cd),
        dim: rgb(0x6272a4),
        accent: rgb(0xbd93f9),
        accent_hover: rgb(0xd0aeff),
        on_accent: rgb(0x282a36),
        danger: rgb(0xff5555),
        warning: rgb(0xffb86c),
        overlay: rgb(0x343746),
        shadow: Color32::from_black_alpha(140),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("snoop-themes-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("the themes directory");
        dir
    }

    /// The name is what settings.json stores, so a rename resets everyone's
    /// choice on upgrade.
    #[test]
    fn names_survive_a_round_trip() {
        for theme in [
            Theme::Dracula,
            Theme::Spotify,
            Theme::Custom("my-theme".to_owned()),
        ] {
            let json = serde_json::to_string(&theme).expect("serialise");
            assert_eq!(
                serde_json::from_str::<Theme>(&json).expect("deserialise"),
                theme
            );
        }
        assert_eq!(
            serde_json::to_string(&Theme::Dracula).unwrap(),
            "\"dracula\""
        );
    }

    /// A theme that is not installed yet keeps its name rather than taking
    /// every other preference in the file down with it.
    #[test]
    fn an_unknown_theme_keeps_the_rest_of_the_settings() {
        let settings: crate::settings::Settings =
            serde_json::from_str(r#"{"color_theme":"nosferatu","volume":123}"#)
                .expect("settings still load");
        assert_eq!(settings.color_theme, Theme::Custom("nosferatu".to_owned()));
        assert_eq!(settings.volume, 123);
    }

    /// The `#` that opens a comment is the same character a colour starts
    /// with, so the obvious "cut at the first #" loses every colour.
    #[test]
    fn a_hash_inside_quotes_is_a_colour_not_a_comment() {
        let loaded = parse(
            r##"
            # a leading comment
            accent = "#9580ff"   # the accent, with a trailing comment
            window = "#22212c"
            "##,
            Palette::dark(),
        )
        .expect("parses");
        assert_eq!(loaded.palette.accent, rgb(0x9580ff));
        assert_eq!(loaded.palette.window, rgb(0x22212c));
        assert!(loaded.unknown_keys.is_empty());
    }

    /// Every key the file leaves out keeps the value it had, so a one-line
    /// theme is a valid theme.
    #[test]
    fn absent_keys_keep_the_base_colours() {
        let base = Palette::dark();
        let loaded = parse("accent = \"#ff0000\"", base).expect("parses");
        assert_eq!(loaded.palette.accent, rgb(0xff0000));
        assert_eq!(loaded.palette.window, base.window);
        assert_eq!(loaded.palette.text, base.text);
    }

    #[test]
    fn the_accepted_colour_spellings_agree() {
        for spelling in ["\"#9580ff\"", "\"0x9580ff\"", "\"9580ff\"", "9580ff"] {
            let text = format!("accent = {spelling}");
            let loaded = parse(&text, Palette::dark())
                .unwrap_or_else(|error| panic!("{spelling} should parse: {error}"));
            assert_eq!(loaded.palette.accent, rgb(0x9580ff), "{spelling}");
        }
        let alpha = parse("shadow = \"#0000008c\"", Palette::dark()).expect("parses");
        assert_eq!(alpha.palette.shadow, Color32::from_black_alpha(140));
    }

    /// A typo in a key should be visible rather than silently doing
    /// nothing, but must not stop the rest of the theme loading.
    #[test]
    fn an_unknown_key_is_reported_and_skipped() {
        let loaded = parse(
            "acccent = \"#ff0000\"\naccent = \"#00ff00\"",
            Palette::dark(),
        )
        .expect("parses");
        assert_eq!(loaded.unknown_keys, vec!["acccent".to_owned()]);
        assert_eq!(loaded.palette.accent, rgb(0x00ff00));
    }

    #[test]
    fn a_broken_theme_says_which_line() {
        for (text, expected) in [
            ("accent = \"#12345\"", "line 1"),
            ("accent = \"not a colour\"", "line 1"),
            ("\n\naccent", "line 3"),
            ("dark = maybe", "line 1"),
            // Unquoted, so the whole value reads as a comment.
            ("accent = #9580ff", "line 1"),
        ] {
            let error = parse(text, Palette::dark()).expect_err(text);
            assert!(error.starts_with(expected), "{text:?} said {error:?}");
        }
    }

    /// The name comes from a hand-edited settings file and is joined onto a
    /// path, so it must not be able to point outside the themes directory.
    #[test]
    fn a_theme_name_cannot_escape_the_themes_directory() {
        let dir = temp_dir("escape");
        for name in ["../secrets", "..", "a/b", "a\\b", ""] {
            assert_eq!(
                Theme::Custom(name.to_owned()).file(&dir),
                None,
                "{name:?} should be refused"
            );
        }
        assert_eq!(
            Theme::Custom("ok".to_owned()).file(&dir),
            Some(dir.join("ok.toml"))
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A missing or unreadable theme must still draw something.
    #[test]
    fn a_missing_theme_falls_back_to_the_default() {
        let dir = temp_dir("missing");
        let fallback = Theme::default().palette(true, &dir);
        assert_eq!(
            Theme::Custom("nope".to_owned()).palette(true, &dir),
            fallback
        );
        std::fs::write(dir.join("broken.toml"), "accent = \"nonsense\"").expect("write");
        assert_eq!(
            Theme::Custom("broken".to_owned()).palette(true, &dir),
            fallback
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_theme_file_is_found_and_applied() {
        let dir = temp_dir("found");
        std::fs::write(dir.join("mine.toml"), "accent = \"#010203\"").expect("write");
        std::fs::write(dir.join("notes.txt"), "ignored").expect("write");
        // Shadowing a built-in name would list the same label twice.
        std::fs::write(dir.join("dracula.toml"), "accent = \"#040506\"").expect("write");

        assert_eq!(
            Theme::discover(&dir),
            vec![Theme::Custom("mine".to_owned())]
        );
        assert_eq!(
            Theme::Custom("mine".to_owned()).palette(true, &dir).accent,
            rgb(0x010203)
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The built-in schemes must fill a whole palette, not leave half of
    /// another scheme's colours showing.
    #[test]
    fn the_built_in_schemes_are_complete() {
        let dir = temp_dir("built-in");
        for theme in Theme::BUILT_IN {
            let dark = theme.palette(true, &dir);
            assert!(dark.dark, "{} is not dark", theme.label());
            assert!(!theme.palette(false, &dir).dark, "{}", theme.label());
            assert_eq!(theme.file(&dir), None, "{} needs no file", theme.label());
        }
        assert_ne!(
            Theme::Dracula.palette(true, &dir).accent,
            Theme::Spotify.palette(true, &dir).accent
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
