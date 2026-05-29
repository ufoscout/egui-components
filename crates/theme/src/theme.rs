use crate::tokens::ThemeColor;
use egui::{Color32, CornerRadius, Stroke};

/// Sizing / spacing / radius / typography tokens.
///
/// These mirror gpui-component's per-component sizing constants but are
/// centralized here so the look stays coherent across components.
#[derive(Clone, Copy, Debug)]
pub struct ThemeMetrics {
    pub radius: f32,
    pub radius_sm: f32,
    pub radius_lg: f32,
    pub border_width: f32,
    pub focus_ring_width: f32,
    pub button_height_sm: f32,
    pub button_height_md: f32,
    pub button_height_lg: f32,
    pub button_padding_x_sm: f32,
    pub button_padding_x_md: f32,
    pub button_padding_x_lg: f32,
    pub input_height: f32,
    pub input_padding_x: f32,
    pub switch_width: f32,
    pub switch_height: f32,
    pub switch_thumb_padding: f32,
    pub checkbox_size: f32,
    pub slider_thumb_radius: f32,
    pub slider_track_height: f32,
    pub font_size_xs: f32,
    pub font_size_sm: f32,
    pub font_size_md: f32,
    pub font_size_lg: f32,
}

impl Default for ThemeMetrics {
    fn default() -> Self {
        Self {
            radius: 6.0,
            radius_sm: 4.0,
            radius_lg: 8.0,
            border_width: 1.0,
            focus_ring_width: 2.0,
            button_height_sm: 28.0,
            button_height_md: 34.0,
            button_height_lg: 40.0,
            button_padding_x_sm: 10.0,
            button_padding_x_md: 14.0,
            button_padding_x_lg: 18.0,
            input_height: 34.0,
            input_padding_x: 10.0,
            switch_width: 36.0,
            switch_height: 20.0,
            switch_thumb_padding: 2.0,
            checkbox_size: 16.0,
            slider_thumb_radius: 9.0,
            slider_track_height: 6.0,
            font_size_xs: 11.0,
            font_size_sm: 13.0,
            font_size_md: 14.0,
            font_size_lg: 16.0,
        }
    }
}

