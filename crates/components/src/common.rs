//! Shared types used across components.

/// Visual variant — used by [`Button`](crate::button::Button),
/// [`Badge`](crate::badge::Badge), [`Alert`](crate::alert::Alert) and
/// [`Tag`](crate::tag::Tag).
///
/// Mirrors the variants in `gpui-component`'s `ButtonVariant` / `BadgeVariant`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Outline,
    Link,
    Danger,
    Success,
    Warning,
    Info,
}

/// Standard size scale used by sized components.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

impl Size {
    pub fn button_height(&self, m: &egui_components_theme::ThemeMetrics) -> f32 {
        match self {
            Size::Small => m.button_height_sm,
            Size::Medium => m.button_height_md,
            Size::Large => m.button_height_lg,
        }
    }

    pub fn input_height(&self, m: &egui_components_theme::ThemeMetrics) -> f32 {
        match self {
            Size::Small => m.input_height_sm,
            Size::Medium => m.input_height,
            Size::Large => m.input_height_lg,
        }
    }

    pub fn button_padding_x(&self, m: &egui_components_theme::ThemeMetrics) -> f32 {
        match self {
            Size::Small => m.button_padding_x_sm,
            Size::Medium => m.button_padding_x_md,
            Size::Large => m.button_padding_x_lg,
        }
    }

    pub fn font_size(&self, m: &egui_components_theme::ThemeMetrics) -> f32 {
        match self {
            Size::Small => m.font_size_sm,
            Size::Medium => m.font_size_md,
            Size::Large => m.font_size_lg,
        }
    }
}
