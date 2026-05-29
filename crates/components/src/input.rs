//! gpui-component-style single-line `Input` field.
//!
//! Wraps [`egui::TextEdit::singleline`] with theme-aware framing (border,
//! focus ring, padding, optional placeholder + leading/trailing label).

use crate::common::Size;
use egui::{vec2, FontId, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::{mix, Theme};

pub struct Input<'a> {
    value: &'a mut String,
    placeholder: Option<String>,
    width: Option<f32>,
    password: bool,
    disabled: bool,
    size: Size,
}

impl<'a> Input<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            placeholder: None,
            width: None,
            password: false,
            disabled: false,
            size: Size::Medium,
        }
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = Some(p.into());
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn password(mut self, p: bool) -> Self {
        self.password = p;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
        self
    }
    pub fn small(self) -> Self {
        self.size(Size::Small)
    }
    pub fn large(self) -> Self {
        self.size(Size::Large)
    }
}

impl<'a> Widget for Input<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = theme.colors;
        let height = self.size.input_height(&m);
        let width = self.width.unwrap_or_else(|| ui.available_width().min(240.0));
        let desired = vec2(width, height);

        // Reserve the frame, then place the TextEdit inside.
        let (rect, response) = ui.allocate_exact_size(desired, Sense::hover());

        let painter = ui.painter();
        let radius = theme.corner();

        let bg = if self.disabled {
            mix(c.background, c.muted_background, 0.6)
        } else {
            c.background
        };
        painter.rect_filled(rect, radius, bg);

        // Border (will be re-drawn focused below).
        let mut border_color = c.input_border;
        let mut has_focus = false;

        // Place the TextEdit area inside the frame.
        let inner_rect = rect.shrink2(vec2(m.input_padding_x, 4.0));
        let inner_response = {
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(inner_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            if self.disabled {
                child.disable();
            }
            let edit = egui::TextEdit::singleline(self.value)
                .frame(egui::Frame::NONE)
                .desired_width(inner_rect.width())
                .password(self.password)
                .font(FontId::proportional(m.font_size_md))
                .text_color(if self.disabled {
                    mix(c.foreground, c.muted_foreground, 0.5)
                } else {
                    c.foreground
                });
            let r = child.add(edit);
            if r.has_focus() {
                has_focus = true;
            }
            r
        };

        if has_focus {
            border_color = c.ring;
        } else if response.hovered() || inner_response.hovered() {
            border_color = mix(c.input_border, c.foreground, 0.25);
        }

        ui.painter().rect_stroke(
            rect,
            radius,
            Stroke::new(m.border_width, border_color),
            egui::StrokeKind::Inside,
        );

        if has_focus {
            ui.painter().rect_stroke(
                rect.expand(2.0),
                theme.corner(),
                theme.focus_ring(),
                egui::StrokeKind::Outside,
            );
        }

        // Placeholder
        if self.value.is_empty() && !has_focus {
            if let Some(ph) = &self.placeholder {
                let font = FontId::proportional(m.font_size_md);
                ui.painter().text(
                    egui::pos2(inner_rect.left(), inner_rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    ph,
                    font,
                    c.muted_foreground,
                );
            }
        }

        if !self.disabled && (response.hovered() || inner_response.hovered()) {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Text);
        }

        // Return the inner response so callers see `.changed()`, `.lost_focus()` etc.
        inner_response.union(response)
    }
}
