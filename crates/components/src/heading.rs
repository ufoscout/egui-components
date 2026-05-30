//! `Heading` — section title typography.
//!
//! A heading renders a strong title at one of four levels (`H1`–`H4`) with an
//! optional muted description line underneath, matching the shadcn/typography
//! section-title pattern. Upstream gpui-component has no dedicated heading type
//! (its `Text` primitive is a full rich-text view); this is the lightweight
//! egui-idiomatic equivalent used to label pages and sections.
//!
//! ```ignore
//! ui.add(sc::Heading::new("Settings"));                       // H2 (default)
//! ui.add(sc::Heading::new("Welcome back").level(sc::HeadingLevel::H1));
//! ui.add(
//!     sc::Heading::new("Notifications")
//!         .level(sc::HeadingLevel::H3)
//!         .description("Choose what you want to be notified about."),
//! );
//! ```

use egui::{FontId, Response, RichText, Ui, Widget};
use egui_components_theme::Theme;

/// Heading level — controls the title's font size (and, transitively, its
/// visual weight in the hierarchy). Smaller number = larger / more prominent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HeadingLevel {
    H1,
    #[default]
    H2,
    H3,
    H4,
}

impl HeadingLevel {
    /// Font size for the title at this level.
    fn font_size(&self) -> f32 {
        match self {
            HeadingLevel::H1 => 30.0,
            HeadingLevel::H2 => 24.0,
            HeadingLevel::H3 => 19.0,
            HeadingLevel::H4 => 16.0,
        }
    }
}

/// A section title with an optional description sub-line.
pub struct Heading {
    text: String,
    level: HeadingLevel,
    description: Option<String>,
}

impl Heading {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            level: HeadingLevel::default(),
            description: None,
        }
    }

    pub fn level(mut self, level: HeadingLevel) -> Self {
        self.level = level;
        self
    }

    pub fn h1(self) -> Self {
        self.level(HeadingLevel::H1)
    }
    pub fn h2(self) -> Self {
        self.level(HeadingLevel::H2)
    }
    pub fn h3(self) -> Self {
        self.level(HeadingLevel::H3)
    }
    pub fn h4(self) -> Self {
        self.level(HeadingLevel::H4)
    }

    /// A muted description rendered on its own line beneath the title.
    pub fn description(mut self, text: impl Into<String>) -> Self {
        self.description = Some(text.into());
        self
    }
}

impl Widget for Heading {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        ui.vertical(|ui| {
            ui.add(egui::Label::new(
                RichText::new(self.text)
                    .color(c.foreground)
                    .font(FontId::proportional(self.level.font_size()))
                    .strong(),
            ));
            if let Some(desc) = self.description {
                ui.add_space(2.0);
                ui.add(egui::Label::new(
                    RichText::new(desc)
                        .color(c.muted_foreground)
                        .font(FontId::proportional(theme.metrics.font_size_sm)),
                ));
            }
        })
        .response
    }
}
