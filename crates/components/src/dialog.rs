//! `Dialog` and `AlertDialog` — modal overlays built on [`egui::Modal`].
//!
//! Both are controlled by an `&mut bool` you own. `Dialog` is a general
//! container with a title + body closure; `AlertDialog` is a focused
//! confirm/cancel prompt that reports which button was pressed.
//!
//! ```ignore
//! if sc::Dialog::new("Edit profile").show(ctx, &mut self.open, |ui| {
//!     ui.add(sc::Input::new(&mut self.name));
//! }).is_some() { /* rendered this frame */ }
//!
//! match sc::AlertDialog::new("Delete file?")
//!     .description("This cannot be undone.")
//!     .danger()
//!     .show(ctx, &mut self.confirm_open)
//! {
//!     Some(sc::AlertChoice::Confirm) => { /* delete */ }
//!     Some(sc::AlertChoice::Cancel) | None => {}
//! }
//! ```

use egui::{Frame, Id, Margin, Sense};
use egui_components_theme::Theme;

use crate::button::Button;
use crate::common::{Size, Variant};
use crate::icon::{paint_icon, IconKind};
use crate::label::Label;

/// A general modal dialog.
pub struct Dialog {
    title: String,
    width: f32,
    closable: bool,
}

impl Dialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            width: 420.0,
            closable: true,
        }
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    /// Hide the header close (×) button (Esc / backdrop still close it).
    pub fn no_close_button(mut self) -> Self {
        self.closable = false;
        self
    }

    /// Show the dialog while `*open`. Returns `Some(R)` (the body's value) on
    /// the frames it is visible; sets `*open = false` when dismissed.
    pub fn show<R>(
        self,
        ctx: &egui::Context,
        open: &mut bool,
        body: impl FnOnce(&mut egui::Ui) -> R,
    ) -> Option<R> {
        if !*open {
            return None;
        }
        let theme = Theme::get(ctx);
        let c = theme.colors;
        let width = self.width;

        let frame = Frame::new()
            .fill(c.popover_background)
            .stroke(theme.border_stroke())
            .corner_radius(theme.corner_lg())
            .inner_margin(Margin::same(20))
            .shadow(egui::epaint::Shadow {
                offset: [0, 8],
                blur: 32,
                spread: 0,
                color: c.overlay,
            });

        let modal = egui::Modal::new(Id::new(("sc-dialog", self.title.as_str())))
            .frame(frame)
            .backdrop_color(c.overlay);

        let title = self.title;
        let closable = self.closable;
        let res = modal.show(ctx, |ui| {
            ui.set_width(width);
            let mut close_requested = false;
            ui.horizontal(|ui| {
                ui.add(Label::new(title.clone()).strong().size(Size::Large));
                if closable {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let (rect, resp) =
                            ui.allocate_exact_size(egui::vec2(20.0, 20.0), Sense::click());
                        let col = if resp.hovered() {
                            c.foreground
                        } else {
                            c.muted_foreground
                        };
                        paint_icon(ui.painter(), IconKind::Close, rect, col, 1.6);
                        if resp.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        if resp.clicked() {
                            close_requested = true;
                        }
                    });
                }
            });
            ui.add_space(12.0);
            let inner = body(ui);
            (inner, close_requested)
        });

        let should_close = res.should_close();
        let (inner, close_requested) = res.inner;
        if should_close || close_requested {
            *open = false;
        }
        Some(inner)
    }
}

/// Which button an [`AlertDialog`] reported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlertChoice {
    Confirm,
    Cancel,
}

/// A confirm / cancel prompt.
pub struct AlertDialog {
    title: String,
    description: Option<String>,
    confirm_label: String,
    cancel_label: String,
    confirm_variant: Variant,
    width: f32,
}

impl AlertDialog {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            confirm_label: "Continue".to_string(),
            cancel_label: "Cancel".to_string(),
            confirm_variant: Variant::Primary,
            width: 380.0,
        }
    }
    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }
    pub fn confirm_label(mut self, s: impl Into<String>) -> Self {
        self.confirm_label = s.into();
        self
    }
    pub fn cancel_label(mut self, s: impl Into<String>) -> Self {
        self.cancel_label = s.into();
        self
    }
    /// Style the confirm button as destructive.
    pub fn danger(mut self) -> Self {
        self.confirm_variant = Variant::Danger;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// Show while `*open`. Returns the chosen action on the frame a button is
    /// pressed (or the dialog is dismissed), and sets `*open = false` then.
    pub fn show(self, ctx: &egui::Context, open: &mut bool) -> Option<AlertChoice> {
        if !*open {
            return None;
        }
        let theme = Theme::get(ctx);
        let c = theme.colors;
        let width = self.width;

        let frame = Frame::new()
            .fill(c.popover_background)
            .stroke(theme.border_stroke())
            .corner_radius(theme.corner_lg())
            .inner_margin(Margin::same(20))
            .shadow(egui::epaint::Shadow {
                offset: [0, 8],
                blur: 32,
                spread: 0,
                color: c.overlay,
            });

        let modal = egui::Modal::new(Id::new(("sc-alert", self.title.as_str())))
            .frame(frame)
            .backdrop_color(c.overlay);

        let AlertDialog {
            title,
            description,
            confirm_label,
            cancel_label,
            confirm_variant,
            ..
        } = self;

        let res = modal.show(ctx, |ui| {
            ui.set_width(width);
            ui.add(Label::new(title.clone()).strong().size(Size::Large));
            if let Some(desc) = &description {
                ui.add_space(6.0);
                ui.add(Label::new(desc.clone()).muted());
            }
            ui.add_space(18.0);
            let mut choice = None;
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(Button::new(confirm_label.clone()).variant(confirm_variant))
                    .clicked()
                {
                    choice = Some(AlertChoice::Confirm);
                }
                ui.add_space(8.0);
                if ui.add(Button::secondary(cancel_label.clone())).clicked() {
                    choice = Some(AlertChoice::Cancel);
                }
            });
            choice
        });

        let choice = res.inner;
        if let Some(ch) = choice {
            *open = false;
            return Some(ch);
        }
        if res.should_close() {
            *open = false;
            return Some(AlertChoice::Cancel);
        }
        None
    }
}
