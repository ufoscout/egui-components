//! `ListItem` — a clickable, themed row used to build list-style UIs.
//!
//! State (the selected index) lives on the caller, just like
//! [`Checkbox`](crate::checkbox::Checkbox) holds `&mut bool`. The matching
//! pattern is:
//!
//! ```ignore
//! egui::ScrollArea::vertical().show(ui, |ui| {
//!     for (i, item) in items.iter().enumerate() {
//!         let r = ui.add(
//!             sc::ListItem::new(item)
//!                 .selected(selected == Some(i)),
//!         );
//!         if r.clicked() {
//!             selected = Some(i);
//!         }
//!     }
//! });
//! ```
//!
//! For visual grouping a [`List`] container wraps the rows in a bordered,
//! rounded frame.

use egui::{
    pos2, vec2, Color32, FontId, Frame, InnerResponse, Margin, Response, Sense, Stroke, Ui,
    Vec2, Widget, WidgetText,
};
use egui_components_theme::{mix, Theme};

/// One row in a list. Stateless; the caller controls `selected`.
pub struct ListItem {
    label: WidgetText,
    secondary: Option<WidgetText>,
    selected: bool,
    disabled: bool,
    confirmed: bool,
}

impl ListItem {
    pub fn new(label: impl Into<WidgetText>) -> Self {
        Self {
            label: label.into(),
            secondary: None,
            selected: false,
            disabled: false,
            confirmed: false,
        }
    }

    /// Adds a muted, right-aligned secondary label (e.g. shortcut / metadata).
    pub fn secondary(mut self, text: impl Into<WidgetText>) -> Self {
        self.secondary = Some(text.into());
        self
    }

    pub fn selected(mut self, b: bool) -> Self {
        self.selected = b;
        self
    }
    pub fn disabled(mut self, b: bool) -> Self {
        self.disabled = b;
        self
    }
    /// Render a small check icon on the right (e.g. for "applied" status).
    pub fn confirmed(mut self, b: bool) -> Self {
        self.confirmed = b;
        self
    }
}

impl Widget for ListItem {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = &theme.colors;
        let font = FontId::proportional(m.font_size_md);

        let row_h = m.button_height_sm.max(28.0);
        let pad_x = 10.0;
        let gap = 8.0;

        let label_galley = self.label.clone().into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            ui.available_width(),
            font.clone(),
        );
        let secondary_galley = self.secondary.as_ref().map(|t| {
            t.clone().into_galley(
                ui,
                Some(egui::TextWrapMode::Extend),
                f32::INFINITY,
                font.clone(),
            )
        });

        let secondary_w = secondary_galley.as_ref().map(|g| g.size().x + gap).unwrap_or(0.0);
        let check_w = if self.confirmed { row_h * 0.6 + gap } else { 0.0 };
        let total_w = ui.available_width();
        let desired = vec2(total_w, row_h);
        let sense = if self.disabled { Sense::hover() } else { Sense::click() };
        let (rect, response) = ui.allocate_exact_size(desired, sense);

        if !ui.is_rect_visible(rect) {
            return response;
        }

        let painter = ui.painter();

        // Background
        let bg = if self.disabled {
            Color32::TRANSPARENT
        } else if self.selected {
            c.secondary_background
        } else if response.is_pointer_button_down_on() {
            c.accent_background
        } else if response.hovered() {
            c.accent_background
        } else {
            Color32::TRANSPARENT
        };
        if bg != Color32::TRANSPARENT {
            painter.rect_filled(rect, theme.corner_sm(), bg);
        }
        if self.selected {
            // Subtle left accent bar in primary
            painter.rect_filled(
                egui::Rect::from_min_size(
                    pos2(rect.left(), rect.top() + 4.0),
                    vec2(2.0, rect.height() - 8.0),
                ),
                egui::CornerRadius::same(1),
                c.primary_background,
            );
        }

        // Label
        let label_color = if self.disabled {
            mix(c.muted_foreground, Color32::TRANSPARENT, 0.3)
        } else {
            c.foreground
        };
        let label_x = rect.left() + pad_x;
        let label_y = rect.center().y - label_galley.size().y * 0.5;
        painter.galley_with_override_text_color(
            pos2(label_x, label_y),
            label_galley.clone(),
            label_color,
        );

        // Secondary (right-aligned, before optional check)
        let mut right = rect.right() - pad_x;
        if self.confirmed {
            let cx = right - check_w * 0.5 + gap * 0.5;
            let cy = rect.center().y;
            let size = row_h * 0.5;
            draw_check(painter, pos2(cx, cy), size, c.primary_background);
            right -= check_w;
        }
        if let Some(g) = secondary_galley {
            let pos = pos2(right - g.size().x, rect.center().y - g.size().y * 0.5);
            painter.galley_with_override_text_color(pos, g, c.muted_foreground);
            let _ = secondary_w; // reserved space; consumed visually
        }

        if response.has_focus() {
            painter.rect_stroke(
                rect.expand(1.0),
                theme.corner_sm(),
                theme.focus_ring(),
                egui::StrokeKind::Outside,
            );
        }

        if !self.disabled && response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        response
    }
}

fn draw_check(painter: &egui::Painter, center: egui::Pos2, size: f32, color: Color32) {
    let stroke = Stroke::new(1.8, color);
    let half = size * 0.5;
    let p1 = pos2(center.x - half * 0.6, center.y);
    let p2 = pos2(center.x - half * 0.15, center.y + half * 0.45);
    let p3 = pos2(center.x + half * 0.6, center.y - half * 0.5);
    painter.line_segment([p1, p2], stroke);
    painter.line_segment([p2, p3], stroke);
}

/// A bordered, rounded container that visually groups a list of [`ListItem`]s.
///
/// The container does not own the items — pass a closure that adds them.
pub struct List {
    id_salt: egui::Id,
    max_height: Option<f32>,
    padding: f32,
}

impl List {
    /// Create a list. `id_source` is hashed into the inner [`ScrollArea`]'s id
    /// so multiple `List`s on the same page can coexist without colliding.
    pub fn new(id_source: impl std::hash::Hash) -> Self {
        Self {
            id_salt: egui::Id::new(id_source),
            max_height: None,
            padding: 4.0,
        }
    }
    /// Cap the visible height; the list scrolls when its content exceeds it.
    pub fn max_height(mut self, h: f32) -> Self {
        self.max_height = Some(h);
        self
    }
    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = Theme::get(ui.ctx());
        Frame::new()
            .fill(theme.colors.background)
            .stroke(theme.border_stroke())
            .corner_radius(theme.corner())
            .inner_margin(Margin::same(self.padding as i8))
            .show(ui, |ui| {
                if let Some(h) = self.max_height {
                    egui::ScrollArea::vertical()
                        .id_salt(self.id_salt)
                        .max_height(h)
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            body(ui)
                        })
                        .inner
                } else {
                    body(ui)
                }
            })
    }
}

// Suppress unused-warnings for helpers we expose for future variants.
#[allow(dead_code)]
fn _ensure_vec_used(_: Vec2) {}
