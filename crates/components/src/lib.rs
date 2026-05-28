//! gpui-component-style component library for [`egui`] 0.34, ported from
//! [`gpui-component`](https://github.com/longbridge/gpui-component).
//!
//! All components are idiomatic egui widgets that return [`egui::Response`].
//! Most can be added with `ui.add(Button::primary("OK"))`; a few that need
//! to return extra info (e.g. [`Tag::show`]) provide a `.show(ui)` method.
//!
//! Theming comes from [`egui_components_theme::Theme`]. Install it once at
//! startup:
//!
//! ```no_run
//! # let ctx = egui::Context::default();
//! egui_components_theme::Theme::dark().install(&ctx);
//! ```
//!
//! Components read the installed theme via [`egui_components_theme::Theme::get`].

pub mod alert;
pub mod badge;
pub mod button;
pub mod checkbox;
pub mod common;
pub mod input;
pub mod label;
pub mod separator;
pub mod slider;
pub mod switch;
pub mod tag;

pub use alert::Alert;
pub use badge::Badge;
pub use button::Button;
pub use checkbox::Checkbox;
pub use common::{Size, Variant};
pub use input::Input;
pub use label::{Label, LabelTone};
pub use separator::Separator;
pub use slider::Slider;
pub use switch::Switch;
pub use tag::{Tag, TagResponse};

pub use egui_components_theme as theme;
