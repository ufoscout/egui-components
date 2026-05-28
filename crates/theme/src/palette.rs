//! Tailwind-style color palette (subset matching gpui-component defaults).
//!
//! Hex values come from `gpui-component/crates/ui/src/theme/default-colors.json`.

use egui::Color32;

const fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

/// Eleven-stop Tailwind color scale (50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950).
#[derive(Clone, Copy)]
pub struct Scale(pub [Color32; 11]);

impl Scale {
    pub const fn get(&self, step: u16) -> Color32 {
        match step {
            50 => self.0[0],
            100 => self.0[1],
            200 => self.0[2],
            300 => self.0[3],
            400 => self.0[4],
            500 => self.0[5],
            600 => self.0[6],
            700 => self.0[7],
            800 => self.0[8],
            900 => self.0[9],
            950 => self.0[10],
            _ => self.0[5],
        }
    }
}

pub const NEUTRAL: Scale = Scale([
    rgb(250, 250, 250),
    rgb(245, 245, 245),
    rgb(229, 229, 229),
    rgb(212, 212, 212),
    rgb(163, 163, 163),
    rgb(115, 115, 115),
    rgb(82, 82, 82),
    rgb(64, 64, 64),
    rgb(38, 38, 38),
    rgb(23, 23, 23),
    rgb(10, 10, 10),
]);

pub const SLATE: Scale = Scale([
    rgb(248, 250, 252),
    rgb(241, 245, 249),
    rgb(226, 232, 240),
    rgb(203, 213, 225),
    rgb(148, 163, 184),
    rgb(100, 116, 139),
    rgb(71, 85, 105),
    rgb(51, 65, 85),
    rgb(30, 41, 59),
    rgb(15, 23, 42),
    rgb(2, 6, 23),
]);

pub const GRAY: Scale = Scale([
    rgb(249, 250, 251),
    rgb(243, 244, 246),
    rgb(229, 231, 235),
    rgb(209, 213, 219),
    rgb(156, 163, 175),
    rgb(107, 114, 128),
    rgb(75, 85, 99),
    rgb(55, 65, 81),
    rgb(31, 41, 55),
    rgb(17, 24, 39),
    rgb(3, 7, 18),
]);

pub const RED: Scale = Scale([
    rgb(254, 242, 242),
    rgb(254, 226, 226),
    rgb(254, 202, 202),
    rgb(252, 165, 165),
    rgb(248, 113, 113),
    rgb(239, 68, 68),
    rgb(220, 38, 38),
    rgb(185, 28, 28),
    rgb(153, 27, 27),
    rgb(127, 29, 29),
    rgb(69, 10, 10),
]);

pub const GREEN: Scale = Scale([
    rgb(240, 253, 244),
    rgb(220, 252, 231),
    rgb(187, 247, 208),
    rgb(134, 239, 172),
    rgb(74, 222, 128),
    rgb(34, 197, 94),
    rgb(22, 163, 74),
    rgb(21, 128, 61),
    rgb(22, 101, 52),
    rgb(20, 83, 45),
    rgb(5, 46, 22),
]);

pub const BLUE: Scale = Scale([
    rgb(239, 246, 255),
    rgb(219, 234, 254),
    rgb(191, 219, 254),
    rgb(147, 197, 253),
    rgb(96, 165, 250),
    rgb(59, 130, 246),
    rgb(37, 99, 235),
    rgb(29, 78, 216),
    rgb(30, 64, 175),
    rgb(30, 58, 138),
    rgb(23, 37, 84),
]);

pub const YELLOW: Scale = Scale([
    rgb(254, 252, 232),
    rgb(254, 249, 195),
    rgb(254, 240, 138),
    rgb(253, 224, 71),
    rgb(250, 204, 21),
    rgb(234, 179, 8),
    rgb(202, 138, 4),
    rgb(161, 98, 7),
    rgb(133, 77, 14),
    rgb(113, 63, 18),
    rgb(66, 32, 6),
]);

pub const CYAN: Scale = Scale([
    rgb(236, 254, 255),
    rgb(207, 250, 254),
    rgb(165, 243, 252),
    rgb(103, 232, 249),
    rgb(34, 211, 238),
    rgb(6, 182, 212),
    rgb(8, 145, 178),
    rgb(14, 116, 144),
    rgb(21, 94, 117),
    rgb(22, 78, 99),
    rgb(8, 51, 68),
]);

pub const PURPLE: Scale = Scale([
    rgb(250, 245, 255),
    rgb(243, 232, 255),
    rgb(233, 213, 255),
    rgb(216, 180, 254),
    rgb(192, 132, 252),
    rgb(168, 85, 247),
    rgb(147, 51, 234),
    rgb(126, 34, 206),
    rgb(107, 33, 168),
    rgb(88, 28, 135),
    rgb(59, 7, 100),
]);

pub const WHITE: Color32 = Color32::WHITE;
pub const BLACK: Color32 = Color32::BLACK;
