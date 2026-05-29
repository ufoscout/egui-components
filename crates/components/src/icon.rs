//! `Icon` — a small set of vector icons drawn with the painter.
//!
//! egui bundles no icon font, and gpui-component's Lucide set is far too large
//! to vendor, so this provides the handful of glyphs the other components
//! need (chevrons, check, close, search, …) as stroke-drawn shapes. Each is
//! laid out inside a unit box and scaled to the requested size, so they stay
//! crisp at any scale.
//!
//! ```ignore
//! ui.add(sc::Icon::new(sc::IconKind::Search).size(18.0));
//! ```

use egui::{pos2, vec2, Color32, Pos2, Rect, Response, Sense, Stroke, Ui, Widget};
use egui_components_theme::Theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconKind {
    Check,
    Close,
    ChevronRight,
    ChevronDown,
    ChevronLeft,
    ChevronUp,
    Search,
    Menu,
    Plus,
    Minus,
    Bell,
    Info,
    Warning,
    Error,
    Home,
    Settings,
    User,
    File,
    Folder,
    Trash,
    Star,
    Heart,
}

pub struct Icon {
    kind: IconKind,
    size: f32,
    color: Option<Color32>,
    stroke_width: f32,
}

impl Icon {
    pub fn new(kind: IconKind) -> Self {
        Self {
            kind,
            size: 16.0,
            color: None,
            stroke_width: 1.6,
        }
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    pub fn color(mut self, c: Color32) -> Self {
        self.color = Some(c);
        self
    }
    pub fn stroke_width(mut self, w: f32) -> Self {
        self.stroke_width = w;
        self
    }
}

impl Widget for Icon {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(vec2(self.size, self.size), Sense::hover());
        if ui.is_rect_visible(rect) {
            let color = self
                .color
                .unwrap_or_else(|| Theme::get(ui.ctx()).colors.foreground);
            paint_icon(ui.painter(), self.kind, rect, color, self.stroke_width);
        }
        response
    }
}

