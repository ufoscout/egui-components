//! `Notification` toasts — transient messages stacked in a screen corner.
//!
//! Hold a [`Toasts`] in your app state, push messages onto it, and call
//! [`Toasts::show`] once per frame. Toasts fade in, auto-dismiss after their
//! duration (unless hovered), and can be closed manually.
//!
//! ```ignore
//! // in app state: toasts: sc::Toasts,
//! if ui.button("Notify").clicked() {
//!     self.toasts.success("Saved", "Your changes were stored.");
//! }
//! self.toasts.show(ui.ctx());
//! ```

use egui::{vec2, Align2, Area, Color32, Frame, Id, Margin, Order, Sense, Stroke};
use egui_components_theme::{mix, Theme};

use crate::common::Variant;
use crate::icon::{paint_icon, IconKind};

/// Where toasts stack on screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToastAnchor {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
}

struct Toast {
    id: Id,
    variant: Variant,
    title: Option<String>,
    message: String,
    duration: f64,
    /// Wall-clock time (ctx time) at which the toast was created.
    created: Option<f64>,
}

/// A stack of active toasts. Cheap to keep in app state.
pub struct Toasts {
    anchor: ToastAnchor,
    width: f32,
    gap: f32,
    margin: f32,
    next_id: u64,
    toasts: Vec<Toast>,
}

impl Default for Toasts {
    fn default() -> Self {
        Self {
            anchor: ToastAnchor::TopRight,
            width: 320.0,
            gap: 8.0,
            margin: 16.0,
            next_id: 0,
            toasts: Vec::new(),
        }
    }
}

impl Toasts {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn anchor(mut self, anchor: ToastAnchor) -> Self {
        self.anchor = anchor;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// Push a toast with an explicit variant and 4s duration.
    pub fn add(&mut self, variant: Variant, title: Option<String>, message: impl Into<String>) {
        let id = Id::new(("toast", self.next_id));
        self.next_id = self.next_id.wrapping_add(1);
        self.toasts.push(Toast {
            id,
            variant,
            title,
            message: message.into(),
            duration: 4.0,
            created: None,
        });
    }

    pub fn info(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.add(Variant::Info, Some(title.into()), message);
    }
    pub fn success(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.add(Variant::Success, Some(title.into()), message);
    }
    pub fn warning(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.add(Variant::Warning, Some(title.into()), message);
    }
    pub fn error(&mut self, title: impl Into<String>, message: impl Into<String>) {
        self.add(Variant::Danger, Some(title.into()), message);
    }

    /// Render the active toasts and drop the expired ones.
    pub fn show(&mut self, ctx: &egui::Context) {
        if self.toasts.is_empty() {
            return;
        }
        let now = ctx.input(|i| i.time);
        let theme = Theme::get(ctx);

        let (pivot, base) = match self.anchor {
            ToastAnchor::TopRight => (
                Align2::RIGHT_TOP,
                ctx.content_rect().right_top() + vec2(-self.margin, self.margin),
            ),
            ToastAnchor::TopLeft => (
                Align2::LEFT_TOP,
                ctx.content_rect().left_top() + vec2(self.margin, self.margin),
            ),
            ToastAnchor::BottomRight => (
                Align2::RIGHT_BOTTOM,
                ctx.content_rect().right_bottom() + vec2(-self.margin, -self.margin),
            ),
            ToastAnchor::BottomLeft => (
                Align2::LEFT_BOTTOM,
                ctx.content_rect().left_bottom() + vec2(self.margin, -self.margin),
            ),
        };
        let stack_down = matches!(self.anchor, ToastAnchor::TopRight | ToastAnchor::TopLeft);

        let mut remove: Vec<Id> = Vec::new();
        let mut offset_y = 0.0;
        let mut need_repaint = false;

        for toast in self.toasts.iter_mut() {
            if toast.created.is_none() {
                toast.created = Some(now);
            }
            let age = now - toast.created.unwrap_or(now);
            let anchor_pos = base + vec2(0.0, if stack_down { offset_y } else { -offset_y });

            let resp = Area::new(toast.id)
                .order(Order::Foreground)
                .fixed_pos(anchor_pos)
                .pivot(pivot)
                .show(ctx, |ui| {
                    ui.set_width(self.width);
                    paint_toast(ui, &theme, toast)
                });

            let card_h = resp.response.rect.height();
            offset_y += card_h + self.gap;

            let hovered = resp.response.hovered();
            if resp.inner {
                // Close button clicked.
                remove.push(toast.id);
            } else if !hovered && age >= toast.duration {
                remove.push(toast.id);
            } else if !hovered {
                need_repaint = true;
            }
        }

        self.toasts.retain(|t| !remove.contains(&t.id));
        if need_repaint {
            ctx.request_repaint();
        }
    }
}

/// Returns true if the close button was clicked.
fn paint_toast(ui: &mut egui::Ui, theme: &Theme, toast: &Toast) -> bool {
    let c = theme.colors;
    let (accent, icon) = toast_accent(&c, toast.variant);

    let mut close_clicked = false;
    Frame::new()
        .fill(c.popover_background)
        .stroke(Stroke::new(theme.metrics.border_width, c.border))
        .corner_radius(theme.corner())
        .inner_margin(Margin::same(12))
        .shadow(egui::epaint::Shadow {
            offset: [0, 4],
            blur: 18,
            spread: 0,
            color: c.overlay,
        })
        .show(ui, |ui| {
            ui.horizontal_top(|ui| {
                // Accent icon.
                let (ir, _) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
                paint_icon(ui.painter(), icon, ir, accent, 1.8);
                ui.add_space(8.0);

                ui.vertical(|ui| {
                    ui.set_width(ui.available_width() - 22.0);
                    if let Some(title) = &toast.title {
                        ui.add(
                            crate::label::Label::new(title.clone())
                                .strong()
                                .size(crate::common::Size::Small),
                        );
                    }
                    ui.add(
                        crate::label::Label::new(toast.message.clone())
                            .muted()
                            .size(crate::common::Size::Small),
                    );
                });

                // Close (×) button.
                let (x_rect, x_resp) =
                    ui.allocate_exact_size(vec2(16.0, 16.0), Sense::click());
                let x_color = if x_resp.hovered() {
                    c.foreground
                } else {
                    c.muted_foreground
                };
                paint_icon(ui.painter(), IconKind::Close, x_rect, x_color, 1.4);
                if x_resp.clicked() {
                    close_clicked = true;
                }
                if x_resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
            });
        });

    close_clicked
}

fn toast_accent(c: &egui_components_theme::ThemeColor, v: Variant) -> (Color32, IconKind) {
    match v {
        Variant::Success => (c.success_background, IconKind::Check),
        Variant::Warning => (c.warning_background, IconKind::Warning),
        Variant::Danger => (c.danger_background, IconKind::Error),
        Variant::Info => (c.info_background, IconKind::Info),
        _ => (mix(c.foreground, c.muted_foreground, 0.3), IconKind::Info),
    }
}
