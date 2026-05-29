//! `Card` — a rounded surface that groups related content.
//!
//! Doubles as gpui-component's section/`GroupBox` container: pass an optional
//! [`title`](Card::title) / [`description`](Card::description) to render a
//! header, then add the body in the `show` closure, just like
//! [`List`](crate::list::List).
//!
//! Mirroring upstream `GroupBox`, the surface uses one of three mutually
//! exclusive styles — it never combines a fill *and* a border (doing so makes
//! the border invisible in themes where the muted surface and border share a
//! color, e.g. the default dark theme):
//!
//! * [`CardVariant::Fill`] (default) — filled `muted_background`, no border.
//! * [`CardVariant::Outline`] — a border, no fill.
//! * [`CardVariant::Normal`] — neither; just padded content.
//!
//! ```ignore
//! sc::Card::new()
//!     .title("Account")
//!     .description("Manage your profile settings.")
//!     .show(ui, |ui| {
//!         ui.add(sc::Input::new(&mut name));
//!     });
//! ```

use egui::{Color32, Frame, InnerResponse, Margin, Stroke, Ui};
use egui_components_theme::Theme;

use crate::common::Size;
use crate::label::Label;
use crate::separator::Separator;

/// How a [`Card`]'s surface is drawn. Matches upstream `GroupBoxVariant`:
/// a card is either filled *or* outlined, never both.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CardVariant {
    /// Filled `muted_background` surface with no border (default).
    #[default]
    Fill,
    /// A border with a transparent fill.
    Outline,
    /// Neither fill nor border — just padded content.
    Normal,
}

/// A surface container with an optional header (title + description).
pub struct Card {
    title: Option<String>,
    description: Option<String>,
    padding: f32,
    /// Draw a separator between the header and the body.
    divider: bool,
    variant: CardVariant,
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Card {
    pub fn new() -> Self {
        Self {
            title: None,
            description: None,
            padding: 16.0,
            divider: false,
            variant: CardVariant::default(),
        }
    }

    pub fn title(mut self, t: impl Into<String>) -> Self {
        self.title = Some(t.into());
        self
    }

    pub fn description(mut self, d: impl Into<String>) -> Self {
        self.description = Some(d.into());
        self
    }

    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    /// Render a horizontal separator between the header and the body.
    pub fn divider(mut self) -> Self {
        self.divider = true;
        self
    }

    pub fn variant(mut self, v: CardVariant) -> Self {
        self.variant = v;
        self
    }
    /// Filled surface, no border (the default).
    pub fn fill(self) -> Self {
        self.variant(CardVariant::Fill)
    }
    /// Bordered, transparent fill.
    pub fn outline(self) -> Self {
        self.variant(CardVariant::Outline)
    }
    /// No fill and no border.
    pub fn normal(self) -> Self {
        self.variant(CardVariant::Normal)
    }

    /// Render the card frame and run `body` inside it, returning the body's
    /// value alongside the frame [`egui::Response`].
    pub fn show<R>(self, ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;

        let (fill, stroke) = match self.variant {
            CardVariant::Fill => (c.muted_background, Stroke::NONE),
            CardVariant::Outline => (Color32::TRANSPARENT, theme.border_stroke()),
            CardVariant::Normal => (Color32::TRANSPARENT, Stroke::NONE),
        };

        Frame::new()
            .fill(fill)
            .stroke(stroke)
            .corner_radius(theme.corner_lg())
            .inner_margin(Margin::same(self.padding as i8))
            .show(ui, |ui| {
                let has_header = self.title.is_some() || self.description.is_some();
                if let Some(title) = self.title {
                    ui.add(Label::new(title).strong().size(Size::Large));
                }
                if let Some(desc) = self.description {
                    ui.add(Label::new(desc).muted().size(Size::Small));
                }
                if has_header {
                    if self.divider {
                        ui.add_space(self.padding * 0.5);
                        ui.add(Separator::horizontal());
                    }
                    ui.add_space(self.padding * 0.75);
                }
                body(ui)
            })
    }
}
