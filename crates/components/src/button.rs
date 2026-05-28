//! `Button` widget.
//!
//! Idiomatic egui widget: build with the chainable setters, then call
//! `.ui(ui)` (or pass to `ui.add(...)`). Returns [`egui::Response`] so callers
//! can `.clicked()` etc. exactly as with any built-in widget.

use crate::common::{Size, Variant};
use egui::{Color32, FontId, Rect, Response, Sense, Stroke, Ui, Vec2, Widget, WidgetText};
use egui_components_theme::{mix, Theme, ThemeColor};

pub struct Button {
    label: WidgetText,
    variant: Variant,
    size: Size,
    disabled: bool,
    full_width: bool,
    min_width: Option<f32>,
}

impl Button {
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::Primary,
            size: Size::Medium,
            disabled: false,
            full_width: false,
            min_width: None,
        }
    }

    pub fn primary(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(Variant::Primary)
    }
    pub fn secondary(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(Variant::Secondary)
    }
    pub fn ghost(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(Variant::Ghost)
    }
    pub fn outline(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(Variant::Outline)
    }
    pub fn danger(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(Variant::Danger)
    }
    pub fn link(label: impl Into<WidgetText>) -> Self {
        Self::new(label).variant(Variant::Link)
    }

    pub fn variant(mut self, v: Variant) -> Self {
        self.variant = v;
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
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn full_width(mut self) -> Self {
        self.full_width = true;
        self
    }
    pub fn min_width(mut self, w: f32) -> Self {
        self.min_width = Some(w);
        self
    }
}

impl Widget for Button {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let height = self.size.button_height(&m);
        let pad_x = self.size.button_padding_x(&m);
        let font = FontId::proportional(self.size.font_size(&m));

        let galley = self.label.clone().into_galley(
            ui,
            Some(egui::TextWrapMode::Extend),
            f32::INFINITY,
            font,
        );
        let text_w = galley.size().x;

        let desired_w = if self.full_width {
            ui.available_width()
        } else {
            (text_w + pad_x * 2.0).max(self.min_width.unwrap_or(0.0))
        };
        let desired_size = Vec2::new(desired_w, height);

        let sense = if self.disabled { Sense::hover() } else { Sense::click() };
        let (rect, response) = ui.allocate_exact_size(desired_size, sense);

        if ui.is_rect_visible(rect) {
            paint_button(ui, rect, &response, &theme, self.variant, self.disabled, &galley);
        }

        response
    }
}

fn paint_button(
    ui: &mut Ui,
    rect: Rect,
    response: &Response,
    theme: &Theme,
    variant: Variant,
    disabled: bool,
    galley: &std::sync::Arc<egui::Galley>,
) {
    let c = &theme.colors;
    let radius = theme.corner();

    let (bg, fg, border) = variant_colors(c, variant);

    let state_bg = if disabled {
        mix(bg, Color32::TRANSPARENT, 0.5)
    } else if response.is_pointer_button_down_on() {
        match variant {
            Variant::Primary => c.primary_active_background,
            Variant::Secondary => c.secondary_active_background,
            Variant::Danger => darken(c.danger_background, 0.15),
            Variant::Success => darken(c.success_background, 0.15),
            Variant::Warning => darken(c.warning_background, 0.15),
            Variant::Info => darken(c.info_background, 0.15),
            Variant::Ghost | Variant::Outline => mix(c.accent_background, c.foreground, 0.05),
            Variant::Link => bg,
        }
    } else if response.hovered() {
        match variant {
            Variant::Primary => c.primary_hover_background,
            Variant::Secondary => c.secondary_hover_background,
            Variant::Danger => lighten(c.danger_background, 0.08),
            Variant::Success => lighten(c.success_background, 0.08),
            Variant::Warning => lighten(c.warning_background, 0.08),
            Variant::Info => lighten(c.info_background, 0.08),
            Variant::Ghost | Variant::Outline => c.accent_background,
            Variant::Link => bg,
        }
    } else {
        bg
    };

    let painter = ui.painter();

    if !matches!(variant, Variant::Link | Variant::Ghost) || response.hovered() || response.is_pointer_button_down_on() {
        painter.rect_filled(rect, radius, state_bg);
    }

    if let Some(stroke) = border {
        painter.rect_stroke(rect, radius, stroke, egui::StrokeKind::Inside);
    }

    // Focus ring
    if response.has_focus() {
        let ring_rect = rect.expand(2.0);
        painter.rect_stroke(
            ring_rect,
            theme.corner(),
            theme.focus_ring(),
            egui::StrokeKind::Outside,
        );
    }

    // Text
    let text_color = if disabled { mix(fg, c.muted_foreground, 0.5) } else { fg };
    let text_pos = rect.center();
    painter.galley_with_override_text_color(
        text_pos - galley.size() * 0.5,
        galley.clone(),
        text_color,
    );

    // Link underline on hover
    if matches!(variant, Variant::Link) && response.hovered() {
        let underline_y = text_pos.y + galley.size().y * 0.5 - 1.0;
        painter.line_segment(
            [
                egui::pos2(rect.center().x - galley.size().x * 0.5, underline_y),
                egui::pos2(rect.center().x + galley.size().x * 0.5, underline_y),
            ],
            Stroke::new(1.0, text_color),
        );
    }

    // Hover cursor
    if !disabled && response.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
}

fn variant_colors(c: &ThemeColor, variant: Variant) -> (Color32, Color32, Option<Stroke>) {
    match variant {
        Variant::Primary => (c.primary_background, c.primary_foreground, None),
        Variant::Secondary => (c.secondary_background, c.secondary_foreground, None),
        Variant::Ghost => (Color32::TRANSPARENT, c.foreground, None),
        Variant::Outline => (
            Color32::TRANSPARENT,
            c.foreground,
            Some(Stroke::new(1.0, c.border)),
        ),
        Variant::Link => (Color32::TRANSPARENT, c.link_foreground, None),
        Variant::Danger => (c.danger_background, c.danger_foreground, None),
        Variant::Success => (c.success_background, c.success_foreground, None),
        Variant::Warning => (c.warning_background, c.warning_foreground, None),
        Variant::Info => (c.info_background, c.info_foreground, None),
    }
}

fn darken(c: Color32, t: f32) -> Color32 {
    mix(c, Color32::BLACK, t)
}
fn lighten(c: Color32, t: f32) -> Color32 {
    mix(c, Color32::WHITE, t)
}
