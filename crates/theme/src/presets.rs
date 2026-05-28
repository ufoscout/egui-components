//! Bundled themes generated at compile time from
//! [longbridge/gpui-component's `themes/*.json`](https://github.com/longbridge/gpui-component/tree/main/themes).
//!
//! Each entry is a `pub const NAME: Theme = …;`. Use [`ALL`] to enumerate
//! every bundled theme, or [`by_name`] for runtime lookup.

use crate::theme::{Theme, ThemeMode};
use crate::tokens::ThemeColor;
use egui::Color32;

/// A bundled theme alongside the file-stem grouping it came from (`family`),
/// e.g. `"catppuccin"` for all four Catppuccin variants. Useful for finding
/// the light/dark counterpart of a chosen theme.
#[derive(Clone, Copy, Debug)]
pub struct Preset {
    pub name: &'static str,
    pub family: &'static str,
    pub theme: Theme,
}

include!(concat!(env!("OUT_DIR"), "/presets_generated.rs"));
