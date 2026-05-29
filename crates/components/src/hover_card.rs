//! `HoverCard` — a richer [`Tooltip`](crate::tooltip::Tooltip) that shows
//! arbitrary content on hover.
//!
//! ```ignore
//! let r = ui.add(sc::Link::new("@ada"));
//! sc::HoverCard::new().max_width(260.0).show(&r, |ui| {
//!     ui.horizontal(|ui| {
//!         ui.add(sc::Avatar::from_name("Ada Lovelace").small());
//!         ui.add(sc::Label::new("Ada Lovelace").strong());
//!     });
//!     ui.add(sc::Label::new("First programmer.").muted());
//! });
//! ```

use egui::{Response, Ui};

pub struct HoverCard {
    max_width: f32,
    at_pointer: bool,
}

impl Default for HoverCard {
    fn default() -> Self {
        Self::new()
    }
}

impl HoverCard {
    pub fn new() -> Self {
        Self {
            max_width: 300.0,
            at_pointer: false,
        }
    }
    pub fn max_width(mut self, w: f32) -> Self {
        self.max_width = w;
        self
    }
    /// Anchor to the pointer instead of the widget.
    pub fn at_pointer(mut self) -> Self {
        self.at_pointer = true;
        self
    }

    /// Show `content` while `response` is hovered. Returns the (possibly
    /// updated) [`Response`].
    pub fn show(self, response: &Response, content: impl FnOnce(&mut Ui)) -> Response {
        let max_width = self.max_width;
        let add = move |ui: &mut Ui| {
            ui.set_max_width(max_width);
            content(ui);
        };
        if self.at_pointer {
            response.clone().on_hover_ui_at_pointer(add)
        } else {
            response.clone().on_hover_ui(add)
        }
    }
}
