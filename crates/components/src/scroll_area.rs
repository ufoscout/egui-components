//! `ScrollArea` — a theme-styled wrapper around [`egui::ScrollArea`].
//!
//! Plain [`egui::ScrollArea`] works fine; this wrapper exists so scrollable
//! regions in a component layout get the same slim, modern scroll bars as the
//! rest of the kit (the shadcn `ScrollArea` look) without each call site having
//! to tweak `Style::spacing.scroll`. It mirrors upstream gpui-component's
//! `scroll` module, whose `Scrollbar` likewise renders a thin overlay handle.
//!
//! It forwards the common [`egui::ScrollArea`] knobs and returns egui's own
//! [`ScrollAreaOutput`], so it is a drop-in replacement:
//!
//! ```ignore
//! sc::ScrollArea::vertical()
//!     .max_height(200.0)
//!     .show(ui, |ui| {
//!         for i in 0..100 {
//!             ui.add(sc::Label::new(format!("Row {i}")));
//!         }
//!     });
//! ```

use egui::scroll_area::ScrollAreaOutput;
use egui::{Ui, Vec2b};
use egui_components_theme::Theme;

/// A scrollable region with themed scroll bars.
pub struct ScrollArea {
    inner: egui::ScrollArea,
    floating: bool,
}

impl ScrollArea {
    fn wrap(inner: egui::ScrollArea) -> Self {
        Self {
            inner,
            floating: true,
        }
    }

    /// Scrolls vertically only.
    pub fn vertical() -> Self {
        Self::wrap(egui::ScrollArea::vertical())
    }

    /// Scrolls horizontally only.
    pub fn horizontal() -> Self {
        Self::wrap(egui::ScrollArea::horizontal())
    }

    /// Scrolls on both axes.
    pub fn both() -> Self {
        Self::wrap(egui::ScrollArea::both())
    }

    /// Stable id source so scroll offset survives across frames / siblings.
    pub fn id_salt(mut self, id_salt: impl std::hash::Hash) -> Self {
        self.inner = self.inner.id_salt(id_salt);
        self
    }

    /// Clamp the visible height; content beyond it scrolls.
    pub fn max_height(mut self, height: f32) -> Self {
        self.inner = self.inner.max_height(height);
        self
    }

    /// Clamp the visible width; content beyond it scrolls.
    pub fn max_width(mut self, width: f32) -> Self {
        self.inner = self.inner.max_width(width);
        self
    }

    /// Whether each axis shrinks to fit its content (defaults to egui's `true`).
    pub fn auto_shrink(mut self, auto_shrink: impl Into<Vec2b>) -> Self {
        self.inner = self.inner.auto_shrink(auto_shrink);
        self
    }

    /// If `true` (the default) the scroll bars float over the content and
    /// expand on hover; if `false` they are solid and always reserve space.
    pub fn floating(mut self, floating: bool) -> Self {
        self.floating = floating;
        self
    }

    /// Render the scrollable contents. Returns egui's [`ScrollAreaOutput`].
    pub fn show<R>(
        self,
        ui: &mut Ui,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> ScrollAreaOutput<R> {
        let _ = Theme::get(ui.ctx()); // ensure the theme is resolvable for callers

        // Apply a slim scroll-bar geometry just for this region, then restore
        // the previous style so surrounding widgets are unaffected. Only the
        // scroll geometry is touched — handle colours come from the theme's
        // installed widget visuals.
        let prev = ui.spacing().scroll;
        let mut scroll = if self.floating {
            egui::style::ScrollStyle::thin()
        } else {
            egui::style::ScrollStyle::solid()
        };
        scroll.bar_width = if self.floating { 8.0 } else { 10.0 };
        scroll.bar_inner_margin = 2.0;
        scroll.handle_min_length = 24.0;
        ui.style_mut().spacing.scroll = scroll;

        let out = self.inner.show(ui, add_contents);

        ui.style_mut().spacing.scroll = prev;
        out
    }
}
