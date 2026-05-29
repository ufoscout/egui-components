//! `Collapsible` — the low-level show/hide primitive behind
//! [`Accordion`](crate::accordion::Accordion).
//!
//! Unlike `Accordion` (which owns a styled title bar), `Collapsible` lets you
//! render any trigger content next to the toggle chevron and any body when
//! open. Open state lives in egui memory keyed by the id, or pass
//! [`open`](Collapsible::open) for controlled state.
//!
//! ```ignore
//! sc::Collapsible::new("adv").show(
//!     ui,
//!     |ui| { ui.add(sc::Label::new("Advanced settings").strong()); },
//!     |ui| { ui.add(sc::Switch::new(&mut flag)); },
//! );
//! ```

use egui::{vec2, Id, Rect, Sense, Ui};
use egui_components_theme::Theme;

use crate::icon::{paint_icon, IconKind};

pub struct Collapsible<'a> {
    id: Id,
    default_open: bool,
    open: Option<&'a mut bool>,
}

impl<'a> Collapsible<'a> {
    pub fn new(id_salt: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id_salt),
            default_open: false,
            open: None,
        }
    }
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }
    pub fn open(mut self, open: &'a mut bool) -> Self {
        self.open = Some(open);
        self
    }

    pub fn show<R>(
        mut self,
        ui: &mut Ui,
        header: impl FnOnce(&mut Ui),
        body: impl FnOnce(&mut Ui) -> R,
    ) -> egui::InnerResponse<Option<R>> {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let mem_id = ui.make_persistent_id(self.id);

        let mut is_open = match &self.open {
            Some(b) => **b,
            None => ui
                .data(|d| d.get_temp::<bool>(mem_id))
                .unwrap_or(self.default_open),
        };

        // Header row: clickable chevron + caller content.
        let header_resp = ui
            .horizontal(|ui| {
                let chevron = 16.0;
                let (rect, resp) = ui.allocate_exact_size(vec2(chevron, chevron), Sense::click());
                let t = ui.ctx().animate_bool(mem_id.with("anim"), is_open);
                let kind = if t > 0.5 {
                    IconKind::ChevronDown
                } else {
                    IconKind::ChevronRight
                };
                let ir = Rect::from_center_size(rect.center(), vec2(chevron, chevron));
                paint_icon(ui.painter(), kind, ir, c.muted_foreground, 1.6);
                if resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                ui.add_space(4.0);
                header(ui);
                resp
            })
            .inner;

        if header_resp.clicked() {
            is_open = !is_open;
            if let Some(b) = self.open.as_deref_mut() {
                *b = is_open;
            }
        }
        ui.data_mut(|d| d.insert_temp(mem_id, is_open));

        let inner = if is_open {
            let r = ui.scope(|ui| {
                egui::Frame::new()
                    .inner_margin(egui::Margin {
                        left: 20,
                        right: 0,
                        top: 4,
                        bottom: 4,
                    })
                    .show(ui, |ui| body(ui))
                    .inner
            });
            Some(r.inner)
        } else {
            None
        };

        egui::InnerResponse::new(inner, header_resp)
    }
}