/// Paint `kind` inside `rect` — reusable by other components that already own
/// a painter and rect (menus, sidebar, alerts, …).
pub fn paint_icon(painter: &egui::Painter, kind: IconKind, rect: Rect, color: Color32, sw: f32) {
    let stroke = Stroke::new(sw, color);
    // Map unit coordinates (0..1) inside a slightly inset box to screen space.
    let inset = rect.size().min_elem() * 0.12;
    let b = rect.shrink(inset);
    let p = |x: f32, y: f32| -> Pos2 { pos2(b.left() + x * b.width(), b.top() + y * b.height()) };
    let line = |a: Pos2, c: Pos2| {
        painter.line_segment([a, c], stroke);
    };
    let poly = |pts: &[Pos2]| {
        for w in pts.windows(2) {
            painter.line_segment([w[0], w[1]], stroke);
        }
    };

    match kind {
        IconKind::Check => poly(&[p(0.15, 0.55), p(0.42, 0.8), p(0.85, 0.22)]),
        IconKind::Close => {
            line(p(0.2, 0.2), p(0.8, 0.8));
            line(p(0.8, 0.2), p(0.2, 0.8));
        }
        IconKind::ChevronRight => poly(&[p(0.4, 0.2), p(0.7, 0.5), p(0.4, 0.8)]),
        IconKind::ChevronLeft => poly(&[p(0.6, 0.2), p(0.3, 0.5), p(0.6, 0.8)]),
        IconKind::ChevronDown => poly(&[p(0.2, 0.4), p(0.5, 0.7), p(0.8, 0.4)]),
        IconKind::ChevronUp => poly(&[p(0.2, 0.6), p(0.5, 0.3), p(0.8, 0.6)]),
        IconKind::Search => {
            painter.circle_stroke(p(0.42, 0.42), b.width() * 0.26, stroke);
            line(p(0.62, 0.62), p(0.85, 0.85));
        }
        IconKind::Menu => {
            line(p(0.15, 0.28), p(0.85, 0.28));
            line(p(0.15, 0.5), p(0.85, 0.5));
            line(p(0.15, 0.72), p(0.85, 0.72));
        }
        IconKind::Plus => {
            line(p(0.5, 0.18), p(0.5, 0.82));
            line(p(0.18, 0.5), p(0.82, 0.5));
        }
        IconKind::Minus => line(p(0.18, 0.5), p(0.82, 0.5)),
        IconKind::Bell => {
            // Dome + clapper.
            poly(&[
                p(0.25, 0.68),
                p(0.25, 0.45),
                p(0.32, 0.28),
                p(0.5, 0.22),
                p(0.68, 0.28),
                p(0.75, 0.45),
                p(0.75, 0.68),
            ]);
            line(p(0.18, 0.68), p(0.82, 0.68));
            poly(&[p(0.43, 0.78), p(0.5, 0.84), p(0.57, 0.78)]);
        }
        IconKind::Info => {
            painter.circle_stroke(b.center(), b.width() * 0.42, stroke);
            painter.circle_filled(p(0.5, 0.32), sw * 0.8, color);
            line(p(0.5, 0.46), p(0.5, 0.72));
        }
        IconKind::Warning => {
            poly(&[p(0.5, 0.18), p(0.9, 0.82), p(0.1, 0.82), p(0.5, 0.18)]);
            line(p(0.5, 0.4), p(0.5, 0.62));
            painter.circle_filled(p(0.5, 0.72), sw * 0.7, color);
        }
        IconKind::Error => {
            painter.circle_stroke(b.center(), b.width() * 0.42, stroke);
            line(p(0.35, 0.35), p(0.65, 0.65));
            line(p(0.65, 0.35), p(0.35, 0.65));
        }
        IconKind::Home => {
            poly(&[p(0.15, 0.5), p(0.5, 0.18), p(0.85, 0.5)]);
            poly(&[
                p(0.25, 0.45),
                p(0.25, 0.82),
                p(0.75, 0.82),
                p(0.75, 0.45),
            ]);
        }
        IconKind::Settings => {
            painter.circle_stroke(b.center(), b.width() * 0.18, stroke);
            painter.circle_stroke(b.center(), b.width() * 0.4, stroke);
            for k in 0..8 {
                let a = std::f32::consts::TAU * (k as f32) / 8.0;
                let (s, c) = a.sin_cos();
                let inner = b.center() + vec2(c, s) * b.width() * 0.4;
                let outer = b.center() + vec2(c, s) * b.width() * 0.5;
                painter.line_segment([inner, outer], stroke);
            }
        }
        IconKind::User => {
            painter.circle_stroke(p(0.5, 0.35), b.width() * 0.18, stroke);
            poly(&[p(0.22, 0.82), p(0.3, 0.6), p(0.7, 0.6), p(0.78, 0.82)]);
        }
        IconKind::File => {
            poly(&[
                p(0.28, 0.15),
                p(0.6, 0.15),
                p(0.75, 0.3),
                p(0.75, 0.85),
                p(0.28, 0.85),
                p(0.28, 0.15),
            ]);
            poly(&[p(0.6, 0.15), p(0.6, 0.3), p(0.75, 0.3)]);
        }
        IconKind::Folder => {
            poly(&[
                p(0.15, 0.78),
                p(0.15, 0.3),
                p(0.42, 0.3),
                p(0.5, 0.4),
                p(0.85, 0.4),
                p(0.85, 0.78),
                p(0.15, 0.78),
            ]);
        }
        IconKind::Trash => {
            line(p(0.2, 0.28), p(0.8, 0.28));
            poly(&[p(0.4, 0.28), p(0.42, 0.18), p(0.58, 0.18), p(0.6, 0.28)]);
            poly(&[p(0.28, 0.28), p(0.32, 0.85), p(0.68, 0.85), p(0.72, 0.28)]);
        }
        IconKind::Star => {
            let pts = star_points(b.center(), b.width() * 0.45, b.width() * 0.18, 5);
            poly(&pts);
        }
        IconKind::Heart => {
            painter.circle_stroke(p(0.33, 0.38), b.width() * 0.16, stroke);
            painter.circle_stroke(p(0.67, 0.38), b.width() * 0.16, stroke);
            poly(&[p(0.19, 0.45), p(0.5, 0.82), p(0.81, 0.45)]);
        }
    }
}

fn star_points(center: Pos2, outer: f32, inner: f32, points: usize) -> Vec<Pos2> {
    let mut out = Vec::with_capacity(points * 2 + 1);
    let start = -std::f32::consts::FRAC_PI_2;
    for k in 0..points * 2 {
        let r = if k % 2 == 0 { outer } else { inner };
        let a = start + std::f32::consts::PI * (k as f32) / points as f32;
        let (s, c) = a.sin_cos();
        out.push(center + vec2(c, s) * r);
    }
    out.push(out[0]);
    out
}
