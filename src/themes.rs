//! Named colour schemes.
//!
//! [`Palette`] describes *what* a colour is used for;
//! a [`Theme`] here decides *which* colours fill it. Keeping the schemes in
//! their own file means `theme.rs` can stay close to upstream: a merge that
//! adds a palette field touches `theme.rs`, and each scheme below gains one
//! line rather than the whole palette conflicting.
//!
//! Adding a scheme means a variant, an entry in [`Theme::ALL`], and one
//! constructor returning a filled `Palette`.

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::theme::Palette;

const fn rgb(hex: u32) -> Color32 {
    Color32::from_rgb(
        ((hex >> 16) & 0xff) as u8,
        ((hex >> 8) & 0xff) as u8,
        (hex & 0xff) as u8,
    )
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[serde(from = "UnknownIsDefault")]
pub enum Theme {
    #[default]
    DraculaPro,
    Dracula,
    Spotify,
}

/// A theme name written by a newer build, or by hand, must not take the
/// whole settings file down with it: every other preference in the file is
/// still perfectly readable, and losing them all to one bad string is a far
/// worse outcome than falling back to the default scheme.
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum UnknownIsDefault {
    DraculaPro,
    Dracula,
    Spotify,
    #[serde(other)]
    Unknown,
}

impl From<UnknownIsDefault> for Theme {
    fn from(value: UnknownIsDefault) -> Self {
        match value {
            UnknownIsDefault::DraculaPro => Self::DraculaPro,
            UnknownIsDefault::Dracula => Self::Dracula,
            UnknownIsDefault::Spotify => Self::Spotify,
            UnknownIsDefault::Unknown => Self::default(),
        }
    }
}

impl Theme {
    pub const ALL: [Theme; 3] = [Self::DraculaPro, Self::Dracula, Self::Spotify];

    pub fn label(self) -> &'static str {
        match self {
            Self::DraculaPro => "Dracula Pro",
            Self::Dracula => "Dracula",
            Self::Spotify => "Spotify",
        }
    }

    /// The dark palette for this scheme.
    pub fn dark(self) -> Palette {
        match self {
            Self::DraculaPro => dracula_pro(),
            Self::Dracula => dracula(),
            Self::Spotify => Palette::dark(),
        }
    }

    /// The light palette for this scheme.
    ///
    /// The Dracula schemes are dark by design and have no official light
    /// counterpart, so they borrow upstream's light palette and keep only
    /// their accent, which is the colour a user actually recognises.
    pub fn light(self) -> Palette {
        match self {
            Self::Spotify => Palette::light(),
            other => {
                let dark = other.dark();
                Palette {
                    accent: dark.accent,
                    accent_hover: dark.accent_hover,
                    on_accent: Color32::WHITE,
                    ..Palette::light()
                }
            }
        }
    }
}

/// Dracula Pro (Van Helsing), the palette Snoop ships with.
fn dracula_pro() -> Palette {
    Palette {
        dark: true,
        window: rgb(0x22212c),
        panel: rgb(0x1e1d27),
        surface: rgb(0x282a36),
        surface_hover: rgb(0x383a4c),
        surface_active: rgb(0x454158),
        outline: rgb(0x454158),
        text: rgb(0xf8f8f2),
        secondary: rgb(0xc6c6c2),
        dim: rgb(0x7970a9),
        accent: rgb(0x9580ff),
        accent_hover: rgb(0xaa99ff),
        on_accent: rgb(0x22212c),
        danger: rgb(0xff9580),
        warning: rgb(0xffca80),
        overlay: rgb(0x282a36),
        shadow: Color32::from_black_alpha(140),
    }
}

/// The original open-source Dracula.
fn dracula() -> Palette {
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

    /// A scheme that leaves a palette field at some other scheme's value is
    /// the failure mode this module exists to prevent, and it is invisible
    /// until someone opens the app. Every scheme must differ from upstream's
    /// in the colours that define it, and agree with it on structure.
    #[test]
    fn every_theme_fills_the_whole_palette() {
        for theme in Theme::ALL {
            let dark = theme.dark();
            assert!(dark.dark, "{} dark palette is not dark", theme.label());

            let light = theme.light();
            assert!(!light.dark, "{} light palette is not light", theme.label());
            if theme != Theme::Spotify {
                assert_eq!(
                    light.accent,
                    dark.accent,
                    "{} accent changes between light and dark",
                    theme.label()
                );
            }

            if theme != Theme::Spotify {
                assert_ne!(
                    dark.accent,
                    Palette::dark().accent,
                    "{} still carries upstream's accent",
                    theme.label()
                );
                assert_ne!(
                    dark.window,
                    Palette::dark().window,
                    "{} still carries upstream's window colour",
                    theme.label()
                );
            }
        }
    }

    /// The setting is serialised into settings.json, so a rename would
    /// silently reset every user to the default on upgrade.
    #[test]
    fn theme_names_survive_a_round_trip() {
        for theme in Theme::ALL {
            let json = serde_json::to_string(&theme).expect("serialise");
            let back: Theme = serde_json::from_str(&json).expect("deserialise");
            assert_eq!(theme, back, "{json} did not round-trip");
        }
        assert_eq!(
            serde_json::to_string(&Theme::DraculaPro).unwrap(),
            "\"dracula-pro\""
        );
    }

    /// An unknown scheme in settings.json (written by a newer build, or by
    /// hand) must fall back rather than refuse to start.
    #[test]
    fn an_unknown_theme_is_not_fatal() {
        assert_eq!(
            serde_json::from_str::<Theme>("\"nosferatu\"").expect("falls back"),
            Theme::default()
        );
        let settings: crate::settings::Settings =
            serde_json::from_str(r#"{"color_theme":"nosferatu","volume":123}"#)
                .expect("settings still load");
        assert_eq!(settings.color_theme, Theme::default());
        assert_eq!(settings.volume, 123, "the rest of the file survived");
    }
}
