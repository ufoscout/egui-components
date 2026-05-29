//! `Sidebar` — a vertical navigation panel with icon + label items.
//!
//! Build the items inside the [`show`](Sidebar::show) closure; each
//! [`item`](SidebarUi::item) gets an auto-incrementing index. The currently
//! selected index is passed in for highlighting, and the index clicked this
//! frame is returned. Set [`collapsed`](Sidebar::collapsed) for an icon-only
//! rail (labels move into hover tooltips).
//!
//! ```ignore
//! let clicked = sc::Sidebar::new("nav")
//!     .selected(self.page)
//!     .collapsed(self.collapsed)
//!     .show(ui, |s| {
//!         s.header("Workspace");
//!         s.item(sc::IconKind::Home, "Home");      // 0
//!         s.item(sc::IconKind::File, "Documents");  // 1
//!         s.item(sc::IconKind::Settings, "Settings"); // 2
//!     });
//! if let Some(i) = clicked { self.page = i; }
//! ```

use egui::{pos2, vec2, Align, FontId, Layout, Rect, Sense, Ui, UiBuilder};
use egui_components_theme::Theme;

use crate::icon::{paint_icon, IconKind};
use crate::tooltip::Tooltip;

pub struct Sidebar {
    width: f32,
    collapsed_width: f32,
    collapsed: bool,
    selected: Option<usize>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new("sidebar")
    }
}

impl Sidebar {
    pub fn new(_id_salt: impl std::hash::Hash) -> Self {
        Self {
            width: 220.0,
            collapsed_width: 56.0,
            collapsed: false,
            selected: None,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    pub fn collapsed(mut self, c: bool) -> Self {
        self.collapsed = c;
        self
    }
    pub fn selected(mut self, idx: Option<usize>) -> Self {
        self.selected = idx;
        self
    }

    pub fn show(self, ui: &mut Ui, build: impl FnOnce(&mut SidebarUi)) -> Option<usize> {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let width = if self.collapsed {
            self.collapsed_width
        } else {
            self.width
        };
        let height = ui.available_height();

        let (rect, _) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
        // Panel surface + right border.
        ui.painter().rect_filled(rect, 0.0, c.muted_background);
        ui.painter().line_segment(
            [rect.right_top(), rect.right_bottom()],
            theme.border_stroke(),
        );

        let mut content = ui.new_child(
            UiBuilder::new()
                .max_rect(rect.shrink(8.0))
                .layout(Layout::top_down(Align::Min)),
        );

        let mut sb = SidebarUi {
            ui: &mut content,
            theme,
            collapsed: self.collapsed,
            selected: self.selected,
            next_index: 0,
            clicked: None,
        };
        build(&mut sb);
        sb.clicked
    }
}

/// Builder handed to the [`Sidebar::show`] closure.
pub struct SidebarUi<'a> {
    ui: &'a mut Ui,
    theme: Theme,
    collapsed: bool,
    selected: Option<usize>,
    next_index: usize,
    clicked: Option<usize>,
}

impl SidebarUi<'_> {
    /// A muted group heading (hidden when the sidebar is collapsed).
    pub fn header(&mut self, text: impl Into<String>) {
        if self.collapsed {
            self.ui.add_space(8.0);
            return;
        }
        self.ui.add_space(8.0);
        self.ui.add(
            crate::label::Label::new(text.into())
                .muted()
                .size(crate::common::Size::Small),
        );
        self.ui.add_space(2.0);
    }

    pub fn separator(&mut self) {
        self.ui.add_space(6.0);
        self.ui.add(crate::separator::Separator::horizontal());
        self.ui.add_space(6.0);
    }

    /// A navigation item. Returns `true` if it was clicked this frame.
    pub fn item(&mut self, icon: IconKind, label: impl Into<String>) -> bool {
        let index = self.next_index;
        self.next_index += 1;
        let selected = self.selected == Some(index);
        let label = label.into();

        let c = self.theme.colors;
        let m = self.theme.metrics;
        let row_h = m.button_height_md;
        let resp = {
            let ui = &mut self.ui;
            let (rect, resp) =
                ui.allocate_exact_size(vec2(ui.available_width(), row_h), Sense::click());

            if ui.is_rect_visible(rect) {
                let painter = ui.painter();
                let bg = if selected {
                    c.secondary_background
                } else if resp.hovered() {
                    c.accent_background
                } else {
                    egui::Color32::TRANSPARENT
                };
                if bg != egui::Color32::TRANSPARENT {
                    painter.rect_filled(rect, self.theme.corner_sm(), bg);
                }
                if selected {
                    painter.rect_filled(
                        Rect::from_min_size(
                            pos2(rect.left(), rect.top() + 5.0),
                            vec2(2.5, rect.height() - 10.0),
                        ),
                        egui::CornerRadius::same(1),
                        c.primary_background,
                    );
                }

                let fg = if selected { c.foreground } else { c.muted_foreground };
                let icon_size = 18.0;
                if self.collapsed {
                    let ir = Rect::from_center_size(rect.center(), vec2(icon_size, icon_size));
                    paint_icon(painter, icon, ir, fg, 1.7);
                } else {
                    let ir = Rect::from_center_size(
                        pos2(rect.left() + 10.0 + icon_size * 0.5, rect.center().y),
                        vec2(icon_size, icon_size),
                    );
                    paint_icon(painter, icon, ir, fg, 1.7);
                    painter.text(
                        pos2(ir.right() + 10.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        &label,
                        FontId::proportional(m.font_size_md),
                        fg,
                    );
                }
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
            }
            resp
        };

        // In collapsed mode, reveal the label as a tooltip.
        if self.collapsed {
            Tooltip::new(label).attach(resp.clone());
        }

        if resp.clicked() {
            self.clicked = Some(index);
            true
        } else {
            false
        }
    }
}
