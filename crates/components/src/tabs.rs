//! `Tabs` widget — a horizontal tab bar driving a `&mut usize` selection.
//!
//! Three visual variants from gpui-component (`underline`, `pill`, `segmented`).
//! Callers render their own per-tab content based on the resulting selection.

use crate::common::Size;
use egui::{
    pos2, vec2, Color32, FontId, Rect, Response, Sense, Stroke, Ui, Vec2, Widget, WidgetText,
};
use egui_components_theme::{mix, Theme};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TabVariant {
    #[default]
    Underline,
    Pill,
    Segmented,
}

pub struct Tabs<'a> {
    selected: &'a mut usize,
    tabs: Vec<WidgetText>,
    disabled: Vec<bool>,
    variant: TabVariant,
    size: Size,
}

impl<'a> Tabs<'a> {
    pub fn new(selected: &'a mut usize) -> Self {
        Self {
            selected,
            tabs: Vec::new(),
            disabled: Vec::new(),
            variant: TabVariant::default(),
            size: Size::Medium,
        }
    }

    /// Append a single tab.
    pub fn tab(mut self, label: impl Into<WidgetText>) -> Self {
        self.tabs.push(label.into());
        self.disabled.push(false);
        self
    }

    /// Append a disabled tab — still rendered, never selectable via click.
    pub fn disabled_tab(mut self, label: impl Into<WidgetText>) -> Self {
        self.tabs.push(label.into());
        self.disabled.push(true);
        self
    }

    /// Append several tabs at once.
    pub fn tabs<I, T>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<WidgetText>,
    {
        for it in items {
            self.tabs.push(it.into());
            self.disabled.push(false);
        }
        self
    }

    pub fn variant(mut self, v: TabVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn underline(self) -> Self {
        self.variant(TabVariant::Underline)
    }
    pub fn pill(self) -> Self {
        self.variant(TabVariant::Pill)
    }
    pub fn segmented(self) -> Self {
        self.variant(TabVariant::Segmented)
    }

    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
        self
    }
    pub fn small(self) -> Self {
        self.size(Size::Small)
    }
    pub fn large(self) -> Self {
        self.size(Size::Large)
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let m = theme.metrics;
        let c = &theme.colors;
        let font = FontId::proportional(self.size.font_size(&m));
        let height = self.size.button_height(&m);
        let pad_x = match self.size {
            Size::Small => 10.0,
            Size::Medium => 14.0,
            Size::Large => 18.0,
        };
        let gap = match self.variant {
            TabVariant::Underline => 4.0,
            TabVariant::Pill => 4.0,
            TabVariant::Segmented => 2.0,
        };

        if self.tabs.is_empty() {
            return ui.allocate_response(Vec2::ZERO, Sense::hover());
        }
        if *self.selected >= self.tabs.len() {
            *self.selected = 0;
        }

        let galleys: Vec<_> = self
            .tabs
            .iter()
            .map(|t| {
                t.clone().into_galley(
                    ui,
                    Some(egui::TextWrapMode::Extend),
                    f32::INFINITY,
                    font.clone(),
                )
            })
            .collect();

        let widths: Vec<f32> = galleys
            .iter()
            .map(|g| g.size().x + pad_x * 2.0)
            .collect();

        let outer_pad = match self.variant {
            TabVariant::Segmented => 3.0,
            _ => 0.0,
        };
        let total_w: f32 = widths.iter().sum::<f32>()
            + gap * (self.tabs.len() as f32 - 1.0).max(0.0)
            + outer_pad * 2.0;
        let total_h = height + outer_pad * 2.0;

        let (rect, response) = ui.allocate_exact_size(vec2(total_w, total_h), Sense::hover());

        if !ui.is_rect_visible(rect) {
            return response;
        }

        // Container background for segmented variant
        let painter = ui.painter();
        if matches!(self.variant, TabVariant::Segmented) {
            painter.rect_filled(rect, theme.corner(), c.muted_background);
        }
        if matches!(self.variant, TabVariant::Underline) {
            // Bottom border under the whole bar
            painter.line_segment(
                [
                    pos2(rect.left(), rect.bottom() - 1.0),
                    pos2(rect.right(), rect.bottom() - 1.0),
                ],
                Stroke::new(1.0, c.border),
            );
        }

        let mut x = rect.left() + outer_pad;
        let y = rect.top() + outer_pad;
        let mut clicked_idx: Option<usize> = None;

        for (i, galley) in galleys.iter().enumerate() {
            let w = widths[i];
            let tab_rect = Rect::from_min_size(pos2(x, y), vec2(w, height));
            let disabled = self.disabled[i];
            let id = response.id.with(("tab", i));
            let sense = if disabled { Sense::hover() } else { Sense::click() };
            let tab_resp = ui.interact(tab_rect, id, sense);
            let is_selected = *self.selected == i;

            paint_tab(
                ui,
                tab_rect,
                &tab_resp,
                &theme,
                self.variant,
                is_selected,
                disabled,
                galley,
            );

            if tab_resp.clicked() && !disabled {
                clicked_idx = Some(i);
            }
            if !disabled && tab_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            x += w + gap;
        }

        if let Some(i) = clicked_idx {
            *self.selected = i;
        }

        response
    }
}

