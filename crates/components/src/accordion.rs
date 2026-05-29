//! `Accordion` — a single collapsible section with an animated chevron.
//!
//! Each section manages its own open/closed state in egui memory keyed by the
//! supplied id, so stacking several `Accordion`s makes a classic accordion
//! list. Pass `&mut bool` via [`open`](Accordion::open) for controlled state.
//!
//! ```ignore
//! sc::Accordion::new("billing", "Billing").show(ui, |ui| {
//!     ui.add(sc::Label::new("Card ending 4242"));
//! });
//! ```

use egui::{pos2, vec2, FontId, Id, Rect, Sense, Ui};
use egui_components_theme::Theme;

use crate::icon::{paint_icon, IconKind};

pub struct Accordion<'a> {
    id: Id,
    title: String,
    default_open: bool,
    open: Option<&'a mut bool>,
}

impl<'a> Accordion<'a> {
    pub fn new(id_salt: impl std::hash::Hash, title: impl Into<String>) -> Self {
        Self {
            id: Id::new(id_salt),
            title: title.into(),
            default_open: false,
            open: None,
        }
    }

    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    /// Drive the open state from the caller instead of egui memory.
    pub fn open(mut self, open: &'a mut bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn show<R>(
        mut self,
        ui: &mut Ui,
        body: impl FnOnce(&mut Ui) -> R,
    ) -> egui::InnerResponse<Option<R>> {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let m = theme.metrics;
        let mem_id = ui.make_persistent_id(self.id);

        let mut is_open = match &self.open {
            Some(b) => **b,
            None => ui
                .data(|d| d.get_temp::<bool>(mem_id))
                .unwrap_or(self.default_open),
        };

        // --- Header row ---
        let header_h = m.button_height_md;
        let (header_rect, header_resp) =
            ui.allocate_exact_size(vec2(ui.available_width(), header_h), Sense::click());

        if header_resp.clicked() {
            is_open = !is_open;
            if let Some(b) = self.open.as_deref_mut() {
                *b = is_open;
            }
            ui.data_mut(|d| d.insert_temp(mem_id, is_open));
        } else {
            // Persist (e.g. first frame with default) so state is stable.
            ui.data_mut(|d| d.insert_temp(mem_id, is_open));
        }

        let t = ui.ctx().animate_bool(mem_id.with("anim"), is_open);

        if ui.is_rect_visible(header_rect) {
            let painter = ui.painter();
            if header_resp.hovered() {
                painter.rect_filled(header_rect, theme.corner_sm(), c.accent_background);
            }
            // Rotating chevron.
            let icon_size = 16.0;
            let icon_rect = Rect::from_center_size(
                pos2(header_rect.left() + 4.0 + icon_size * 0.5, header_rect.center().y),
                vec2(icon_size, icon_size),
            );
            let kind = if t > 0.5 {
                IconKind::ChevronDown
            } else {
                IconKind::ChevronRight
            };
            paint_icon(painter, kind, icon_rect, c.muted_foreground, 1.6);

            painter.text(
                pos2(icon_rect.right() + 8.0, header_rect.center().y),
                egui::Align2::LEFT_CENTER,
                &self.title,
                FontId::proportional(m.font_size_md),
                c.foreground,
            );
            if header_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        }

        // --- Body ---
        let inner = if t > 0.0 {
            let r = ui.scope(|ui| {
                ui.add_space(4.0);
                egui::Frame::new()
                    .inner_margin(egui::Margin {
                        left: 28,
                        right: 8,
                        top: 0,
                        bottom: 8,
                    })
                    .show(ui, |ui| body(ui))
                    .inner
            });
            Some(r.inner)
        } else {
            None
        };

        ui.add(crate::separator::Separator::horizontal());

        egui::InnerResponse::new(inner, header_resp)
    }
}
