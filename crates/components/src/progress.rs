//! `Progress` — a horizontal progress bar (determinate or indeterminate).
//!
//! ```ignore
//! ui.add(sc::Progress::new(0.6));            // 60%
//! ui.add(sc::Progress::indeterminate());     // animated sweep
//! ```

use egui::{vec2, Color32, Response, Sense, Ui, Widget};
use egui_components_theme::Theme;

use crate::common::Variant;

pub struct Progress {
    value: f32,
    indeterminate: bool,
    width: Option<f32>,
    height: f32,
    variant: Variant,
}

impl Progress {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            indeterminate: false,
            width: None,
            height: 8.0,
            variant: Variant::Primary,
        }
    }

    pub fn indeterminate() -> Self {
        Self {
            value: 0.0,
            indeterminate: true,
            width: None,
            height: 8.0,
            variant: Variant::Primary,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }
    pub fn variant(mut self, v: Variant) -> Self {
        self.variant = v;
        self
    }
}

impl Widget for Progress {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let width = self.width.unwrap_or_else(|| ui.available_width().min(280.0));
        let (rect, response) = ui.allocate_exact_size(vec2(width, self.height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let radius = egui::CornerRadius::same((self.height * 0.5) as u8);
            let painter = ui.painter();
            painter.rect_filled(rect, radius, c.muted_background);

            let fill = fill_color(&c, self.variant);
            if self.indeterminate {
                // A segment sweeping left-to-right on a loop.
                let t = (ui.input(|i| i.time) % 1.4) as f32 / 1.4;
                let seg_w = width * 0.35;
                let travel = width + seg_w;
                let x = rect.left() - seg_w + travel * t;
                let seg = egui::Rect::from_min_max(
                    egui::pos2(x.max(rect.left()), rect.top()),
                    egui::pos2((x + seg_w).min(rect.right()), rect.bottom()),
                );
                if seg.width() > 0.0 {
                    painter.rect_filled(seg, radius, fill);
                }
                ui.ctx().request_repaint();
            } else if self.value > 0.0 {
                let filled = egui::Rect::from_min_size(
                    rect.min,
                    vec2(width * self.value, self.height),
                );
                painter.rect_filled(filled, radius, fill);
            }
        }

        response
    }
}

fn fill_color(c: &egui_components_theme::ThemeColor, v: Variant) -> Color32 {
    match v {
        Variant::Success => c.success_background,
        Variant::Warning => c.warning_background,
        Variant::Danger => c.danger_background,
        Variant::Info => c.info_background,
        _ => c.primary_background,
    }
}
