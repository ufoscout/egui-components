//! `Alert` widget — boxed inline message with optional title.

use crate::common::Variant;
use egui::{
    pos2, vec2, Color32, FontId, Response, Sense, Stroke, Ui, Widget,
};
use egui_components_theme::{mix, Theme, ThemeColor};

pub struct Alert {
    title: Option<String>,
    body: String,
    variant: Variant,
}

impl Alert {
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            title: None,
            body: body.into(),
            variant: Variant::Secondary,
        }
    }
    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }
    pub fn variant(mut self, v: Variant) -> Self {
        self.variant = v;
        self
    }
    pub fn info(self) -> Self {
        self.variant(Variant::Info)
    }
    pub fn success(self) -> Self {
        self.variant(Variant::Success)
    }
    pub fn warning(self) -> Self {
        self.variant(Variant::Warning)
    }
    pub fn danger(self) -> Self {
        self.variant(Variant::Danger)
    }
}

impl Widget for Alert {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = theme.colors;

        let (bg, fg, border) = alert_colors(&c, self.variant);

        let pad_x = 14.0;
        let pad_y = 12.0;
        let width = ui.available_width();
        let title_font = FontId::proportional(m.font_size_md);
        let body_font = FontId::proportional(m.font_size_sm);

        let max_text_w = width - pad_x * 2.0;
        let title_galley = self.title.as_ref().map(|t| {
            ui.ctx().fonts_mut(|f| {
                f.layout(
                    t.clone(),
                    title_font.clone(),
                    fg,
                    max_text_w,
                )
            })
        });
        let body_galley = ui.ctx().fonts_mut(|f| {
            f.layout(self.body.clone(), body_font.clone(), fg, max_text_w)
        });

        let mut content_h = body_galley.size().y;
        if let Some(g) = &title_galley {
            content_h += g.size().y + 4.0;
        }
        let total_size = vec2(width, content_h + pad_y * 2.0);
        let (rect, response) = ui.allocate_exact_size(total_size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let radius = theme.corner();
            painter.rect(
                rect,
                radius,
                bg,
                Stroke::new(1.0, border),
                egui::StrokeKind::Inside,
            );

            let mut y = rect.top() + pad_y;
            let x = rect.left() + pad_x;
            if let Some(g) = title_galley {
                painter.galley_with_override_text_color(pos2(x, y), g.clone(), fg);
                y += g.size().y + 4.0;
            }
            painter.galley_with_override_text_color(pos2(x, y), body_galley, fg);
        }

        response
    }
}

fn alert_colors(c: &ThemeColor, v: Variant) -> (Color32, Color32, Color32) {
    let (accent, fg) = match v {
        Variant::Info => (c.info_background, c.info_foreground),
        Variant::Success => (c.success_background, c.success_foreground),
        Variant::Warning => (c.warning_background, c.warning_foreground),
        Variant::Danger => (c.danger_background, c.danger_foreground),
        _ => (c.muted_foreground, c.foreground),
    };
    // Tint background by mixing the accent into the surface.
    let bg = mix(c.background, accent, 0.10);
    let border = mix(c.border, accent, 0.40);
    let text = if matches!(v, Variant::Secondary) {
        c.foreground
    } else {
        // For colored variants we prefer dark text on light tint.
        if is_light(bg) { darken(accent, 0.35) } else { fg }
    };
    (bg, text, border)
}

fn is_light(c: Color32) -> bool {
    // Perceptual luminance approximation.
    let r = c.r() as f32;
    let g = c.g() as f32;
    let b = c.b() as f32;
    (0.299 * r + 0.587 * g + 0.114 * b) > 140.0
}

fn darken(c: Color32, t: f32) -> Color32 {
    mix(c, Color32::BLACK, t)
}
