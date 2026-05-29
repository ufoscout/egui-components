//! `DescriptionList` — term / description pairs (an HTML `<dl>` equivalent).
//!
//! ```ignore
//! sc::DescriptionList::new()
//!     .item("Name", "Ada Lovelace")
//!     .item("Role", "Mathematician")
//!     .bordered()
//!     .show(ui);
//! ```

use egui::{Frame, Margin, Ui};
use egui_components_theme::Theme;

use crate::common::Size;
use crate::label::Label;
use crate::separator::Separator;

pub struct DescriptionList {
    rows: Vec<(String, String)>,
    label_width: f32,
    bordered: bool,
}

impl Default for DescriptionList {
    fn default() -> Self {
        Self::new()
    }
}

impl DescriptionList {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            label_width: 140.0,
            bordered: false,
        }
    }
    pub fn item(mut self, term: impl Into<String>, description: impl Into<String>) -> Self {
        self.rows.push((term.into(), description.into()));
        self
    }
    pub fn label_width(mut self, w: f32) -> Self {
        self.label_width = w;
        self
    }
    /// Wrap the list in a bordered card with separators between rows.
    pub fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> egui::Response {
        let theme = Theme::get(ui.ctx());
        if self.bordered {
            Frame::new()
                .fill(theme.colors.background)
                .stroke(theme.border_stroke())
                .corner_radius(theme.corner())
                .inner_margin(Margin::symmetric(14, 10))
                .show(ui, |ui| self.rows_ui(ui))
                .response
        } else {
            ui.scope(|ui| self.rows_ui(ui)).response
        }
    }

    fn rows_ui(&self, ui: &mut Ui) {
        let row_count = self.rows.len();
        for (i, (term, desc)) in self.rows.iter().enumerate() {
            ui.horizontal(|ui| {
                ui.allocate_ui(egui::vec2(self.label_width, 0.0), |ui| {
                    ui.add(Label::new(term.clone()).muted().size(Size::Small));
                });
                ui.add(Label::new(desc.clone()));
            });
            if self.bordered && i + 1 < row_count {
                ui.add_space(6.0);
                ui.add(Separator::horizontal());
                ui.add_space(6.0);
            } else {
                ui.add_space(6.0);
            }
        }
    }
}
