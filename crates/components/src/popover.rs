//! `Popover` — floating content anchored to a trigger widget.
//!
//! The generic sibling of [`Menu`](crate::menu::Menu) / [`Select`](crate::select::Select):
//! it toggles open on the trigger's click and renders whatever you put in the
//! closure inside a themed popover card.
//!
//! ```ignore
//! let trigger = ui.add(sc::Button::secondary("Open"));
//! sc::Popover::new("settings").show(ui, &trigger, |ui| {
//!     ui.add(sc::Label::new("Popover content").strong());
//!     ui.add(sc::Switch::new(&mut flag));
//! });
//! ```

use egui::{Frame, Id, InnerResponse, Margin, Response, Ui};
use egui_components_theme::Theme;

pub struct Popover {
    id: Id,
    width: Option<f32>,
    gap: f32,
    close_on_click: bool,
}

impl Popover {
    pub fn new(id_salt: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id_salt),
            width: None,
            gap: 4.0,
            close_on_click: false,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }
    /// Close as soon as anything inside is clicked (default: only on outside
    /// click / Esc).
    pub fn close_on_click(mut self) -> Self {
        self.close_on_click = true;
        self
    }

    /// Show anchored to `trigger`, toggled by its clicks. Returns the content
    /// closure's value when the popover is open.
    pub fn show<R>(
        self,
        ui: &mut Ui,
        trigger: &Response,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> Option<R> {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;

        let frame = Frame::new()
            .fill(c.popover_background)
            .stroke(theme.border_stroke())
            .corner_radius(theme.corner())
            .inner_margin(Margin::same(12))
            .shadow(egui::epaint::Shadow {
                offset: [0, 4],
                blur: 16,
                spread: 0,
                color: c.overlay,
            });

        let close = if self.close_on_click {
            egui::PopupCloseBehavior::CloseOnClick
        } else {
            egui::PopupCloseBehavior::CloseOnClickOutside
        };

        let mut popup = egui::Popup::from_toggle_button_response(trigger)
            .id(ui.make_persistent_id(self.id).with("popup"))
            .gap(self.gap)
            .close_behavior(close)
            .frame(frame);
        if let Some(w) = self.width {
            popup = popup.width(w);
        }

        popup
            .show(|ui| {
                if let Some(w) = self.width {
                    ui.set_width(w - 24.0);
                }
                content(ui)
            })
            .map(|r: InnerResponse<R>| r.inner)
    }
}
