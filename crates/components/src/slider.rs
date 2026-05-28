//! `Slider` widget.
//!
//! egui ships its own [`egui::Slider`], but it has a very different visual
//! identity. This widget paints a flat track + circular thumb to match the
//! gpui-component look. Use [`egui::Slider`] if you need text input alongside,
//! ticks, or logarithmic scales.

use egui::{pos2, vec2, Color32, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::Theme;

pub struct Slider<'a> {
    value: &'a mut f32,
    range: std::ops::RangeInclusive<f32>,
    width: f32,
    disabled: bool,
}

impl<'a> Slider<'a> {
    pub fn new(value: &'a mut f32, range: std::ops::RangeInclusive<f32>) -> Self {
        Self { value, range, width: 200.0, disabled: false }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
}

impl<'a> Widget for Slider<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = theme.colors;
        let height = (m.slider_thumb_radius * 2.0).max(m.slider_track_height) + 4.0;
        let desired = vec2(self.width, height);

        let sense = if self.disabled { Sense::hover() } else { Sense::click_and_drag() };
        let (rect, mut response) = ui.allocate_exact_size(desired, sense);

        let (min, max) = (*self.range.start(), *self.range.end());
        let track_y = rect.center().y;
        let track_left = rect.left() + m.slider_thumb_radius;
        let track_right = rect.right() - m.slider_thumb_radius;
        let track_w = track_right - track_left;

        if !self.disabled {
            if let Some(pointer) = response.interact_pointer_pos() {
                let t = ((pointer.x - track_left) / track_w).clamp(0.0, 1.0);
                let new = min + t * (max - min);
                if (new - *self.value).abs() > f32::EPSILON {
                    *self.value = new;
                    response.mark_changed();
                }
            }
        }

        if ui.is_rect_visible(rect) {
            let t = ((*self.value - min) / (max - min)).clamp(0.0, 1.0);
            let thumb_x = track_left + t * track_w;

            let track_rect = egui::Rect::from_min_max(
                pos2(track_left, track_y - m.slider_track_height * 0.5),
                pos2(track_right, track_y + m.slider_track_height * 0.5),
            );
            let track_radius =
                egui::CornerRadius::same((m.slider_track_height * 0.5) as u8);

            let painter = ui.painter();

            let track_bg = if self.disabled {
                fade(c.muted_background)
            } else {
                c.muted_background
            };
            painter.rect_filled(track_rect, track_radius, track_bg);

            // Filled portion
            let filled = egui::Rect::from_min_max(track_rect.min, pos2(thumb_x, track_rect.max.y));
            let fill_color = if self.disabled { fade(c.slider_bar_background) } else { c.slider_bar_background };
            painter.rect_filled(filled, track_radius, fill_color);

            // Thumb
            let thumb_color = if self.disabled { fade(c.slider_thumb_background) } else { c.slider_thumb_background };
            let thumb_border = if self.disabled { fade(c.slider_bar_background) } else { c.slider_bar_background };
            painter.circle(
                pos2(thumb_x, track_y),
                m.slider_thumb_radius,
                thumb_color,
                Stroke::new(2.0, thumb_border),
            );

            if response.has_focus() {
                painter.circle_stroke(
                    pos2(thumb_x, track_y),
                    m.slider_thumb_radius + 3.0,
                    theme.focus_ring(),
                );
            }

            if !self.disabled && response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
            }
        }

        response
    }
}

fn fade(c: Color32) -> Color32 {
    egui_components_theme::mix(c, Color32::from_rgba_unmultiplied(0, 0, 0, 0), 0.4)
}
