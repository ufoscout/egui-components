//! Theme-aware `Label`. A thin wrapper around [`egui::Label`] that knows
//! about the theme's semantic text colors (`muted`, `secondary`, `primary`, …).

use crate::common::Size;
use egui::{FontId, Response, RichText, Ui, Widget};
use egui_components_theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LabelTone {
    #[default]
    Default,
    Muted,
    Primary,
    Secondary,
    Danger,
    Success,
    Warning,
}

pub struct Label {
    text: String,
    tone: LabelTone,
    size: Size,
    strong: bool,
    italic: bool,
    underline: bool,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            tone: LabelTone::Default,
            size: Size::Medium,
            strong: false,
            italic: false,
            underline: false,
        }
    }
    pub fn tone(mut self, t: LabelTone) -> Self {
        self.tone = t;
        self
    }
    pub fn muted(self) -> Self {
        self.tone(LabelTone::Muted)
    }
    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
        self
    }
    pub fn strong(mut self) -> Self {
        self.strong = true;
        self
    }
    pub fn italic(mut self) -> Self {
        self.italic = true;
        self
    }
    pub fn underline(mut self) -> Self {
        self.underline = true;
        self
    }
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let color = match self.tone {
            LabelTone::Default => c.foreground,
            LabelTone::Muted => c.muted_foreground,
            LabelTone::Primary => c.primary_background,
            LabelTone::Secondary => c.secondary_foreground,
            LabelTone::Danger => c.danger_background,
            LabelTone::Success => c.success_background,
            LabelTone::Warning => c.warning_background,
        };
        let mut rich = RichText::new(self.text)
            .color(color)
            .font(FontId::proportional(self.size.font_size(&theme.metrics)));
        if self.strong {
            rich = rich.strong();
        }
        if self.italic {
            rich = rich.italics();
        }
        if self.underline {
            rich = rich.underline();
        }
        ui.add(egui::Label::new(rich))
    }
}
