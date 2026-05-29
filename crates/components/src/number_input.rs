//! `NumberInput` — a numeric field flanked by `−` / `+` stepper buttons.
//!
//! Wraps a single-line [`egui::TextEdit`] in the same themed frame as
//! [`Input`](crate::input::Input), keeps an internal text buffer so partial
//! edits (e.g. `"1."`) don't fight the bound value, and clamps to
//! `[min, max]`. Edits the bound `f64`; the returned [`Response`] reports
//! `.changed()` whenever the value changes.
//!
//! ```ignore
//! ui.add(sc::NumberInput::new(&mut qty).range(0.0..=99.0).step(1.0));
//! ```

use std::ops::RangeInclusive;

use crate::common::Size;
use egui::{pos2, vec2, FontId, Rect, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::{mix, Theme};

pub struct NumberInput<'a> {
    value: &'a mut f64,
    min: f64,
    max: f64,
    step: f64,
    precision: usize,
    width: Option<f32>,
    disabled: bool,
    size: Size,
}

impl<'a> NumberInput<'a> {
    pub fn new(value: &'a mut f64) -> Self {
        Self {
            value,
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            step: 1.0,
            precision: 0,
            width: None,
            disabled: false,
            size: Size::Medium,
        }
    }

    pub fn range(mut self, range: RangeInclusive<f64>) -> Self {
        self.min = *range.start();
        self.max = *range.end();
        self
    }
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }
    /// Number of decimal places used when displaying the value.
    pub fn precision(mut self, p: usize) -> Self {
        self.precision = p;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
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

    fn format(&self, v: f64) -> String {
        format!("{:.*}", self.precision, v)
    }
}

impl<'a> Widget for NumberInput<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = theme.colors;

        let height = self.size.input_height(&m);
        let step_w = height; // square stepper buttons
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().min(180.0))
            .max(step_w * 3.0);
        let radius = theme.corner();

        let (rect, mut response) =
            ui.allocate_exact_size(vec2(width, height), Sense::hover());
        let buf_id = response.id.with("buf");

        let minus_rect = Rect::from_min_size(rect.min, vec2(step_w, height));
        let plus_rect =
            Rect::from_min_size(pos2(rect.right() - step_w, rect.top()), vec2(step_w, height));
        let field_rect = Rect::from_min_max(
            pos2(minus_rect.right(), rect.top()),
            pos2(plus_rect.left(), rect.bottom()),
        );

        let mut changed = false;
        let mut value = *self.value;

        // Stepper interaction.
        let minus = ui.interact(minus_rect, response.id.with("minus"), step_sense(self.disabled));
        let plus = ui.interact(plus_rect, response.id.with("plus"), step_sense(self.disabled));
        if minus.clicked() {
            value = (value - self.step).clamp(self.min, self.max);
            changed = true;
        }
        if plus.clicked() {
            value = (value + self.step).clamp(self.min, self.max);
            changed = true;
        }

        // Text buffer kept in memory so partial input survives across frames.
        let mut buf = ui
            .data_mut(|d| d.get_temp::<String>(buf_id))
            .unwrap_or_else(|| self.format(value));
        if changed {
            buf = self.format(value);
        }

        // The editable field.
        let inner_rect = field_rect.shrink2(vec2(6.0, 4.0));
        let field_resp = {
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(inner_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            if self.disabled {
                child.disable();
            }
            let edit = egui::TextEdit::singleline(&mut buf)
                .frame(egui::Frame::NONE)
                .desired_width(inner_rect.width())
                .horizontal_align(egui::Align::Center)
                .font(FontId::proportional(m.font_size_md))
                .text_color(if self.disabled {
                    mix(c.foreground, c.muted_foreground, 0.5)
                } else {
                    c.foreground
                });
            child.add(edit)
        };

        let has_focus = field_resp.has_focus();
        if field_resp.changed() {
            if let Ok(parsed) = buf.trim().parse::<f64>() {
                value = parsed.clamp(self.min, self.max);
                changed = true;
            }
        }
        // Resync the buffer from the (possibly clamped) value once editing ends,
        // so an out-of-range or malformed entry settles to a valid display.
        if !has_focus {
            buf = self.format(value);
        }

        ui.data_mut(|d| d.insert_temp(buf_id, buf));

        if changed {
            *self.value = value;
            response.mark_changed();
        }

        // Painting.
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let bg = if self.disabled {
                mix(c.background, c.muted_background, 0.6)
            } else {
                c.background
            };
            painter.rect_filled(rect, radius, bg);

            let border_color = if has_focus {
                c.ring
            } else if response.hovered() || field_resp.hovered() {
                mix(c.input_border, c.foreground, 0.25)
            } else {
                c.input_border
            };
            painter.rect_stroke(
                rect,
                radius,
                Stroke::new(m.border_width, border_color),
                egui::StrokeKind::Inside,
            );

            // Divider lines between steppers and field.
            let divider = Stroke::new(m.border_width, c.input_border);
            painter.line_segment(
                [minus_rect.right_top(), minus_rect.right_bottom()],
                divider,
            );
            painter.line_segment([plus_rect.left_top(), plus_rect.left_bottom()], divider);

            let minus_disabled = self.disabled || value <= self.min;
            let plus_disabled = self.disabled || value >= self.max;
            paint_stepper(ui, minus_rect, &minus, "−", minus_disabled, &theme);
            paint_stepper(ui, plus_rect, &plus, "+", plus_disabled, &theme);

            if has_focus {
                ui.painter().rect_stroke(
                    rect.expand(2.0),
                    radius,
                    theme.focus_ring(),
                    egui::StrokeKind::Outside,
                );
            }
        }

        if !self.disabled && field_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Text);
        }

        response | field_resp | minus | plus
    }
}

fn step_sense(disabled: bool) -> Sense {
    if disabled {
        Sense::hover()
    } else {
        Sense::click()
    }
}

fn paint_stepper(
    ui: &Ui,
    rect: Rect,
    response: &Response,
    glyph: &str,
    disabled: bool,
    theme: &Theme,
) {
    let c = &theme.colors;
    let painter = ui.painter();
    let bg = if disabled {
        egui::Color32::TRANSPARENT
    } else if response.is_pointer_button_down_on() {
        c.secondary_active_background
    } else if response.hovered() {
        c.accent_background
    } else {
        egui::Color32::TRANSPARENT
    };
    if bg != egui::Color32::TRANSPARENT {
        painter.rect_filled(rect, 0.0, bg);
    }
    let fg = if disabled {
        mix(c.muted_foreground, c.background, 0.4)
    } else {
        c.foreground
    };
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        glyph,
        FontId::proportional(theme.metrics.font_size_lg),
        fg,
    );
    if !disabled && response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
}
