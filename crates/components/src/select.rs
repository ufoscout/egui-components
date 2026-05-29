//! `Select` / `Combobox` — a dropdown that picks one option from a list.
//!
//! The selected index lives on the caller (`&mut Option<usize>`), mirroring
//! the state-on-the-caller pattern used by [`Checkbox`](crate::checkbox) and
//! [`List`](crate::list). Build it with an id salt (so multiple selects on a
//! page don't collide), a slice of option labels, and the bound selection:
//!
//! ```ignore
//! sc::Select::new("fruit", &mut self.fruit)
//!     .options(["Apple", "Banana", "Cherry"])
//!     .placeholder("Pick a fruit")
//!     .show(ui);
//! ```
//!
//! Add [`searchable`](Select::searchable) (or use [`Select::combobox`]) to get
//! a filter field at the top of the dropdown.

use egui::{
    pos2, vec2, Frame, Id, Margin, Response, Sense, Stroke, Ui,
};
use egui_components_theme::{mix, Theme};

use crate::input::Input;
use crate::list::ListItem;

pub struct Select<'a> {
    id_salt: Id,
    selected: &'a mut Option<usize>,
    options: Vec<String>,
    placeholder: String,
    width: Option<f32>,
    max_dropdown_height: f32,
    disabled: bool,
    searchable: bool,
}

impl<'a> Select<'a> {
    pub fn new(id_salt: impl std::hash::Hash, selected: &'a mut Option<usize>) -> Self {
        Self {
            id_salt: Id::new(id_salt),
            selected,
            options: Vec::new(),
            placeholder: "Select…".to_string(),
            width: None,
            max_dropdown_height: 240.0,
            disabled: false,
            searchable: false,
        }
    }

    /// Shorthand for a searchable select (combobox).
    pub fn combobox(id_salt: impl std::hash::Hash, selected: &'a mut Option<usize>) -> Self {
        Self::new(id_salt, selected).searchable()
    }

    pub fn option(mut self, label: impl Into<String>) -> Self {
        self.options.push(label.into());
        self
    }
    pub fn options<I, S>(mut self, options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.options = options.into_iter().map(Into::into).collect();
        self
    }
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn max_dropdown_height(mut self, h: f32) -> Self {
        self.max_dropdown_height = h;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn searchable(mut self) -> Self {
        self.searchable = true;
        self
    }

    /// Render the select. The returned [`Response`] reports `.changed()` when
    /// the selection changes this frame.
    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = theme.colors;
        let radius = theme.corner();

        let height = m.input_height;
        let width = self
            .width
            .unwrap_or_else(|| ui.available_width().min(240.0));

        let sense = if self.disabled {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, mut response) = ui.allocate_exact_size(vec2(width, height), sense);

        // Derive popup / search-buffer ids from the caller-supplied salt so
        // multiple selects on a page stay distinct and stable across frames.
        let base = ui.make_persistent_id(self.id_salt);
        let popup_id = base.with("popup");
        let search_id = base.with("search");

        let is_open = egui::Popup::is_id_open(ui.ctx(), popup_id);

        // --- Trigger paint ---
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let bg = if self.disabled {
                mix(c.background, c.muted_background, 0.6)
            } else {
                c.background
            };
            painter.rect_filled(rect, radius, bg);

            let border_color = if is_open {
                c.ring
            } else if response.hovered() {
                mix(c.input_border, c.foreground, 0.25)
            } else {
                c.input_border
            };
            painter.rect_stroke(
                rect,
                radius,
                Stroke::new(m.border_width, border_color),
                egui::StrokeKind::Inside,
            );

            // Label / placeholder.
            let chevron_w = 22.0;
            let (text, color) = match self.selected.and_then(|i| self.options.get(i)) {
                Some(label) => (label.clone(), c.foreground),
                None => (self.placeholder.clone(), c.muted_foreground),
            };
            let color = if self.disabled {
                mix(color, c.muted_foreground, 0.5)
            } else {
                color
            };
            let galley = ui.ctx().fonts_mut(|f| {
                f.layout(
                    text,
                    egui::FontId::proportional(m.font_size_md),
                    color,
                    rect.width() - m.input_padding_x * 2.0 - chevron_w,
                )
            });
            ui.painter().galley_with_override_text_color(
                pos2(rect.left() + m.input_padding_x, rect.center().y - galley.size().y * 0.5),
                galley,
                color,
            );

            // Chevron.
            draw_chevron(
                ui.painter(),
                pos2(rect.right() - chevron_w * 0.5 - 2.0, rect.center().y),
                if self.disabled { mix(c.muted_foreground, c.background, 0.4) } else { c.muted_foreground },
                is_open,
            );

            if is_open {
                ui.painter().rect_stroke(
                    rect.expand(2.0),
                    radius,
                    theme.focus_ring(),
                    egui::StrokeKind::Outside,
                );
            }
        }

        if !self.disabled && response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        // --- Dropdown ---
        let mut changed = false;

        let popover_frame = Frame::new()
            .fill(c.popover_background)
            .stroke(theme.border_stroke())
            .corner_radius(radius)
            .inner_margin(Margin::same(4))
            .shadow(egui::epaint::Shadow {
                offset: [0, 4],
                blur: 16,
                spread: 0,
                color: c.overlay,
            });

        egui::Popup::from_toggle_button_response(&response)
            .id(popup_id)
            .width(width)
            .gap(4.0)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .frame(popover_frame)
            .show(|ui| {
                ui.set_width(width - 8.0);

                let mut query = if self.searchable {
                    ui.data_mut(|d| d.get_temp::<String>(search_id))
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                if self.searchable {
                    let r = ui.add(
                        Input::new(&mut query)
                            .placeholder("Search…")
                            .width(width - 8.0),
                    );
                    if r.changed() {
                        ui.data_mut(|d| d.insert_temp(search_id, query.clone()));
                    }
                    // Keep focus in the search box while the dropdown is open.
                    if !r.has_focus() && !ui.memory(|m| m.focused().is_some()) {
                        r.request_focus();
                    }
                    ui.add_space(4.0);
                }

                let needle = query.trim().to_lowercase();
                egui::ScrollArea::vertical()
                    .max_height(self.max_dropdown_height)
                    .show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        let mut any = false;
                        for (i, opt) in self.options.iter().enumerate() {
                            if !needle.is_empty() && !opt.to_lowercase().contains(&needle) {
                                continue;
                            }
                            any = true;
                            let item = ListItem::new(opt.clone())
                                .selected(*self.selected == Some(i))
                                .confirmed(*self.selected == Some(i));
                            if ui.add(item).clicked() {
                                *self.selected = Some(i);
                                changed = true;
                            }
                        }
                        if !any {
                            ui.add(crate::label::Label::new("No results").muted());
                        }
                    });
            });

        if changed {
            response.mark_changed();
            egui::Popup::close_id(ui.ctx(), popup_id);
            ui.data_mut(|d| d.insert_temp(search_id, String::new()));
        }

        response
    }
}

fn draw_chevron(painter: &egui::Painter, center: egui::Pos2, color: egui::Color32, open: bool) {
    let w = 4.5;
    let h = 3.0;
    let stroke = Stroke::new(1.5, color);
    let (top, bottom) = if open { (h, -h) } else { (-h, h) };
    painter.line_segment(
        [pos2(center.x - w, center.y + top), pos2(center.x, center.y + bottom)],
        stroke,
    );
    painter.line_segment(
        [pos2(center.x + w, center.y + top), pos2(center.x, center.y + bottom)],
        stroke,
    );
}