/// The full theme: colors + metrics + mode flag.
#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ThemeColor,
    pub metrics: ThemeMetrics,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl Theme {
    pub const fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ThemeColor::light(),
            metrics: ThemeMetrics {
                radius: 6.0,
                radius_sm: 4.0,
                radius_lg: 8.0,
                border_width: 1.0,
                focus_ring_width: 2.0,
                button_height_sm: 28.0,
                button_height_md: 34.0,
                button_height_lg: 40.0,
                button_padding_x_sm: 10.0,
                button_padding_x_md: 14.0,
                button_padding_x_lg: 18.0,
                input_height: 34.0,
                input_padding_x: 10.0,
                switch_width: 36.0,
                switch_height: 20.0,
                switch_thumb_padding: 2.0,
                checkbox_size: 16.0,
                slider_thumb_radius: 9.0,
                slider_track_height: 6.0,
                font_size_xs: 11.0,
                font_size_sm: 13.0,
                font_size_md: 14.0,
                font_size_lg: 16.0,
            },
        }
    }

    pub const fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ThemeColor::dark(),
            metrics: Self::light().metrics,
        }
    }

    pub fn corner(&self) -> CornerRadius {
        CornerRadius::same(self.metrics.radius as u8)
    }

    pub fn corner_sm(&self) -> CornerRadius {
        CornerRadius::same(self.metrics.radius_sm as u8)
    }

    pub fn corner_lg(&self) -> CornerRadius {
        CornerRadius::same(self.metrics.radius_lg as u8)
    }

    pub fn border_stroke(&self) -> Stroke {
        Stroke::new(self.metrics.border_width, self.colors.border)
    }

    pub fn focus_ring(&self) -> Stroke {
        Stroke::new(self.metrics.focus_ring_width, self.colors.ring)
    }

    pub fn input_border_stroke(&self) -> Stroke {
        Stroke::new(self.metrics.border_width, self.colors.input_border)
    }

    /// Push the theme's typography / colors into the supplied [`egui::Style`].
    ///
    /// Components do *not* require this — they read [`Theme`] directly via
    /// [`Self::install`] so they look correct even when the host app uses its
    /// own egui style. This is a convenience that makes plain `egui::Label`
    /// and built-in widgets blend in with the theme palette.
    pub fn apply_to_style(&self, style: &mut egui::Style) {
        let c = &self.colors;
        let visuals = &mut style.visuals;

        visuals.dark_mode = matches!(self.mode, ThemeMode::Dark);
        visuals.override_text_color = Some(c.foreground);
        visuals.window_fill = c.popover_background;
        visuals.panel_fill = c.background;
        visuals.faint_bg_color = c.muted_background;
        visuals.extreme_bg_color = c.background;
        visuals.code_bg_color = c.muted_background;
        visuals.window_stroke = self.border_stroke();
        visuals.selection.bg_fill = c.selection_background;
        visuals.selection.stroke = Stroke::new(1.0, c.primary_foreground);
        visuals.hyperlink_color = c.link_foreground;

        // Widget visuals — only used by built-in widgets; our own widgets paint manually.
        let radius = self.corner();
        visuals.widgets.noninteractive.bg_fill = c.background;
        visuals.widgets.noninteractive.weak_bg_fill = c.muted_background;
        visuals.widgets.noninteractive.bg_stroke = self.border_stroke();
        visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, c.foreground);
        visuals.widgets.noninteractive.corner_radius = radius;

        visuals.widgets.inactive.bg_fill = c.secondary_background;
        visuals.widgets.inactive.weak_bg_fill = c.secondary_background;
        visuals.widgets.inactive.bg_stroke = Stroke::NONE;
        visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, c.secondary_foreground);
        visuals.widgets.inactive.corner_radius = radius;

        visuals.widgets.hovered.bg_fill = c.secondary_hover_background;
        visuals.widgets.hovered.weak_bg_fill = c.secondary_hover_background;
        visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, c.border);
        visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, c.secondary_foreground);
        visuals.widgets.hovered.corner_radius = radius;

        visuals.widgets.active.bg_fill = c.secondary_active_background;
        visuals.widgets.active.weak_bg_fill = c.secondary_active_background;
        visuals.widgets.active.bg_stroke = Stroke::new(1.0, c.border);
        visuals.widgets.active.fg_stroke = Stroke::new(1.0, c.secondary_foreground);
        visuals.widgets.active.corner_radius = radius;

        visuals.widgets.open.bg_fill = c.secondary_background;
        visuals.widgets.open.weak_bg_fill = c.secondary_background;
        visuals.widgets.open.bg_stroke = self.border_stroke();
        visuals.widgets.open.fg_stroke = Stroke::new(1.0, c.secondary_foreground);
        visuals.widgets.open.corner_radius = radius;

        // Typography
        for (_text_style, font_id) in style.text_styles.iter_mut() {
            // Keep relative ratios but bump base size to our md.
            font_id.size = font_id.size.max(self.metrics.font_size_sm);
        }
        style.spacing.button_padding = egui::vec2(self.metrics.button_padding_x_md, 6.0);
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    }

    /// Convenience: install into the active [`egui::Context`] *and* stash the
    /// theme in `ctx.data` so any component can fetch it via [`Theme::get`].
    pub fn install(self, ctx: &egui::Context) {
        ctx.all_styles_mut(|s| self.apply_to_style(s));
        ctx.data_mut(|d| d.insert_temp(egui::Id::new(THEME_KEY), self));
    }

    /// Fetch the installed theme — or fall back to `Theme::light()` if the
    /// host never called [`Self::install`].
    pub fn get(ctx: &egui::Context) -> Self {
        ctx.data(|d| d.get_temp::<Theme>(egui::Id::new(THEME_KEY)))
            .unwrap_or_else(Self::light)
    }
}

const THEME_KEY: &str = "egui-components-theme/theme";

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

/// Mix two colors by `t` (0.0 = a, 1.0 = b).
pub fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let lerp = |x: u8, y: u8| (x as f32 * (1.0 - t) + y as f32 * t) as u8;
    Color32::from_rgba_unmultiplied(
        lerp(a.r(), b.r()),
        lerp(a.g(), b.g()),
        lerp(a.b(), b.b()),
        lerp(a.a(), b.a()),
    )
}
