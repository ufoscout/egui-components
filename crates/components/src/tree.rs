//! Tree widget powered by [`egui_ltreeview`].
//!
//! Our prior hand-rolled tree didn't deliver working keyboard navigation, so
//! we delegate to `egui_ltreeview` (MIT-licensed) which provides keyboard
//! arrows, multi-select, drag-and-drop, and other features for free. Theming
//! is applied automatically — `egui_components_theme::Theme::install` writes
//! into `egui::Visuals`, which is what `egui_ltreeview` reads.
//!
//! Public types are re-exported below so callers can use them as
//! `egui_components::Tree` etc. without depending on `egui_ltreeview`
//! directly.
//!
//! Selected rows in `egui_ltreeview` use `visuals.selection.bg_fill`, which
//! our theme sets to the bright text-selection color. That's correct for text
//! inputs but visually loud for a tree. [`Tree::show_themed`] applies a small
//! local visual tweak so the selected row uses the same soft secondary
//! background that [`crate::ListItem`] uses, keeping the look consistent.
//!
//! ```ignore
//! use egui_components::{Tree, TreeViewBuilder};
//!
//! let id = ui.make_persistent_id("file-tree");
//! let (response, actions) = Tree::show_themed(ui, id, |builder| {
//!     builder.dir("src", "src");
//!     builder.leaf("src/lib.rs", "lib.rs");
//!     builder.close_dir();
//!     builder.leaf("Cargo.toml", "Cargo.toml");
//! });
//! for action in actions {
//!     if let egui_components::TreeAction::SetSelected(ids) = action {
//!         // update your own state from ids
//!     }
//! }
//! ```

use egui::{Id, Response, Ui};
use egui_components_theme::Theme;

pub use egui_ltreeview::{
    Action as TreeAction, Activate, DirPosition, IndentHintStyle, NodeId, RowLayout, TreeView,
    TreeViewBuilder, TreeViewSettings, TreeViewState,
};

/// Convenience alias — `egui_components::Tree::new(id)` returns the upstream
/// [`TreeView`].
pub type Tree<'cm, NodeIdType> = TreeView<'cm, NodeIdType>;

/// Build + show a [`TreeView`] inside a scope that overrides
/// `visuals.selection.bg_fill` to the theme's `secondary_background`, so the
/// selected row matches the look of [`crate::ListItem`]. Returns the upstream
/// `(Response, Vec<TreeAction>)`.
pub fn show_themed<NodeIdType, F>(
    ui: &mut Ui,
    id: Id,
    body: F,
) -> (Response, Vec<TreeAction<NodeIdType>>)
where
    NodeIdType: NodeId + Send + Sync + 'static,
    F: FnOnce(&mut TreeViewBuilder<'_, NodeIdType>),
{
    let theme = Theme::get(ui.ctx());
    let result = ui.scope(|ui| {
        ui.visuals_mut().selection.bg_fill = theme.colors.secondary_background;
        TreeView::new(id).show(ui, body)
    });
    result.inner
}
