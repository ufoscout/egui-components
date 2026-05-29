//! `Pagination` — page navigation with prev/next arrows and numbered buttons
//! (collapsing to ellipses when there are many pages).
//!
//! The current page (0-based) lives on the caller. The returned [`Response`]
//! reports `.changed()` when the page changes.
//!
//! ```ignore
//! ui.add(sc::Pagination::new(&mut self.page, total_pages));
//! ```

use egui::{vec2, Rect, Response, Sense, Ui, Widget};
use egui_components_theme::{mix, Theme};

use crate::icon::{paint_icon, IconKind};

pub struct Pagination<'a> {
    current: &'a mut usize,
    pages: usize,
    siblings: usize,
}

impl<'a> Pagination<'a> {
    pub fn new(current: &'a mut usize, pages: usize) -> Self {
        Self {
            current,
            pages: pages.max(1),
            siblings: 1,
        }
    }
    /// How many page numbers to show on each side of the current one.
    pub fn siblings(mut self, n: usize) -> Self {
        self.siblings = n;
        self
    }
}

impl<'a> Widget for Pagination<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let pages = self.pages;
        let cur = (*self.current).min(pages - 1);
        let entries = layout_pages(cur, pages, self.siblings);

        let mut changed = false;
        let resp = ui
            .horizontal(|ui| {
                if arrow_button(ui, IconKind::ChevronLeft, cur > 0) {
                    *self.current = cur.saturating_sub(1);
                    changed = true;
                }
                for entry in entries {
                    match entry {
                        Some(p) => {
                            if page_button(ui, p + 1, p == cur) && p != cur {
                                *self.current = p;
                                changed = true;
                            }
                        }
                        None => ellipsis(ui),
                    }
                }
                if arrow_button(ui, IconKind::ChevronRight, cur + 1 < pages) {
                    *self.current = (cur + 1).min(pages - 1);
                    changed = true;
                }
            })
            .response;

        let mut resp = resp;
        if changed {
            resp.mark_changed();
        }
        resp
    }
}

/// Build the displayed sequence: `Some(page)` or `None` for an ellipsis.
fn layout_pages(cur: usize, pages: usize, siblings: usize) -> Vec<Option<usize>> {
    let mut out = Vec::new();
    let near = |i: usize| {
        i == 0
            || i + 1 == pages
            || (i as isize - cur as isize).unsigned_abs() <= siblings
    };
    let mut last: Option<usize> = None;
    for i in 0..pages {
        if near(i) {
            if let Some(l) = last {
                if i > l + 1 {
                    out.push(None);
                }
            }
            out.push(Some(i));
            last = Some(i);
        }
    }
    out
}

fn button_size(ui: &Ui) -> f32 {
    Theme::get(ui.ctx()).metrics.button_height_sm
}

fn arrow_button(ui: &mut Ui, icon: IconKind, enabled: bool) -> bool {
    let theme = Theme::get(ui.ctx());
    let c = theme.colors;
    let s = button_size(ui);
    let sense = if enabled { Sense::click() } else { Sense::hover() };
    let (rect, resp) = ui.allocate_exact_size(vec2(s, s), sense);
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        if enabled && resp.hovered() {
            painter.rect_filled(rect, theme.corner_sm(), c.accent_background);
        }
        let fg = if enabled {
            c.foreground
        } else {
            mix(c.muted_foreground, c.background, 0.5)
        };
        let ir = Rect::from_center_size(rect.center(), vec2(14.0, 14.0));
        paint_icon(painter, icon, ir, fg, 1.6);
        if enabled && resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }
    resp.clicked()
}

fn page_button(ui: &mut Ui, number: usize, selected: bool) -> bool {
    let theme = Theme::get(ui.ctx());
    let c = theme.colors;
    let s = button_size(ui);
    let (rect, resp) = ui.allocate_exact_size(vec2(s, s), Sense::click());
    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let bg = if selected {
            c.primary_background
        } else if resp.hovered() {
            c.accent_background
        } else {
            egui::Color32::TRANSPARENT
        };
        if bg != egui::Color32::TRANSPARENT {
            painter.rect_filled(rect, theme.corner_sm(), bg);
        }
        let fg = if selected {
            c.primary_foreground
        } else {
            c.foreground
        };
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            number.to_string(),
            egui::FontId::proportional(theme.metrics.font_size_sm),
            fg,
        );
        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }
    resp.clicked()
}

fn ellipsis(ui: &mut Ui) {
    let theme = Theme::get(ui.ctx());
    let s = button_size(ui);
    let (rect, _) = ui.allocate_exact_size(vec2(s * 0.7, s), Sense::hover());
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "…",
        egui::FontId::proportional(theme.metrics.font_size_md),
        theme.colors.muted_foreground,
    );
}
