//! `Rating` — a row of clickable stars bound to a `&mut u32`.
//!
//! ```ignore
//! ui.add(sc::Rating::new(&mut self.stars).max(5));
//! ui.add(sc::Rating::new(&mut fixed).read_only());  // display only
//! ```

use egui::{pos2, vec2, Color32, Pos2, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::{mix, Theme};

pub struct Rating<'a> {
    value: &'a mut u32,
    max: u32,
    star_size: f32,
    gap: f32,
    read_only: bool,
    color: Option<Color32>,
}

impl<'a> Rating<'a> {
    pub fn new(value: &'a mut u32) -> Self {
        Self {
            value,
            max: 5,
            star_size: 20.0,
            gap: 4.0,
            read_only: false,
            color: None,
        }
    }
    pub fn max(mut self, m: u32) -> Self {
        self.max = m.max(1);
        self
    }
    pub fn star_size(mut self, s: f32) -> Self {
        self.star_size = s;
        self
    }
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }
    pub fn color(mut self, c: Color32) -> Self {
        self.color = Some(c);
        self
    }
}

impl<'a> Widget for Rating<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let fill = self.color.unwrap_or(c.warning_background);
        let empty = mix(c.muted_foreground, c.background, 0.4);

        let n = self.max as f32;
        let total = vec2(n * self.star_size + (n - 1.0) * self.gap, self.star_size);
        let sense = if self.read_only {
            Sense::hover()
        } else {
            Sense::click()
        };
        let (rect, mut response) = ui.allocate_exact_size(total, sense);

        // Which star is under the pointer (1-based), if hovering interactively.
        let hover_index = if !self.read_only && response.hovered() {
            response.hover_pos().map(|p| {
                let rel = (p.x - rect.left()) / (self.star_size + self.gap);
                (rel.floor() as i64 + 1).clamp(1, self.max as i64) as u32
            })
        } else {
            None
        };

        if let Some(h) = hover_index {
            if response.clicked() {
                // Click the already-selected single star to clear it.
                *self.value = if *self.value == h { 0 } else { h };
                response.mark_changed();
            }
        }

        let shown = hover_index.unwrap_or(*self.value);

        if ui.is_rect_visible(rect) {
            for i in 0..self.max {
                let cx = rect.left() + self.star_size * 0.5 + i as f32 * (self.star_size + self.gap);
                let center = pos2(cx, rect.center().y);
                let filled = i < shown;
                draw_star(
                    ui.painter(),
                    center,
                    self.star_size * 0.5,
                    if filled { fill } else { Color32::TRANSPARENT },
                    Stroke::new(1.4, if filled { fill } else { empty }),
                );
            }
            if !self.read_only && response.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        }

        response
    }
}

fn draw_star(painter: &egui::Painter, center: Pos2, radius: f32, fill: Color32, stroke: Stroke) {
    let inner = radius * 0.4;
    let mut pts = Vec::with_capacity(10);
    let start = -std::f32::consts::FRAC_PI_2;
    for k in 0..10 {
        let r = if k % 2 == 0 { radius } else { inner };
        let a = start + std::f32::consts::PI * (k as f32) / 5.0;
        let (s, co) = a.sin_cos();
        pts.push(center + vec2(co, s) * r);
    }
    painter.add(egui::Shape::convex_polygon(pts, fill, stroke));
}
