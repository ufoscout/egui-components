//! `Tooltip` — themed hover help attached to any widget [`Response`].
//!
//! egui already renders tooltips in a framed popover; this wrapper styles the
//! content with the component theme (foreground / muted colors, optional bold
//! title) so it matches the rest of the library. Attach it to whatever a
//! widget returns:
//!
//! ```ignore
//! let r = sc::Tooltip::new("Delete this item")
//!     .attach(ui.add(sc::Button::danger("Delete")));
//! ```

use egui::{Response, Ui};

use crate::common::Size;
use crate::label::Label;

pub struct Tooltip {
    title: Option<String>,
    text: String,
    at_pointer: bool,
}

impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            title: None,
            text: text.into(),
            at_pointer: false,
        }
    }

    /// Add a bold title line above the body text.
    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }

    /// Anchor the tooltip to the pointer instead of the widget.
    pub fn at_pointer(mut self) -> Self {
        self.at_pointer = true;
        self
    }

    /// Attach the tooltip to `response`, shown while the widget is hovered.
    /// Returns the (possibly updated) [`Response`].
    pub fn attach(self, response: Response) -> Response {
        let Tooltip {
            title,
            text,
            at_pointer,
        } = self;
        let add = move |ui: &mut Ui| {
            ui.set_max_width(280.0);
            if let Some(title) = &title {
                ui.add(Label::new(title.clone()).strong().size(Size::Small));
            }
            ui.add(Label::new(text.clone()).size(Size::Small));
        };
        if at_pointer {
            response.on_hover_ui_at_pointer(add)
        } else {
            response.on_hover_ui(add)
        }
    }
}
