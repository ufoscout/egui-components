//! Semantic color tokens (light + dark) mirroring `gpui-component`'s
//! `default-theme.json` gpui-component default preset.

use crate::palette::*;
use egui::Color32;

/// Semantic color tokens consumed by the components.
///
/// Field naming follows `gpui-component`'s `ThemeColor` (e.g. `primary_foreground`
/// for `primary.foreground` in the upstream JSON), so cross-referencing the
/// original sources stays easy.
#[derive(Clone, Copy, Debug)]
pub struct ThemeColor {
    // Base surface
    pub background: Color32,
    pub foreground: Color32,
    pub border: Color32,
    pub ring: Color32,
    pub overlay: Color32,
    pub caret: Color32,
    pub selection_background: Color32,

    // Primary
    pub primary_background: Color32,
    pub primary_foreground: Color32,
    pub primary_hover_background: Color32,
    pub primary_active_background: Color32,

    // Secondary
    pub secondary_background: Color32,
    pub secondary_foreground: Color32,
    pub secondary_hover_background: Color32,
    pub secondary_active_background: Color32,

    // Accent (used by ghost / outline hover)
    pub accent_background: Color32,
    pub accent_foreground: Color32,

    // Muted (subdued text/surfaces)
    pub muted_background: Color32,
    pub muted_foreground: Color32,

    // Danger / Success / Warning / Info
    pub danger_background: Color32,
    pub danger_foreground: Color32,
    pub success_background: Color32,
    pub success_foreground: Color32,
    pub warning_background: Color32,
    pub warning_foreground: Color32,
    pub info_background: Color32,
    pub info_foreground: Color32,

    // Inputs
    pub input_border: Color32,

    // Popover
    pub popover_background: Color32,
    pub popover_foreground: Color32,

    // Slider / Switch
    pub slider_bar_background: Color32,
    pub slider_thumb_background: Color32,
    pub switch_background: Color32,

    // Link
    pub link_foreground: Color32,
    pub link_hover_foreground: Color32,
    pub link_active_foreground: Color32,
}

impl ThemeColor {
    /// Light preset — matches `Default Light` in `default-theme.json`.
    pub const fn light() -> Self {
        Self {
            background: WHITE,
            foreground: NEUTRAL.get(950),
            border: NEUTRAL.get(200),
            ring: NEUTRAL.get(950),
            overlay: Color32::from_rgba_premultiplied(0, 0, 0, 0x0d),
            caret: Color32::from_rgb(0x0a, 0x0a, 0x0a),
            selection_background: Color32::from_rgb(0x55, 0xa0, 0xfc),

            primary_background: NEUTRAL.get(900),
            primary_foreground: NEUTRAL.get(50),
            primary_hover_background: NEUTRAL.get(800),
            primary_active_background: NEUTRAL.get(950),

            secondary_background: NEUTRAL.get(200),
            secondary_foreground: NEUTRAL.get(900),
            secondary_hover_background: NEUTRAL.get(200),
            secondary_active_background: NEUTRAL.get(300),

            accent_background: NEUTRAL.get(100),
            accent_foreground: NEUTRAL.get(900),

            muted_background: NEUTRAL.get(100),
            muted_foreground: NEUTRAL.get(500),

            danger_background: RED.get(500),
            danger_foreground: NEUTRAL.get(50),
            success_background: GREEN.get(500),
            success_foreground: NEUTRAL.get(50),
            warning_background: YELLOW.get(500),
            warning_foreground: NEUTRAL.get(50),
            info_background: CYAN.get(500),
            info_foreground: NEUTRAL.get(50),

            input_border: NEUTRAL.get(200),

            popover_background: WHITE,
            popover_foreground: NEUTRAL.get(950),

            slider_bar_background: Color32::from_rgb(0x17, 0x17, 0x17),
            slider_thumb_background: WHITE,
            switch_background: Color32::from_rgb(0xd4, 0xd4, 0xd4),

            link_foreground: Color32::from_rgb(0x0a, 0x0a, 0x0a),
            link_hover_foreground: Color32::from_rgb(0x40, 0x40, 0x40),
            link_active_foreground: Color32::from_rgb(0x0a, 0x0a, 0x0a),
        }
    }

    /// Dark preset — matches `Default Dark` in `default-theme.json`.
    pub const fn dark() -> Self {
        Self {
            background: NEUTRAL.get(950),
            foreground: NEUTRAL.get(50),
            border: NEUTRAL.get(800),
            ring: NEUTRAL.get(300),
            overlay: Color32::from_rgba_premultiplied(0, 0, 0, 0x33),
            caret: Color32::from_rgb(0xfa, 0xfa, 0xfa),
            selection_background: Color32::from_rgb(0x1d, 0x4e, 0xd8),

            primary_background: NEUTRAL.get(50),
            primary_foreground: NEUTRAL.get(900),
            primary_hover_background: NEUTRAL.get(100),
            primary_active_background: NEUTRAL.get(200),

            secondary_background: NEUTRAL.get(800),
            secondary_foreground: NEUTRAL.get(50),
            secondary_hover_background: Color32::from_rgb(0x29, 0x29, 0x29),
            secondary_active_background: Color32::from_rgb(0x21, 0x21, 0x21),

            accent_background: NEUTRAL.get(800),
            accent_foreground: NEUTRAL.get(50),

            muted_background: NEUTRAL.get(800),
            muted_foreground: NEUTRAL.get(400),

            danger_background: RED.get(400),
            danger_foreground: RED.get(600),
            success_background: GREEN.get(400),
            success_foreground: GREEN.get(600),
            warning_background: YELLOW.get(400),
            warning_foreground: YELLOW.get(600),
            info_background: CYAN.get(400),
            info_foreground: CYAN.get(600),

            input_border: Color32::from_rgb(0x2f, 0x2f, 0x2f),

            popover_background: NEUTRAL.get(950),
            popover_foreground: NEUTRAL.get(50),

            slider_bar_background: Color32::from_rgb(0xfa, 0xfa, 0xfa),
            slider_thumb_background: Color32::from_rgb(0x0a, 0x0a, 0x0a),
            switch_background: Color32::from_rgb(0x40, 0x40, 0x40),

            link_foreground: Color32::from_rgb(0xfa, 0xfa, 0xfa),
            link_hover_foreground: Color32::from_rgb(0xff, 0xff, 0xff),
            link_active_foreground: Color32::from_rgb(0xd4, 0xd4, 0xd4),
        }
    }
}
