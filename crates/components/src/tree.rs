//! `Tree` widget — a recursively-rendered tree of [`TreeNode`]s with
//! expand/collapse and single-selection state stored in [`TreeState`].
//!
//! Designed for the common case: a static (or rebuilt-each-frame) `Vec<TreeNode>`
//! plus a `TreeState` you persist between frames. Each node carries its own
//! [`egui::Id`] which keys expansion + selection — `id`s must be unique
//! within the tree.
//!
//! ```ignore
//! let nodes = vec![
//!     TreeNode::new("src", "src").with_children(vec![
//!         TreeNode::new("src/lib.rs", "lib.rs"),
//!         TreeNode::new("src/main.rs", "main.rs"),
//!     ]),
//!     TreeNode::new("Cargo.toml", "Cargo.toml"),
//! ];
//! egui_components::Tree::new(&nodes, &mut self.tree_state).show(ui);
//! ```
use std::collections::HashSet;

use egui::{
    pos2, vec2, Color32, FontId, Id, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, Widget,
    WidgetText,
};
use egui_components_theme::{mix, Theme};

/// One node in a [`Tree`]. Leaves have an empty `children` vec.
#[derive(Clone, Debug)]
pub struct TreeNode {
    pub id: Id,
    pub label: WidgetText,
    pub children: Vec<TreeNode>,
    pub disabled: bool,
}

impl TreeNode {
    /// Create a node. `id_source` keys expansion/selection — use a stable
    /// path-like value, not the display label, so renames don't lose state.
    pub fn new(id_source: impl std::hash::Hash, label: impl Into<WidgetText>) -> Self {
        Self {
            id: Id::new(id_source),
            label: label.into(),
            children: Vec::new(),
            disabled: false,
        }
    }

    pub fn with_children(mut self, children: Vec<TreeNode>) -> Self {
        self.children = children;
        self
    }
    pub fn child(mut self, c: TreeNode) -> Self {
        self.children.push(c);
        self
    }
    pub fn disabled(mut self, b: bool) -> Self {
        self.disabled = b;
        self
    }

    pub fn is_folder(&self) -> bool {
        !self.children.is_empty()
    }
}

/// Persistent state for a [`Tree`]: which nodes are expanded and which is
/// currently selected. Owned by the caller — typically a field on the App.
#[derive(Clone, Debug, Default)]
pub struct TreeState {
    pub expanded: HashSet<Id>,
    pub selected: Option<Id>,
}

impl TreeState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_expanded(&self, id: Id) -> bool {
        self.expanded.contains(&id)
    }
    pub fn expand(&mut self, id: Id) {
        self.expanded.insert(id);
    }
    pub fn collapse(&mut self, id: Id) {
        self.expanded.remove(&id);
    }
    pub fn toggle(&mut self, id: Id) {
        if !self.expanded.insert(id) {
            self.expanded.remove(&id);
        }
    }
    pub fn select(&mut self, id: Id) {
        self.selected = Some(id);
    }
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }
}

pub struct Tree<'a> {
    nodes: &'a [TreeNode],
    state: &'a mut TreeState,
    indent: f32,
    row_height: Option<f32>,
}

impl<'a> Tree<'a> {
    pub fn new(nodes: &'a [TreeNode], state: &'a mut TreeState) -> Self {
        Self {
            nodes,
            state,
            indent: 16.0,
            row_height: None,
        }
    }

    /// Horizontal indentation per level (pixels). Defaults to 16.
    pub fn indent(mut self, px: f32) -> Self {
        self.indent = px;
        self
    }

    /// Override the row height. Defaults to ~28px (matches `ListItem`).
    pub fn row_height(mut self, h: f32) -> Self {
        self.row_height = Some(h);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = Theme::get(ui.ctx());
        let row_h = self.row_height.unwrap_or(theme.metrics.button_height_sm.max(28.0));
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 1.0;
            for node in self.nodes {
                render_node(ui, &theme, node, 0, self.indent, row_h, self.state);
            }
        })
        .response
    }
}

