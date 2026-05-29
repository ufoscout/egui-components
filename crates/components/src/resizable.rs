//! `Resizable` — two panes separated by a draggable divider.
//!
//! The split position is persisted in egui memory keyed by the supplied id, so
//! it survives across frames. Works horizontally (side-by-side, default) or
//! vertically (stacked).
//!
//! ```ignore
//! sc::Resizable::new("split")
//!     .default_fraction(0.3)
//!     .show(ui, |ui| { ui.label("left"); }, |ui| { ui.label("right"); });
//! ```

use egui::{pos2, Id, Rect, Sense, Ui, UiBuilder};
use egui_components_theme::{mix, Theme};

pub struct Resizable {
    id: Id,
    vertical: bool,
    default_fraction: f32,
    min_fraction: f32,
    max_fraction: f32,
    handle_thickness: f32,
}

impl Resizable {
    pub fn new(id_salt: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id_salt),
            vertical: false,
            default_fraction: 0.5,
            min_fraction: 0.1,
            max_fraction: 0.9,
            handle_thickness: 6.0,
        }
    }
    /// Stack the panes vertically (divider is horizontal) instead of side by side.
    pub fn vertical(mut self) -> Self {
        self.vertical = true;
        self
    }
    pub fn default_fraction(mut self, f: f32) -> Self {
        self.default_fraction = f.clamp(0.05, 0.95);
        self
    }
    pub fn min_fraction(mut self, f: f32) -> Self {
        self.min_fraction = f;
        self
    }
    pub fn max_fraction(mut self, f: f32) -> Self {
        self.max_fraction = f;
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        first: impl FnOnce(&mut Ui),
        second: impl FnOnce(&mut Ui),
    ) {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let mem_id = ui.make_persistent_id(self.id);

        let mut fraction = ui
            .data(|d| d.get_temp::<f32>(mem_id))
            .unwrap_or(self.default_fraction);

        // Claim the whole available space.
        let avail = ui.available_size();
        let total = if self.vertical { avail.y } else { avail.x };
        let (rect, _) = ui.allocate_exact_size(avail, Sense::hover());
        let half = self.handle_thickness * 0.5;

        let split = (total * fraction).round();

        let (first_rect, handle_rect, second_rect) = if self.vertical {
            (
                Rect::from_min_max(rect.min, pos2(rect.right(), rect.top() + split - half)),
                Rect::from_min_max(
                    pos2(rect.left(), rect.top() + split - half),
                    pos2(rect.right(), rect.top() + split + half),
                ),
                Rect::from_min_max(pos2(rect.left(), rect.top() + split + half), rect.max),
            )
        } else {
            (
                Rect::from_min_max(rect.min, pos2(rect.left() + split - half, rect.bottom())),
                Rect::from_min_max(
                    pos2(rect.left() + split - half, rect.top()),
                    pos2(rect.left() + split + half, rect.bottom()),
                ),
                Rect::from_min_max(pos2(rect.left() + split + half, rect.top()), rect.max),
            )
        };

        // Drag handle.
        let handle = ui.interact(handle_rect, mem_id.with("handle"), Sense::drag());
        if handle.dragged() && total > 0.0 {
            let delta = if self.vertical {
                handle.drag_delta().y
            } else {
                handle.drag_delta().x
            };
            fraction = (fraction + delta / total).clamp(self.min_fraction, self.max_fraction);
            ui.data_mut(|d| d.insert_temp(mem_id, fraction));
        }
        if handle.hovered() || handle.dragged() {
            ui.ctx().set_cursor_icon(if self.vertical {
                egui::CursorIcon::ResizeVertical
            } else {
                egui::CursorIcon::ResizeHorizontal
            });
        }

        // Divider line + grip.
        let line_color = if handle.hovered() || handle.dragged() {
            c.ring
        } else {
            c.border
        };
        let painter = ui.painter();
        if self.vertical {
            let y = handle_rect.center().y;
            painter.line_segment(
                [pos2(rect.left(), y), pos2(rect.right(), y)],
                egui::Stroke::new(1.0, line_color),
            );
        } else {
            let x = handle_rect.center().x;
            painter.line_segment(
                [pos2(x, rect.top()), pos2(x, rect.bottom())],
                egui::Stroke::new(1.0, line_color),
            );
        }
        // A faint grip block under the pointer for affordance.
        if handle.hovered() || handle.dragged() {
            painter.rect_filled(
                handle_rect,
                theme.corner_sm(),
                mix(c.ring, c.background, 0.85),
            );
        }

        // Pane contents.
        let mut first_ui = ui.new_child(
            UiBuilder::new()
                .max_rect(first_rect.shrink(2.0))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        first(&mut first_ui);

        let mut second_ui = ui.new_child(
            UiBuilder::new()
                .max_rect(second_rect.shrink(2.0))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        second(&mut second_ui);
    }
}
