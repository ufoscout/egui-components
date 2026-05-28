//! `Separator` widget. Horizontal or vertical thin line that follows
//! the theme's border color.

use egui::{Response, Sense, Ui, Widget};
use egui_components_theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Separator {
    orientation: Orientation,
    thickness: f32,
    length: Option<f32>,
}

impl Separator {
    pub fn horizontal() -> Self {
        Self { orientation: Orientation::Horizontal, thickness: 1.0, length: None }
    }
    pub fn vertical() -> Self {
        Self { orientation: Orientation::Vertical, thickness: 1.0, length: None }
    }
    pub fn length(mut self, l: f32) -> Self {
        self.length = Some(l);
        self
    }
    pub fn thickness(mut self, t: f32) -> Self {
        self.thickness = t;
        self
    }
}

impl Widget for Separator {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let color = theme.colors.border;
        let (w, h) = match self.orientation {
            Orientation::Horizontal => {
                (self.length.unwrap_or(ui.available_width()), self.thickness)
            }
            Orientation::Vertical => {
                (self.thickness, self.length.unwrap_or(ui.available_height().max(16.0)))
            }
        };
        let (rect, response) = ui.allocate_exact_size(egui::vec2(w, h), Sense::hover());
        if ui.is_rect_visible(rect) {
            ui.painter().rect_filled(rect, 0.0, color);
        }
        response
    }
}
