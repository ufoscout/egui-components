//! `TitleBar` — a custom window title bar (for borderless / custom-chrome apps).
//!
//! Renders a draggable bar with the app title on the left, a closure for your
//! own right-aligned content, and optional minimize / maximize / close buttons
//! wired to the platform via [`egui::ViewportCommand`]. Double-clicking the bar
//! toggles maximize; dragging it moves the window.
//!
//! ```ignore
//! sc::TitleBar::new("My App").show(ui, |ui| {
//!     ui.add(sc::Button::ghost("Help"));
//! });
//! ```

use egui::{pos2, vec2, Align, Layout, Rect, Sense, Ui, UiBuilder, ViewportCommand};
use egui_components_theme::Theme;

use crate::icon::{paint_icon, IconKind};

pub struct TitleBar {
    title: String,
    height: f32,
    window_controls: bool,
}

impl TitleBar {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            height: 38.0,
            window_controls: true,
        }
    }
    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }
    /// Hide the minimize / maximize / close buttons.
    pub fn no_window_controls(mut self) -> Self {
        self.window_controls = false;
        self
    }

    pub fn show(self, ui: &mut Ui, right_content: impl FnOnce(&mut Ui)) {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;

        let width = ui.available_width();
        let (rect, bar_resp) =
            ui.allocate_exact_size(vec2(width, self.height), Sense::click_and_drag());

        // Window move / maximize via the bar background.
        if bar_resp.drag_started() {
            ui.ctx().send_viewport_cmd(ViewportCommand::StartDrag);
        }
        if bar_resp.double_clicked() {
            let maximized = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            ui.ctx()
                .send_viewport_cmd(ViewportCommand::Maximized(!maximized));
        }

        // Surface.
        ui.painter().rect_filled(rect, 0.0, c.background);
        ui.painter().line_segment(
            [rect.left_bottom(), rect.right_bottom()],
            theme.border_stroke(),
        );

        // Title (left).
        ui.painter().text(
            pos2(rect.left() + 14.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            &self.title,
            egui::FontId::proportional(theme.metrics.font_size_md),
            c.foreground,
        );

        // Window controls (right), then user content to their left.
        let mut right_edge = rect.right();
        if self.window_controls {
            let btn_w = self.height * 1.2;
            let close = control_button(ui, btn_rect(rect, right_edge, btn_w), IconKind::Close, true);
            if close {
                ui.ctx().send_viewport_cmd(ViewportCommand::Close);
            }
            right_edge -= btn_w;

            let is_max = ui.input(|i| i.viewport().maximized.unwrap_or(false));
            if maximize_button(ui, btn_rect(rect, right_edge, btn_w)) {
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::Maximized(!is_max));
            }
            right_edge -= btn_w;

            if control_button(ui, btn_rect(rect, right_edge, btn_w), IconKind::Minus, false) {
                ui.ctx().send_viewport_cmd(ViewportCommand::Minimized(true));
            }
            right_edge -= btn_w;
        }

        // User content region (right-aligned, left of the window controls).
        let content_rect = Rect::from_min_max(
            pos2(rect.left() + 120.0, rect.top()),
            pos2(right_edge - 4.0, rect.bottom()),
        );
        if content_rect.width() > 0.0 {
            let mut content = ui.new_child(
                UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(Layout::right_to_left(Align::Center)),
            );
            right_content(&mut content);
        }
    }
}

/// Maximize/restore button — drawn as a square outline (no matching icon glyph).
fn maximize_button(ui: &mut Ui, rect: Rect) -> bool {
    let theme = Theme::get(ui.ctx());
    let c = theme.colors;
    let resp = ui.interact(
        rect,
        ui.id().with(("titlebar-max", rect.center().x as i32)),
        Sense::click(),
    );
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        if resp.hovered() {
            painter.rect_filled(rect, 0.0, c.accent_background);
        }
        let sq = Rect::from_center_size(rect.center(), vec2(11.0, 11.0));
        painter.rect_stroke(
            sq,
            egui::CornerRadius::same(1),
            egui::Stroke::new(1.5, c.foreground),
            egui::StrokeKind::Inside,
        );
        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }
    resp.clicked()
}

fn btn_rect(bar: Rect, right_edge: f32, w: f32) -> Rect {
    Rect::from_min_max(
        pos2(right_edge - w, bar.top()),
        pos2(right_edge, bar.bottom()),
    )
}

/// Returns true if clicked. `danger` tints the hover red (for close).
fn control_button(ui: &mut Ui, rect: Rect, icon: IconKind, danger: bool) -> bool {
    let theme = Theme::get(ui.ctx());
    let c = theme.colors;
    let resp = ui.interact(rect, ui.id().with(("titlebar-ctl", rect.center().x as i32)), Sense::click());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        if resp.hovered() {
            let bg = if danger {
                c.danger_background
            } else {
                c.accent_background
            };
            painter.rect_filled(rect, 0.0, bg);
        }
        let fg = if danger && resp.hovered() {
            c.danger_foreground
        } else {
            c.foreground
        };
        let ir = Rect::from_center_size(rect.center(), vec2(14.0, 14.0));
        paint_icon(painter, icon, ir, fg, 1.5);
        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }
    resp.clicked()
}
