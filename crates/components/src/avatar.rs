//! `Avatar` — a circular (or rounded-square) user thumbnail.
//!
//! egui has no built-in image loader, so this port renders the *fallback*
//! representation gpui-component shows while/instead of an image: a colored
//! disc with the user's initials. Build it from a display name with
//! [`Avatar::from_name`] (initials + a deterministic color are derived for
//! you) or set the initials and colors explicitly.
//!
//! ```ignore
//! ui.add(sc::Avatar::from_name("Ada Lovelace").status(sc::AvatarStatus::Online));
//! ```

use egui::{vec2, Color32, FontId, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::{Theme, ThemeColor};

/// Outline shape of the avatar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    /// Rounded square.
    Square,
}

/// Optional presence indicator drawn as a dot in the bottom-right corner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarStatus {
    Online,
    Offline,
    Busy,
    Away,
}

pub struct Avatar {
    initials: String,
    size: f32,
    shape: AvatarShape,
    bg: Option<Color32>,
    fg: Option<Color32>,
    status: Option<AvatarStatus>,
}

impl Avatar {
    /// Create an avatar from already-computed initials (rendered verbatim).
    pub fn new(initials: impl Into<String>) -> Self {
        Self {
            initials: initials.into(),
            size: 40.0,
            shape: AvatarShape::Circle,
            bg: None,
            fg: None,
            status: None,
        }
    }

    /// Derive initials (up to two letters) and a deterministic background
    /// color from a display name.
    pub fn from_name(name: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        let initials: String = name
            .split_whitespace()
            .filter_map(|w| w.chars().next())
            .take(2)
            .collect::<String>()
            .to_uppercase();
        let mut avatar = Self::new(if initials.is_empty() {
            "?".to_string()
        } else {
            initials
        });
        avatar.bg = Some(color_seed(name));
        avatar
    }

    pub fn size(mut self, px: f32) -> Self {
        self.size = px;
        self
    }
    pub fn small(self) -> Self {
        self.size(28.0)
    }
    pub fn large(self) -> Self {
        self.size(56.0)
    }
    pub fn shape(mut self, s: AvatarShape) -> Self {
        self.shape = s;
        self
    }
    pub fn square(self) -> Self {
        self.shape(AvatarShape::Square)
    }
    /// Override the background fill (otherwise theme/seed derived).
    pub fn background(mut self, c: Color32) -> Self {
        self.bg = Some(c);
        self
    }
    /// Override the initials color (otherwise contrast-picked).
    pub fn foreground(mut self, c: Color32) -> Self {
        self.fg = Some(c);
        self
    }
    pub fn status(mut self, s: AvatarStatus) -> Self {
        self.status = Some(s);
        self
    }
}

impl Widget for Avatar {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;

        let (rect, response) = ui.allocate_exact_size(vec2(self.size, self.size), Sense::hover());

        if ui.is_rect_visible(rect) {
            let bg = self.bg.unwrap_or(c.secondary_background);
            let fg = self.fg.unwrap_or_else(|| contrast_on(bg));
            let painter = ui.painter();

            match self.shape {
                AvatarShape::Circle => {
                    painter.circle_filled(rect.center(), self.size * 0.5, bg);
                }
                AvatarShape::Square => {
                    painter.rect_filled(
                        rect,
                        egui::CornerRadius::same((self.size * 0.22) as u8),
                        bg,
                    );
                }
            }

            let font = FontId::proportional(self.size * 0.4);
            let galley = ui
                .ctx()
                .fonts_mut(|f| f.layout_no_wrap(self.initials.clone(), font, fg));
            painter.galley_with_override_text_color(
                rect.center() - galley.size() * 0.5,
                galley,
                fg,
            );

            if let Some(status) = self.status {
                let dot_r = (self.size * 0.16).max(3.5);
                let offset = self.size * 0.5 - dot_r * 0.7;
                let center = rect.center() + vec2(offset, offset);
                // Ring in the surface color so the dot reads against the avatar.
                painter.circle_filled(center, dot_r + theme.metrics.border_width, c.background);
                painter.circle(
                    center,
                    dot_r,
                    status_color(&c, status),
                    Stroke::NONE,
                );
            }
        }

        response
    }
}

fn status_color(c: &ThemeColor, status: AvatarStatus) -> Color32 {
    match status {
        AvatarStatus::Online => c.success_background,
        AvatarStatus::Offline => c.muted_foreground,
        AvatarStatus::Busy => c.danger_background,
        AvatarStatus::Away => c.warning_background,
    }
}

/// Pick black/white initials for legibility against `bg`.
fn contrast_on(bg: Color32) -> Color32 {
    let luminance = 0.299 * bg.r() as f32 + 0.587 * bg.g() as f32 + 0.114 * bg.b() as f32;
    if luminance > 140.0 {
        Color32::from_rgb(0x0a, 0x0a, 0x0a)
    } else {
        Color32::from_rgb(0xfa, 0xfa, 0xfa)
    }
}

/// Deterministic, pleasant background color derived from a name. Uses a fixed
/// hue palette so the same name always yields the same color.
fn color_seed(name: &str) -> Color32 {
    // Small FNV-1a hash — avoids pulling in `DefaultHasher` randomized state.
    let mut hash: u32 = 0x811c_9dc5;
    for b in name.bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(0x0100_0193);
    }
    const PALETTE: [Color32; 6] = [
        Color32::from_rgb(0x3b, 0x82, 0xf6), // blue
        Color32::from_rgb(0x10, 0xb9, 0x81), // green
        Color32::from_rgb(0xf5, 0x9e, 0x0b), // amber
        Color32::from_rgb(0xef, 0x44, 0x44), // red
        Color32::from_rgb(0x8b, 0x5c, 0xf6), // violet
        Color32::from_rgb(0x06, 0xb6, 0xd4), // cyan
    ];
    PALETTE[(hash as usize) % PALETTE.len()]
}
