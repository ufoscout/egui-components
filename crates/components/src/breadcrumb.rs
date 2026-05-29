//! `Breadcrumb` — a horizontal trail of navigable links.
//!
//! All but the last item render as clickable links separated by a chevron; the
//! final item is the muted "current page" and is not clickable. [`show`] returns
//! the index of the item clicked this frame, if any.
//!
//! ```ignore
//! if let Some(i) = sc::Breadcrumb::new()
//!     .item("Home")
//!     .item("Library")
//!     .current("Data structures")
//!     .show(ui)
//! {
//!     // navigate to crumb `i`
//! }
//! ```

use egui::{vec2, FontId, Rect, Sense, Ui};
use egui_components_theme::Theme;

use crate::icon::{paint_icon, IconKind};

pub struct Breadcrumb {
    items: Vec<String>,
    /// Index of the current (non-clickable) item, if the last `current` was set.
    has_current: bool,
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            has_current: false,
        }
    }

    /// A clickable crumb.
    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.items.push(label.into());
        self.has_current = false;
        self
    }

    /// The final, current crumb (muted, not clickable). Should be added last.
    pub fn current(mut self, label: impl Into<String>) -> Self {
        self.items.push(label.into());
        self.has_current = true;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let m = theme.metrics;
        let font = FontId::proportional(m.font_size_sm);
        let last = self.items.len().saturating_sub(1);

        let mut clicked = None;
        ui.horizontal(|ui| {
            for (i, label) in self.items.iter().enumerate() {
                let is_current = self.has_current && i == last;
                let galley = ui.ctx().fonts_mut(|f| {
                    f.layout_no_wrap(label.clone(), font.clone(), c.muted_foreground)
                });
                let sense = if is_current {
                    Sense::hover()
                } else {
                    Sense::click()
                };
                let (rect, resp) = ui.allocate_exact_size(galley.size(), sense);

                let color = if is_current {
                    c.foreground
                } else if resp.hovered() {
                    c.link_hover_foreground
                } else {
                    c.muted_foreground
                };
                ui.painter()
                    .galley_with_override_text_color(rect.min, galley, color);
                if resp.hovered() && !is_current {
                    let y = rect.bottom() - 1.0;
                    ui.painter().line_segment(
                        [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                        egui::Stroke::new(1.0, color),
                    );
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                if resp.clicked() {
                    clicked = Some(i);
                }

                // Separator chevron (not after the last item).
                if i != last {
                    let (sep_rect, _) =
                        ui.allocate_exact_size(vec2(14.0, galley_h(&font)), Sense::hover());
                    let ir = Rect::from_center_size(sep_rect.center(), vec2(12.0, 12.0));
                    paint_icon(ui.painter(), IconKind::ChevronRight, ir, c.muted_foreground, 1.4);
                }
            }
        });

        clicked
    }
}

fn galley_h(font: &FontId) -> f32 {
    font.size + 4.0
}