impl<'a> Widget for Tabs<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}

fn paint_tab(
    ui: &mut Ui,
    rect: Rect,
    response: &Response,
    theme: &Theme,
    variant: TabVariant,
    selected: bool,
    disabled: bool,
    galley: &std::sync::Arc<egui::Galley>,
) {
    let c = &theme.colors;
    let painter = ui.painter();

    let text_color = if disabled {
        mix(c.muted_foreground, Color32::TRANSPARENT, 0.3)
    } else if selected {
        match variant {
            TabVariant::Pill => c.primary_foreground,
            TabVariant::Underline => c.foreground,
            TabVariant::Segmented => c.foreground,
        }
    } else {
        c.muted_foreground
    };

    match variant {
        TabVariant::Underline => {
            // Background: subtle hover when not selected
            if !disabled && !selected && response.hovered() {
                painter.rect_filled(
                    rect.shrink(2.0),
                    theme.corner_sm(),
                    c.accent_background,
                );
            }
            // Selected: 2px underline at the bottom
            if selected {
                let y = rect.bottom() - 1.0;
                let pad = 4.0;
                painter.line_segment(
                    [
                        pos2(rect.left() + pad, y),
                        pos2(rect.right() - pad, y),
                    ],
                    Stroke::new(2.0, c.primary_background),
                );
            }
        }
        TabVariant::Pill => {
            let radius = egui::CornerRadius::same((rect.height() * 0.5) as u8);
            let bg = if selected {
                if disabled {
                    mix(c.primary_background, Color32::TRANSPARENT, 0.5)
                } else if response.is_pointer_button_down_on() {
                    c.primary_active_background
                } else if response.hovered() {
                    c.primary_hover_background
                } else {
                    c.primary_background
                }
            } else if !disabled && response.hovered() {
                c.secondary_hover_background
            } else {
                Color32::TRANSPARENT
            };
            if bg != Color32::TRANSPARENT {
                painter.rect_filled(rect, radius, bg);
            }
        }
        TabVariant::Segmented => {
            if selected {
                painter.rect_filled(rect, theme.corner_sm(), c.popover_background);
                painter.rect_stroke(
                    rect,
                    theme.corner_sm(),
                    Stroke::new(1.0, c.border),
                    egui::StrokeKind::Inside,
                );
            } else if !disabled && response.hovered() {
                painter.rect_filled(rect, theme.corner_sm(), c.accent_background);
            }
        }
    }

    if response.has_focus() {
        painter.rect_stroke(
            rect.expand(1.0),
            theme.corner(),
            theme.focus_ring(),
            egui::StrokeKind::Outside,
        );
    }

    let text_pos = rect.center() - galley.size() * 0.5;
    painter.galley_with_override_text_color(text_pos, galley.clone(), text_color);
}
