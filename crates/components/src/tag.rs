//! `Tag` widget. Like [`Badge`](crate::badge::Badge) but with an
//! optional close (×) button and click semantics.

use crate::common::Variant;
use egui::{
    pos2, vec2, Color32, FontId, Rect, Response, Sense, Stroke, Ui, WidgetText,
};
use egui_components_theme::{Theme, ThemeColor};

pub struct Tag {
    label: WidgetText,
    variant: Variant,
    closable: bool,
}

pub struct TagResponse {
    pub response: Response,
    pub close_clicked: bool,
}

impl Tag {
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::Secondary,
            closable: false,
        }
    }
    pub fn variant(mut self, v: Variant) -> Self {
        self.variant = v;
        self
    }
    pub fn closable(mut self) -> Self {
        self.closable = true;
        self
    }

    /// Show the tag, returning a [`TagResponse`] with click + close info.
    pub fn show(self, ui: &mut Ui) -> TagResponse {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let pad_x = 8.0;
        let pad_y = 3.0;
        let close_w = if self.closable { 16.0 } else { 0.0 };
        let font = FontId::proportional(m.font_size_xs);
        let galley = self.label.into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            font,
        );
        let desired = vec2(
            galley.size().x + pad_x * 2.0 + close_w,
            galley.size().y + pad_y * 2.0,
        );
        let (rect, response) = ui.allocate_exact_size(desired, Sense::click());

        let (bg, fg) = tag_colors(&theme.colors, self.variant);
        let radius = theme.corner_sm();

        let mut close_clicked = false;
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let bg_eff = if response.hovered() {
                egui_components_theme::mix(bg, Color32::WHITE, 0.05)
            } else {
                bg
            };
            painter.rect(
                rect,
                radius,
                bg_eff,
                Stroke::new(1.0, egui_components_theme::mix(bg, Color32::BLACK, 0.1)),
                egui::StrokeKind::Inside,
            );

            let text_pos = pos2(rect.left() + pad_x, rect.center().y - galley.size().y * 0.5);
            painter.galley_with_override_text_color(text_pos, galley.clone(), fg);

            if self.closable {
                let cx = rect.right() - pad_x - 4.0;
                let cy = rect.center().y;
                let close_size = 10.0;
                let close_rect = Rect::from_center_size(pos2(cx, cy), vec2(close_size, close_size));
                let close_response = ui.interact(
                    close_rect.expand(2.0),
                    response.id.with("close"),
                    Sense::click(),
                );
                let stroke_color = if close_response.hovered() {
                    Color32::WHITE
                } else {
                    fg
                };
                painter.line_segment(
                    [close_rect.left_top(), close_rect.right_bottom()],
                    Stroke::new(1.2, stroke_color),
                );
                painter.line_segment(
                    [close_rect.right_top(), close_rect.left_bottom()],
                    Stroke::new(1.2, stroke_color),
                );
                if close_response.clicked() {
                    close_clicked = true;
                }
                if close_response.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
            }
        }

        TagResponse { response, close_clicked }
    }
}

fn tag_colors(c: &ThemeColor, v: Variant) -> (Color32, Color32) {
    match v {
        Variant::Primary => (c.primary_background, c.primary_foreground),
        Variant::Secondary => (c.secondary_background, c.secondary_foreground),
        Variant::Ghost => (c.muted_background, c.muted_foreground),
        Variant::Outline => (c.background, c.foreground),
        Variant::Link => (c.link_foreground, c.background),
        Variant::Danger => (c.danger_background, c.danger_foreground),
        Variant::Success => (c.success_background, c.success_foreground),
        Variant::Warning => (c.warning_background, c.warning_foreground),
        Variant::Info => (c.info_background, c.info_foreground),
    }
}
