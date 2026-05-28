//! `Switch` widget (toggle).

use egui::{vec2, Color32, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::{mix, Theme};

pub struct Switch<'a> {
    on: &'a mut bool,
    disabled: bool,
}

impl<'a> Switch<'a> {
    pub fn new(on: &'a mut bool) -> Self {
        Self { on, disabled: false }
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
}

impl<'a> Widget for Switch<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = theme.colors;

        let desired = vec2(m.switch_width, m.switch_height);
        let sense = if self.disabled { Sense::hover() } else { Sense::click() };
        let (rect, mut response) = ui.allocate_exact_size(desired, sense);

        if response.clicked() && !self.disabled {
            *self.on = !*self.on;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            // Animate the thumb.
            let t = ui
                .ctx()
                .animate_bool_with_time(response.id, *self.on, 0.12);

            let track_color = if *self.on {
                c.primary_background
            } else {
                c.switch_background
            };
            let track_color = if self.disabled { fade(track_color) } else { track_color };

            let painter = ui.painter();
            let radius = egui::CornerRadius::same((m.switch_height * 0.5) as u8);
            painter.rect_filled(rect, radius, track_color);

            let thumb_d = m.switch_height - m.switch_thumb_padding * 2.0;
            let thumb_y = rect.center().y;
            let off_x = rect.left() + m.switch_thumb_padding + thumb_d * 0.5;
            let on_x = rect.right() - m.switch_thumb_padding - thumb_d * 0.5;
            let thumb_x = off_x + (on_x - off_x) * t;
            let thumb_color = if *self.on {
                c.primary_foreground
            } else {
                Color32::WHITE
            };
            let thumb_color = if self.disabled { fade(thumb_color) } else { thumb_color };

            painter.circle(
                egui::pos2(thumb_x, thumb_y),
                thumb_d * 0.5,
                thumb_color,
                Stroke::NONE,
            );

            if response.has_focus() {
                painter.rect_stroke(
                    rect.expand(2.0),
                    egui::CornerRadius::same((m.switch_height * 0.5 + 2.0) as u8),
                    theme.focus_ring(),
                    egui::StrokeKind::Outside,
                );
            }

            if !self.disabled && response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        }

        response
    }
}

fn fade(c: Color32) -> Color32 {
    mix(c, Color32::from_rgba_unmultiplied(0, 0, 0, 0), 0.4)
}
