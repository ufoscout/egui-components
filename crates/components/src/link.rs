//! `Link` — a themed hyperlink.
//!
//! Returns an [`egui::Response`] so it composes like any widget. Give it a
//! [`url`](Link::url) to open in the browser on click, or leave it off and
//! handle `.clicked()` yourself.
//!
//! ```ignore
//! ui.add(sc::Link::new("Documentation").url("https://docs.rs"));
//! if ui.add(sc::Link::new("Run action")).clicked() { /* … */ }
//! ```

use egui::{FontId, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::Theme;

use crate::common::Size;

pub struct Link {
    text: String,
    url: Option<String>,
    size: Size,
    underline: UnderlineMode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum UnderlineMode {
    OnHover,
    Always,
    Never,
}

impl Link {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            url: None,
            size: Size::Medium,
            underline: UnderlineMode::OnHover,
        }
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
        self
    }
    pub fn underline(mut self) -> Self {
        self.underline = UnderlineMode::Always;
        self
    }
    pub fn no_underline(mut self) -> Self {
        self.underline = UnderlineMode::Never;
        self
    }
}

impl Widget for Link {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let font = FontId::proportional(self.size.font_size(&theme.metrics));

        let galley = ui.ctx().fonts_mut(|f| {
            f.layout_no_wrap(self.text.clone(), font, c.link_foreground)
        });
        let (rect, response) = ui.allocate_exact_size(galley.size(), Sense::click());

        if ui.is_rect_visible(rect) {
            let color = if response.is_pointer_button_down_on() {
                c.link_active_foreground
            } else if response.hovered() {
                c.link_hover_foreground
            } else {
                c.link_foreground
            };
            let painter = ui.painter();
            painter.galley_with_override_text_color(rect.min, galley.clone(), color);

            let underline = match self.underline {
                UnderlineMode::Always => true,
                UnderlineMode::OnHover => response.hovered(),
                UnderlineMode::Never => false,
            };
            if underline {
                let y = rect.bottom() - 1.0;
                painter.line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    Stroke::new(1.0, color),
                );
            }
            if response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        }

        if response.clicked() {
            if let Some(url) = &self.url {
                ui.ctx().open_url(egui::OpenUrl::new_tab(url));
            }
        }

        response
    }
}
