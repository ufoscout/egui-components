//! `Radio` — a single radio button. Pair several with the same backing value
//! to form a group.
//!
//! Like egui's `radio_value`, the idiomatic use is via
//! [`Radio::selectable`], which takes the current value and the value this
//! button represents:
//!
//! ```ignore
//! ui.add(sc::Radio::selectable(&mut self.choice, Choice::A, "Option A"));
//! ui.add(sc::Radio::selectable(&mut self.choice, Choice::B, "Option B"));
//! ```
//!
//! Or drive it manually with [`Radio::new`] (selected = bool) and handle
//! `.clicked()` yourself.

use egui::{pos2, vec2, FontId, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::{mix, Theme};

pub struct Radio {
    selected: bool,
    label: Option<String>,
    disabled: bool,
}

impl Radio {
    pub fn new(selected: bool, label: impl Into<String>) -> Self {
        Self {
            selected,
            label: Some(label.into()),
            disabled: false,
        }
    }
    /// Radio with no text label.
    pub fn bare(selected: bool) -> Self {
        Self {
            selected,
            label: None,
            disabled: false,
        }
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    /// Convenience for radio groups: selects `value` into `current` on click.
    /// Returns the [`Response`]; `.clicked()` is true when this option is
    /// chosen.
    pub fn selectable<'a, T: PartialEq>(
        current: &'a mut T,
        value: T,
        label: impl Into<String>,
    ) -> SelectableRadio<'a, T> {
        SelectableRadio {
            radio: Radio::new(*current == value, label),
            current,
            value,
        }
    }
}

impl Widget for Radio {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let m = theme.metrics;
        let size = m.checkbox_size;
        let gap = 8.0;
        let font = FontId::proportional(m.font_size_md);

        let label_galley = self.label.as_ref().map(|t| {
            ui.ctx()
                .fonts_mut(|f| f.layout_no_wrap(t.clone(), font.clone(), c.foreground))
        });
        let label_w = label_galley.as_ref().map(|g| g.size().x + gap).unwrap_or(0.0);
        let label_h = label_galley.as_ref().map(|g| g.size().y).unwrap_or(0.0);

        let desired = vec2(size + label_w, size.max(label_h));
        let sense = if self.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, response) = ui.allocate_exact_size(desired, sense);

        if ui.is_rect_visible(rect) {
            let center = pos2(rect.left() + size * 0.5, rect.center().y);
            let radius = size * 0.5;
            let painter = ui.painter();

            let (ring, dot) = if self.disabled {
                (mix(c.input_border, c.background, 0.4), mix(c.primary_background, c.background, 0.4))
            } else if self.selected {
                (c.primary_background, c.primary_background)
            } else {
                (c.input_border, c.primary_background)
            };

            painter.circle(
                center,
                radius,
                c.background,
                Stroke::new(m.border_width + if self.selected { 0.5 } else { 0.0 }, ring),
            );
            if self.selected {
                painter.circle_filled(center, radius * 0.5, dot);
            }

            if response.has_focus() {
                painter.circle_stroke(center, radius + 2.5, theme.focus_ring());
            }

            if let Some(g) = label_galley {
                let pos = pos2(rect.left() + size + gap, rect.center().y - g.size().y * 0.5);
                let color = if self.disabled {
                    c.muted_foreground
                } else {
                    c.foreground
                };
                painter.galley_with_override_text_color(pos, g, color);
            }

            if !self.disabled && response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        }

        response
    }
}

/// Returned by [`Radio::selectable`]; updates the backing value on click.
pub struct SelectableRadio<'a, T: PartialEq> {
    radio: Radio,
    current: &'a mut T,
    value: T,
}

impl<T: PartialEq> Widget for SelectableRadio<'_, T> {
    fn ui(self, ui: &mut Ui) -> Response {
        let mut response = self.radio.ui(ui);
        if response.clicked() && *self.current != self.value {
            *self.current = self.value;
            response.mark_changed();
        }
        response
    }
}