impl<'a> Widget for Tree<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}

fn render_node(
    ui: &mut Ui,
    theme: &Theme,
    node: &TreeNode,
    depth: usize,
    indent: f32,
    row_h: f32,
    state: &mut TreeState,
) {
    let c = &theme.colors;
    let font = FontId::proportional(theme.metrics.font_size_md);
    let is_folder = node.is_folder();
    let is_expanded = state.is_expanded(node.id);
    let is_selected = state.selected == Some(node.id);

    let total_w = ui.available_width();
    let sense = if node.disabled { Sense::hover() } else { Sense::click() };
    let (rect, response) = ui.allocate_exact_size(vec2(total_w, row_h), sense);

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();

        // Row background
        let bg = if node.disabled {
            Color32::TRANSPARENT
        } else if is_selected {
            c.secondary_background
        } else if response.hovered() {
            c.accent_background
        } else {
            Color32::TRANSPARENT
        };
        if bg != Color32::TRANSPARENT {
            painter.rect_filled(rect, theme.corner_sm(), bg);
        }
        if is_selected {
            painter.rect_filled(
                Rect::from_min_size(
                    pos2(rect.left(), rect.top() + 4.0),
                    vec2(2.0, rect.height() - 8.0),
                ),
                egui::CornerRadius::same(1),
                c.primary_background,
            );
        }

        // Indentation + chevron
        let chevron_w = 14.0;
        let chevron_x = rect.left() + 6.0 + indent * depth as f32;
        let chevron_center = pos2(chevron_x + chevron_w * 0.5, rect.center().y);
        let icon_color = if node.disabled {
            mix(c.muted_foreground, Color32::TRANSPARENT, 0.3)
        } else {
            c.muted_foreground
        };
        if is_folder {
            draw_chevron(painter, chevron_center, 4.0, is_expanded, icon_color);
        }

        // Label
        let label_galley = node.label.clone().into_galley(
            ui,
            Some(egui::TextWrapMode::Truncate),
            (rect.right() - chevron_x - chevron_w - 6.0).max(0.0),
            font,
        );
        let label_color = if node.disabled {
            mix(c.muted_foreground, Color32::TRANSPARENT, 0.3)
        } else {
            c.foreground
        };
        let label_pos = pos2(
            chevron_x + chevron_w + 4.0,
            rect.center().y - label_galley.size().y * 0.5,
        );
        painter.galley_with_override_text_color(label_pos, label_galley, label_color);

        if response.has_focus() {
            painter.rect_stroke(
                rect.expand(1.0),
                theme.corner_sm(),
                theme.focus_ring(),
                egui::StrokeKind::Outside,
            );
        }

        if !node.disabled && response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }
    }

    if response.clicked() && !node.disabled {
        state.select(node.id);
        if is_folder {
            state.toggle(node.id);
        }
    }

    if is_folder && is_expanded {
        for child in &node.children {
            render_node(ui, theme, child, depth + 1, indent, row_h, state);
        }
    }
}

fn draw_chevron(
    painter: &egui::Painter,
    center: Pos2,
    size: f32,
    expanded: bool,
    color: Color32,
) {
    let stroke = Stroke::new(1.4, color);
    if expanded {
        // ▼ down-pointing
        let a = pos2(center.x - size, center.y - size * 0.35);
        let b = pos2(center.x + size, center.y - size * 0.35);
        let tip = pos2(center.x, center.y + size * 0.65);
        painter.line_segment([a, tip], stroke);
        painter.line_segment([b, tip], stroke);
    } else {
        // ▶ right-pointing
        let a = pos2(center.x - size * 0.35, center.y - size);
        let b = pos2(center.x - size * 0.35, center.y + size);
        let tip = pos2(center.x + size * 0.65, center.y);
        painter.line_segment([a, tip], stroke);
        painter.line_segment([b, tip], stroke);
    }
}

// Suppress unused-warnings on items we may consume in future variants.
#[allow(dead_code)]
fn _unused(_: Vec2) {}
