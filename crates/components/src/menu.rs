//! `Menu` — a dropdown / context menu of clickable items.
//!
//! Build the entries, then attach the menu to a trigger [`Response`] (for a
//! dropdown) or show it as a context menu. [`show`](Menu::show) returns the
//! index of the entry that was clicked this frame, if any.
//!
//! ```ignore
//! let trigger = ui.add(sc::Button::secondary("Options ▾"));
//! if let Some(i) = sc::Menu::new("opts")
//!     .item("Rename")
//!     .item("Duplicate")
//!     .separator()
//!     .danger_item("Delete")
//!     .show(ui, &trigger)
//! {
//!     // handle click on entry `i`
//! }
//! ```

use egui::{vec2, Frame, Id, Margin, Rect, Response, Sense, Ui};
use egui_components_theme::{mix, Theme};

use crate::icon::{paint_icon, IconKind};

enum Entry {
    Item {
        label: String,
        icon: Option<IconKind>,
        shortcut: Option<String>,
        disabled: bool,
        danger: bool,
    },
    Separator,
    Label(String),
}

pub struct Menu {
    id: Id,
    entries: Vec<Entry>,
    width: f32,
}

impl Menu {
    pub fn new(id_salt: impl std::hash::Hash) -> Self {
        Self {
            id: Id::new(id_salt),
            entries: Vec::new(),
            width: 200.0,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn item(mut self, label: impl Into<String>) -> Self {
        self.entries.push(Entry::Item {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            danger: false,
        });
        self
    }

    pub fn icon_item(mut self, icon: IconKind, label: impl Into<String>) -> Self {
        self.entries.push(Entry::Item {
            label: label.into(),
            icon: Some(icon),
            shortcut: None,
            disabled: false,
            danger: false,
        });
        self
    }

    pub fn danger_item(mut self, label: impl Into<String>) -> Self {
        self.entries.push(Entry::Item {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            danger: true,
        });
        self
    }

    pub fn disabled_item(mut self, label: impl Into<String>) -> Self {
        self.entries.push(Entry::Item {
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: true,
            danger: false,
        });
        self
    }

    /// Add a keyboard-shortcut hint shown right-aligned on the last item.
    pub fn shortcut(mut self, s: impl Into<String>) -> Self {
        if let Some(Entry::Item { shortcut, .. }) = self.entries.last_mut() {
            *shortcut = Some(s.into());
        }
        self
    }

    pub fn separator(mut self) -> Self {
        self.entries.push(Entry::Separator);
        self
    }

    pub fn section_label(mut self, label: impl Into<String>) -> Self {
        self.entries.push(Entry::Label(label.into()));
        self
    }

    /// Show as a dropdown anchored to `trigger`, toggled by its clicks.
    /// Returns the clicked entry index, if any.
    pub fn show(self, ui: &mut Ui, trigger: &Response) -> Option<usize> {
        let popup = egui::Popup::from_toggle_button_response(trigger)
            .id(ui.make_persistent_id(self.id).with("popup"))
            .close_behavior(egui::PopupCloseBehavior::CloseOnClick);
        self.render(ui, popup)
    }

    /// Show as a context menu (right-click `trigger`). Returns the clicked
    /// entry index, if any.
    pub fn context_menu(self, ui: &mut Ui, trigger: &Response) -> Option<usize> {
        let popup = egui::Popup::context_menu(trigger)
            .id(ui.make_persistent_id(self.id).with("ctx"))
            .close_behavior(egui::PopupCloseBehavior::CloseOnClick);
        self.render(ui, popup)
    }

    fn render(self, ui: &mut Ui, popup: egui::Popup<'_>) -> Option<usize> {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let width = self.width;

        let frame = Frame::new()
            .fill(c.popover_background)
            .stroke(theme.border_stroke())
            .corner_radius(theme.corner())
            .inner_margin(Margin::same(4))
            .shadow(egui::epaint::Shadow {
                offset: [0, 4],
                blur: 16,
                spread: 0,
                color: c.overlay,
            });

        let entries = self.entries;
        popup
            .width(width)
            .gap(4.0)
            .frame(frame)
            .show(|ui| {
                ui.set_width(width - 8.0);
                let mut clicked = None;
                for (i, entry) in entries.iter().enumerate() {
                    match entry {
                        Entry::Separator => {
                            ui.add_space(2.0);
                            ui.add(crate::separator::Separator::horizontal());
                            ui.add_space(2.0);
                        }
                        Entry::Label(text) => {
                            ui.add(crate::label::Label::new(text.clone()).muted().size(crate::common::Size::Small));
                        }
                        Entry::Item {
                            label,
                            icon,
                            shortcut,
                            disabled,
                            danger,
                        } => {
                            if menu_item(ui, label, *icon, shortcut.as_deref(), *disabled, *danger) {
                                clicked = Some(i);
                            }
                        }
                    }
                }
                clicked
            })
            .and_then(|r| r.inner)
    }
}

fn menu_item(
    ui: &mut Ui,
    label: &str,
    icon: Option<IconKind>,
    shortcut: Option<&str>,
    disabled: bool,
    danger: bool,
) -> bool {
    let theme = Theme::get(ui.ctx());
    let c = theme.colors;
    let m = theme.metrics;
    let row_h = m.button_height_sm;
    let pad_x = 8.0;
    let icon_w = if icon.is_some() { 22.0 } else { 0.0 };

    let sense = if disabled { Sense::hover() } else { Sense::click() };
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), row_h), sense);

    if ui.is_rect_visible(rect) {
        let fg = if disabled {
            mix(c.muted_foreground, c.background, 0.3)
        } else if danger {
            c.danger_background
        } else {
            c.foreground
        };
        let painter = ui.painter();
        if !disabled && response.hovered() {
            let bg = if danger {
                mix(c.danger_background, c.background, 0.85)
            } else {
                c.accent_background
            };
            painter.rect_filled(rect, theme.corner_sm(), bg);
        }
        let mut x = rect.left() + pad_x;
        if let Some(kind) = icon {
            let ir = Rect::from_center_size(
                egui::pos2(x + 8.0, rect.center().y),
                vec2(16.0, 16.0),
            );
            paint_icon(painter, kind, ir, fg, 1.6);
            x += icon_w;
        }
        painter.text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            egui::FontId::proportional(m.font_size_md),
            fg,
        );
        if let Some(sc) = shortcut {
            painter.text(
                egui::pos2(rect.right() - pad_x, rect.center().y),
                egui::Align2::RIGHT_CENTER,
                sc,
                egui::FontId::proportional(m.font_size_sm),
                c.muted_foreground,
            );
        }
        if !disabled && response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }

    response.clicked()
}
