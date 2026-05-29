//! `OtpInput` — a row of single-character boxes for one-time codes.
//!
//! Backed by a single `&mut String`; only the configured number of characters
//! is kept. Click to focus the group, then type — digits fill left to right
//! and `Backspace` removes the last one. The returned [`Response`] reports
//! `.changed()` on edits.
//!
//! ```ignore
//! ui.add(sc::OtpInput::new(&mut self.code).length(6));
//! ```

use egui::{vec2, FontId, Key, Rect, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::Theme;

pub struct OtpInput<'a> {
    value: &'a mut String,
    length: usize,
    digits_only: bool,
    box_size: f32,
    gap: f32,
}

impl<'a> OtpInput<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            length: 6,
            digits_only: true,
            box_size: 40.0,
            gap: 8.0,
        }
    }
    pub fn length(mut self, n: usize) -> Self {
        self.length = n.max(1);
        self
    }
    /// Allow any character (otherwise only ASCII digits are accepted).
    pub fn any_char(mut self) -> Self {
        self.digits_only = false;
        self
    }
    pub fn box_size(mut self, s: f32) -> Self {
        self.box_size = s;
        self
    }
}

impl<'a> Widget for OtpInput<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = theme.colors;

        let total_w = self.length as f32 * self.box_size + (self.length - 1) as f32 * self.gap;
        let desired = vec2(total_w, self.box_size);
        let (rect, mut response) = ui.allocate_exact_size(desired, Sense::click());

        if response.clicked() {
            response.request_focus();
        }
        let has_focus = response.has_focus();

        // Collect typed input while focused.
        let mut changed = false;
        if has_focus {
            let mut chars: Vec<char> = self.value.chars().collect();
            ui.input(|i| {
                for ev in &i.events {
                    match ev {
                        egui::Event::Text(t) => {
                            for ch in t.chars() {
                                if chars.len() >= self.length {
                                    break;
                                }
                                if self.digits_only && !ch.is_ascii_digit() {
                                    continue;
                                }
                                if ch.is_control() {
                                    continue;
                                }
                                chars.push(ch);
                                changed = true;
                            }
                        }
                        egui::Event::Key {
                            key: Key::Backspace,
                            pressed: true,
                            ..
                        } => {
                            changed |= chars.pop().is_some();
                        }
                        _ => {}
                    }
                }
            });
            if changed {
                *self.value = chars.into_iter().take(self.length).collect();
            }
        }

        if changed {
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let radius = theme.corner();
            let filled = self.value.chars().count();
            let font = FontId::proportional(m.font_size_lg);
            for idx in 0..self.length {
                let x = rect.left() + idx as f32 * (self.box_size + self.gap);
                let box_rect =
                    Rect::from_min_size(egui::pos2(x, rect.top()), vec2(self.box_size, self.box_size));
                let painter = ui.painter();
                painter.rect_filled(box_rect, radius, c.background);

                let active = has_focus && idx == filled.min(self.length - 1);
                let border = if active {
                    c.ring
                } else {
                    c.input_border
                };
                painter.rect_stroke(
                    box_rect,
                    radius,
                    Stroke::new(if active { m.focus_ring_width } else { m.border_width }, border),
                    egui::StrokeKind::Inside,
                );

                if let Some(ch) = self.value.chars().nth(idx) {
                    painter.text(
                        box_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        ch,
                        font.clone(),
                        c.foreground,
                    );
                }
            }
        }

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Text);
        }

        response
    }
}
