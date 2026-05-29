//! `Card` — a bordered, rounded surface that groups related content.
//!
//! Doubles as gpui-component's section/`GroupBox` container: pass an optional
//! [`title`](Card::title) / [`description`](Card::description) to render a
//! header, then add the body in the `show` closure, just like
//! [`List`](crate::list::List).
//!
//! ```ignore
//! sc::Card::new()
//!     .title("Account")
//!     .description("Manage your profile settings.")
//!     .show(ui, |ui| {
//!         ui.add(sc::Input::new(&mut name));
//!     });
//! ```

use egui::{Frame, InnerResponse, Margin, Ui};
use egui_components_theme::Theme;

use crate::common::Size;
use crate::label::Label;
use crate::separator::Separator;

/// A surface container with an optional header (title + description).
pub struct Card {
    title: Option<String>,
    description: Option<String>,
    padding: f32,
    /// Draw a separator between the header and the body.
    divider: bool,
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            padding: 16.0,
            divider: false,
        }
    }

    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }

    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    /// Render a horizontal separator between the header and the body.
    pub fn divider(mut self) -> Self {
        self.divider = true;
        self
    }

    /// Render the card frame and run `body` inside it, returning the body's
    /// value alongside the frame [`egui::Response`].
    pub fn show<R>(self, ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;

        Frame::new()
            .fill(c.background)
            .stroke(theme.border_stroke())
            .corner_radius(theme.corner_lg())
            .inner_margin(Margin::same(self.padding as i8))
            .show(ui, |ui| {
                let has_header = self.title.is_some() || self.description.is_some();
                if let Some(title) = self.title {
                    ui.add(Label::new(title).strong().size(Size::Large));
                }
                if let Some(desc) = self.description {
                    ui.add(Label::new(desc).muted().size(Size::Small));
                }
                if has_header {
                    if self.divider {
                        ui.add_space(self.padding * 0.5);
                        ui.add(Separator::horizontal());
                    }
                    ui.add_space(self.padding * 0.75);
                }
                body(ui)
            })
    }
}
