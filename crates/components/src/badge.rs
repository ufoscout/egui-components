//! `Badge` widget — small inline pill / chip.

use crate::common::Variant;
use egui::{vec2, Color32, FontId, Response, Sense, Stroke, Ui, Widget, WidgetText};
use egui_components_theme::{Theme, ThemeColor};

pub struct Badge {
    label: WidgetText,
    variant: Variant,
    outlined: bool,
}

impl Badge {
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::Primary,
            outlined: false,
        }
    }
    pub fn variant(mut self, v: Variant) -> Self {
        self.variant = v;
        self
    }
    pub fn outlined(mut self) -> Self {
        self.outlined = true;
        self
    }
}

impl Widget for Badge {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let pad_x = 8.0;
        let pad_y = 2.0;
        let font = FontId::proportional(m.font_size_xs);
        let galley = self.label.into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            font,
        );
        let desired = vec2(galley.size().x + pad_x * 2.0, galley.size().y + pad_y * 2.0);
        let (rect, response) = ui.allocate_exact_size(desired, Sense::hover());

        if ui.is_rect_visible(rect) {
            let (bg, fg) = variant_colors(&theme.colors, self.variant);
            let radius = egui::CornerRadius::same((desired.y * 0.5) as u8);
            let painter = ui.painter();

            if self.outlined {
                painter.rect(
                    rect,
                    radius,
                    Color32::TRANSPARENT,
                    Stroke::new(1.0, bg),
                    egui::StrokeKind::Inside,
                );
                painter.galley_with_override_text_color(
                    rect.center() - galley.size() * 0.5,
                    galley,
                    bg,
                );
            } else {
                painter.rect_filled(rect, radius, bg);
                painter.galley_with_override_text_color(
                    rect.center() - galley.size() * 0.5,
                    galley,
                    fg,
                );
            }
        }

        response
    }
}

fn variant_colors(c: &ThemeColor, v: Variant) -> (Color32, Color32) {
    match v {
        Variant::Primary => (c.primary_background, c.primary_foreground),
        Variant::Secondary => (c.secondary_background, c.secondary_foreground),
        Variant::Ghost => (c.muted_background, c.muted_foreground),
        Variant::Outline => (c.foreground, c.background),
        Variant::Link => (c.link_foreground, c.background),
        Variant::Danger => (c.danger_background, c.danger_foreground),
        Variant::Success => (c.success_background, c.success_foreground),
        Variant::Warning => (c.warning_background, c.warning_foreground),
        Variant::Info => (c.info_background, c.info_foreground),
    }
}
