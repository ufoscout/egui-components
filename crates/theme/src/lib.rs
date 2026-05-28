//! gpui-component-style theme for [`egui`], ported from
//! [`gpui-component`](https://github.com/longbridge/gpui-component).
//!
//! # Quick start
//!
//! ```no_run
//! use egui_components_theme::Theme;
//!
//! # let ctx = egui::Context::default();
//! Theme::dark().install(&ctx);
//! ```
//!
//! Components in the sibling `egui-components` crate read the theme
//! from [`egui::Context::data`] via [`Theme::get`], so installing it once at
//! startup (or whenever the user toggles light/dark) is all that's required.

pub mod palette;
pub mod theme;
pub mod tokens;

pub use palette::Scale;
pub use theme::{mix, Theme, ThemeMetrics, ThemeMode};
pub use tokens::ThemeColor;
