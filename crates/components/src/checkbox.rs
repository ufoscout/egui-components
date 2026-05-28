//! `Checkbox` widget.

use egui::{
    pos2, vec2, Color32, FontId, Response, Sense, Stroke, Ui, Widget, WidgetText,
};
use egui_components_theme::Theme;

pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: Option<WidgetText>,
    disabled: bool,
}

impl<'a> Checkbox<'a> {
    pub fn new(checked: &'a mut bool, label: impl Into<WidgetText>) -> Self {
        Self {
            checked,
            label: Some(label.into()),
            disabled: false,
        }
    }

    pub fn without_label(checked: &'a mut bool) -> Self {
        Self { checked, label: None, disabled: false }
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
}

impl<'a> Widget for Checkbox<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = theme.colors;
        let box_size = m.checkbox_size;
        let label_font = FontId::proportional(m.font_size_md);

        let galley = self.label.as_ref().map(|t| {
            t.clone().into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                label_font,
            )
        });
        let label_w = galley.as_ref().map(|g| g.size().x).unwrap_or(0.0);
        let label_h = galley.as_ref().map(|g| g.size().y).unwrap_or(0.0);
        let gap = if galley.is_some() { 8.0 } else { 0.0 };

        let total = vec2(box_size + gap + label_w, box_size.max(label_h));
        let sense = if self.disabled { Sense::hover() } else { Sense::click() };
        let (rect, mut response) = ui.allocate_exact_size(total, sense);

        if response.clicked() && !self.disabled {
            *self.checked = !*self.checked;
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let box_rect = egui::Rect::from_min_size(
                pos2(rect.left(), rect.center().y - box_size * 0.5),
                vec2(box_size, box_size),
            );
            let painter = ui.painter();
            let radius = theme.corner_sm();

            let (fill, border) = if *self.checked {
                (c.primary_background, c.primary_background)
            } else if response.hovered() && !self.disabled {
                (c.background, c.foreground)
            } else {
                (c.background, c.border)
            };

            let fill = if self.disabled { fade(fill) } else { fill };
            let border = if self.disabled { fade(border) } else { border };

            painter.rect(
                box_rect,
                radius,
                fill,
                Stroke::new(1.5, border),
                egui::StrokeKind::Inside,
            );

            if *self.checked {
                // Draw a check mark
                let check_color = if self.disabled { fade(c.primary_foreground) } else { c.primary_foreground };
                let stroke = Stroke::new(2.0, check_color);
                let inset = box_size * 0.22;
                let l = box_rect.left() + inset;
                let r = box_rect.right() - inset;
                let t = box_rect.top() + inset;
                let b = box_rect.bottom() - inset;
                let mid_x = l + (r - l) * 0.38;
                let mid_y = b - (b - t) * 0.15;
                let start = pos2(l, t + (b - t) * 0.55);
                let mid = pos2(mid_x, mid_y);
                let end = pos2(r, t + (b - t) * 0.15);
                painter.line_segment([start, mid], stroke);
                painter.line_segment([mid, end], stroke);
            }

            if response.has_focus() {
                painter.rect_stroke(
                    box_rect.expand(2.5),
                    theme.corner(),
                    theme.focus_ring(),
                    egui::StrokeKind::Outside,
                );
            }

            if let Some(galley) = galley {
                let text_color = if self.disabled { fade(c.foreground) } else { c.foreground };
                let pos = pos2(
                    box_rect.right() + gap,
                    rect.center().y - galley.size().y * 0.5,
                );
                painter.galley_with_override_text_color(pos, galley, text_color);
            }

            if !self.disabled && response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        }

        response
    }
}

fn fade(c: Color32) -> Color32 {
    egui_components_theme::mix(c, Color32::from_rgba_unmultiplied(0, 0, 0, 0), 0.5)
}
