//! `Table` — a themed wrapper around [`egui_extras::TableBuilder`].
//!
//! gpui-component's upstream `Table` is a large, virtualization-heavy widget;
//! rather than re-implementing column layout and scrolling from scratch this
//! port delegates the heavy lifting to [`egui_extras`] and layers the
//! `egui-components` look on top: a rounded bordered surface, a muted header
//! with a divider, striped rows, and theme-aware hover / selection highlights.
//!
//! State (which row is selected) lives on the caller, exactly like
//! [`ListItem`](crate::list::ListItem) and [`Checkbox`](crate::checkbox::Checkbox).
//!
//! ```ignore
//! use egui_components as sc;
//!
//! sc::Table::new("users")
//!     .column(sc::TableColumn::auto().header("Name").at_least(120.0))
//!     .column(sc::TableColumn::remainder().header("Email"))
//!     .column(sc::TableColumn::initial(64.0).header("Age").align(egui::Align::RIGHT))
//!     .resizable(true)
//!     .selectable(true)
//!     .show(ui, |mut body| {
//!         for (i, user) in users.iter().enumerate() {
//!             let row = body.row(|mut row| {
//!                 row.selected(selected == Some(i));
//!                 row.col(|ui| { ui.label(&user.name); });
//!                 row.col(|ui| { ui.label(&user.email); });
//!                 row.col(|ui| { ui.label(user.age.to_string()); });
//!             });
//!             if row.clicked() {
//!                 selected = Some(i);
//!             }
//!         }
//!     });
//! ```
//!
//! For large data sets use [`TableBodyUi::rows`], which only builds the rows
//! that are actually visible:
//!
//! ```ignore
//! sc::Table::new("big")
//!     .column(sc::TableColumn::remainder().header("Row"))
//!     .show(ui, |mut body| {
//!         body.rows(100_000, |index, mut row| {
//!             row.col(|ui| { ui.label(index.to_string()); });
//!         });
//!     });
//! ```

use egui::{Align, Frame, Layout, Response, Sense, Stroke, Ui, WidgetText};
use egui_components_theme::Theme;

pub use egui_extras::Column;

/// One column of a [`Table`]: an [`egui_extras::Column`] sizing rule plus the
/// header label and per-cell horizontal alignment used by this port.
pub struct TableColumn {
    column: Column,
    title: WidgetText,
    align: Align,
}

impl TableColumn {
    /// Wrap an existing [`egui_extras::Column`].
    pub fn new(column: Column) -> Self {
        Self {
            column,
            title: WidgetText::default(),
            align: Align::Min,
        }
    }

    /// A column that sizes itself to its content. See [`Column::auto`].
    pub fn auto() -> Self {
        Self::new(Column::auto())
    }

    /// A column that expands to fill the remaining width. See [`Column::remainder`].
    pub fn remainder() -> Self {
        Self::new(Column::remainder())
    }

    /// A column with the given initial width that the user can still resize.
    /// See [`Column::initial`].
    pub fn initial(width: f32) -> Self {
        Self::new(Column::initial(width))
    }

    /// A fixed-width column. See [`Column::exact`].
    pub fn exact(width: f32) -> Self {
        Self::new(Column::exact(width))
    }

    /// The header label drawn for this column.
    pub fn header(mut self, title: impl Into<WidgetText>) -> Self {
        self.title = title.into();
        self
    }

    /// Horizontal alignment of this column's header and body cells
    /// ([`Align::Min`] = left, [`Align::Center`], [`Align::Max`] = right).
    pub fn align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Override whether this specific column can be resized. See [`Column::resizable`].
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.column = self.column.resizable(resizable);
        self
    }

    /// Minimum width this column may shrink to. See [`Column::at_least`].
    pub fn at_least(mut self, minimum: f32) -> Self {
        self.column = self.column.at_least(minimum);
        self
    }

    /// Maximum width this column may grow to. See [`Column::at_most`].
    pub fn at_most(mut self, maximum: f32) -> Self {
        self.column = self.column.at_most(maximum);
        self
    }

    /// Clip cell content that overflows the column instead of letting it
    /// dictate the column width. See [`Column::clip`].
    pub fn clip(mut self, clip: bool) -> Self {
        self.column = self.column.clip(clip);
        self
    }
}

impl From<Column> for TableColumn {
    fn from(column: Column) -> Self {
        Self::new(column)
    }
}

/// A themed table built on top of [`egui_extras::TableBuilder`].
pub struct Table {
    id_salt: egui::Id,
    columns: Vec<TableColumn>,
    striped: bool,
    resizable: bool,
    selectable: bool,
    show_header: bool,
    row_height: Option<f32>,
    header_height: Option<f32>,
    max_height: Option<f32>,
    cell_padding_x: f32,
}

impl Table {
    /// Create a table. `id_salt` is hashed into the inner table/scroll-area id
    /// so several tables can live on the same page without colliding.
    pub fn new(id_salt: impl std::hash::Hash) -> Self {
        Self {
            id_salt: egui::Id::new(id_salt),
            columns: Vec::new(),
            striped: true,
            resizable: false,
            selectable: false,
            show_header: true,
            row_height: None,
            header_height: None,
            max_height: None,
            cell_padding_x: 10.0,
        }
    }

    /// Append a column. Accepts a [`TableColumn`] or a bare
    /// [`egui_extras::Column`] (via `From`).
    pub fn column(mut self, column: impl Into<TableColumn>) -> Self {
        self.columns.push(column.into());
        self
    }

    /// Append several columns at once.
    pub fn columns(mut self, columns: impl IntoIterator<Item = TableColumn>) -> Self {
        self.columns.extend(columns);
        self
    }

    /// Shade alternate rows with the muted surface color (default `true`).
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// Allow the user to drag column borders to resize them (default `false`).
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// Make rows respond to hover and clicks, drawing a hover highlight and
    /// honoring [`TableRowUi::selected`] (default `false`).
    pub fn selectable(mut self, selectable: bool) -> Self {
        self.selectable = selectable;
        self
    }

    /// Draw the column header row (default `true`).
    pub fn show_header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    /// Height of each body row. Defaults to the theme's medium control height.
    pub fn row_height(mut self, height: f32) -> Self {
        self.row_height = Some(height);
        self
    }

    /// Height of the header row. Defaults to [`Table::row_height`].
    pub fn header_height(mut self, height: f32) -> Self {
        self.header_height = Some(height);
        self
    }

    /// Cap the table's visible height; rows beyond it scroll.
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = Some(height);
        self
    }

    /// Horizontal padding applied inside every cell (default `10.0`).
    pub fn cell_padding_x(mut self, padding: f32) -> Self {
        self.cell_padding_x = padding;
        self
    }

    /// Build the table and fill its body via `add_body`.
    pub fn show(self, ui: &mut Ui, add_body: impl FnOnce(TableBodyUi<'_>)) -> Response {
        let theme = Theme::get(ui.ctx());
        let c = theme.colors;
        let m = theme.metrics;

        let row_height = self.row_height.unwrap_or(m.button_height_md);
        let header_height = self.header_height.unwrap_or(row_height);
        let pad = self.cell_padding_x;
        let aligns: Vec<Align> = self.columns.iter().map(|col| col.align).collect();

        Frame::new()
            .fill(c.background)
            .stroke(theme.border_stroke())
            .corner_radius(theme.corner_lg())
            .show(ui, |ui| {
                // Scope the egui style so egui_extras paints with theme colors:
                // striped rows, selection background, selected-text color, and
                // the row hover highlight all read from `Visuals`.
                let v = &mut ui.style_mut().visuals;
                v.faint_bg_color = c.muted_background;
                v.selection.bg_fill = c.secondary_background;
                v.selection.stroke.color = c.foreground;
                v.widgets.hovered.bg_fill = c.accent_background;
                ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 0.0);

                let sense = if self.selectable {
                    Sense::click()
                } else {
                    Sense::hover()
                };

                let mut builder = egui_extras::TableBuilder::new(ui)
                    .id_salt(self.id_salt)
                    .striped(self.striped)
                    .resizable(self.resizable)
                    .sense(sense)
                    .cell_layout(Layout::left_to_right(Align::Center))
                    .auto_shrink([false, true]);
                if let Some(h) = self.max_height {
                    builder = builder.max_scroll_height(h);
                }
                for col in &self.columns {
                    builder = builder.column(col.column);
                }

                let header_color = c.muted_foreground;
                let border = c.border;
                let titles: Vec<WidgetText> =
                    self.columns.iter().map(|col| col.title.clone()).collect();
                // The body filler below moves `aligns`; keep a copy for the header.
                let header_aligns = aligns.clone();

                // `add_body` is `FnOnce`; capture it in one filler used by
                // exactly one of the mutually exclusive branches below.
                let fill_body = move |body: egui_extras::TableBody<'_>| {
                    add_body(TableBodyUi {
                        inner: body,
                        aligns: &aligns,
                        row_height,
                        pad,
                    });
                };

                if self.show_header {
                    builder
                        .header(header_height, |mut header| {
                            for (i, title) in titles.iter().enumerate() {
                                let align =
                                    header_aligns.get(i).copied().unwrap_or(Align::Min);
                                header.col(|ui| {
                                    let rect = ui.max_rect();
                                    cell(ui, align, pad, |ui| {
                                        ui.add(
                                            egui::Label::new(title.clone().color(header_color))
                                                .selectable(false)
                                                .truncate(),
                                        );
                                    });
                                    // Header divider: with zero item-spacing the
                                    // per-cell segments join into one line.
                                    ui.painter().hline(
                                        rect.x_range(),
                                        rect.bottom() - 0.5,
                                        Stroke::new(1.0, border),
                                    );
                                });
                            }
                        })
                        .body(fill_body);
                } else {
                    builder.body(fill_body);
                }
            })
            .response
    }
}

/// The themed body of a [`Table`], handed to the `show` closure. Add rows with
/// [`row`](Self::row) or, for large data sets, [`rows`](Self::rows).
pub struct TableBodyUi<'a> {
    inner: egui_extras::TableBody<'a>,
    aligns: &'a [Align],
    row_height: f32,
    pad: f32,
}

impl<'a> TableBodyUi<'a> {
    /// Add a single row, returning its [`Response`] (clickable when the table
    /// is [`selectable`](Table::selectable)). The closure must add at least one
    /// cell via [`TableRowUi::col`].
    pub fn row(&mut self, add_row: impl FnOnce(TableRowUi<'_, '_, '_>)) -> Response {
        let aligns = self.aligns;
        let pad = self.pad;
        let mut response = None;
        self.inner.row(self.row_height, |mut inner| {
            add_row(TableRowUi {
                inner: &mut inner,
                aligns,
                pad,
            });
            response = Some(inner.response());
        });
        response.expect("a table row must add at least one column")
    }

    /// Add `total_rows` rows, only building the ones currently visible. The
    /// callback receives the row index and the row builder. This consumes the
    /// body, so it is the terminal call (use it instead of [`row`](Self::row)).
    pub fn rows(
        self,
        total_rows: usize,
        mut add_row: impl FnMut(usize, TableRowUi<'_, '_, '_>),
    ) {
        let aligns = self.aligns;
        let pad = self.pad;
        self.inner.rows(self.row_height, total_rows, |mut inner| {
            let index = inner.index();
            add_row(
                index,
                TableRowUi {
                    inner: &mut inner,
                    aligns,
                    pad,
                },
            );
        });
    }

    /// Add rows whose heights are given individually by `heights` (one per
    /// row), instead of the table's uniform [`row_height`](Table::row_height).
    ///
    /// Use this when a row must grow to fit wrapped, multi-line cell
    /// content: measure the content height yourself (e.g. with
    /// [`egui::Ui::fonts`]) and yield it here. Like [`rows`](Self::rows) it
    /// only builds the currently-visible rows and is a terminal call.
    pub fn heterogeneous_rows(
        self,
        heights: impl Iterator<Item = f32>,
        mut add_row: impl FnMut(usize, TableRowUi<'_, '_, '_>),
    ) {
        let aligns = self.aligns;
        let pad = self.pad;
        self.inner.heterogeneous_rows(heights, |mut inner| {
            let index = inner.index();
            add_row(
                index,
                TableRowUi {
                    inner: &mut inner,
                    aligns,
                    pad,
                },
            );
        });
    }
}

/// A single themed [`Table`] row. Add cells with [`col`](Self::col) in column
/// order; mark the row [`selected`](Self::selected) to highlight it.
pub struct TableRowUi<'a, 'b, 'c> {
    inner: &'c mut egui_extras::TableRow<'a, 'b>,
    aligns: &'c [Align],
    pad: f32,
}

impl TableRowUi<'_, '_, '_> {
    /// Highlight this row as selected. Call before adding cells.
    pub fn selected(&mut self, selected: bool) -> &mut Self {
        self.inner.set_selected(selected);
        self
    }

    /// Add the next cell, laying it out with this column's alignment and padding.
    pub fn col(&mut self, add_contents: impl FnOnce(&mut Ui)) -> &mut Self {
        let index = self.inner.col_index();
        let align = self.aligns.get(index).copied().unwrap_or(Align::Min);
        let pad = self.pad;
        self.inner.col(|ui| {
            cell(ui, align, pad, add_contents);
        });
        self
    }

    /// The union [`Response`] of the cells added so far.
    pub fn response(&self) -> Response {
        self.inner.response()
    }

    /// This row's index in the body.
    pub fn index(&self) -> usize {
        self.inner.index()
    }
}

/// Lay out a cell's contents with this column's alignment and horizontal
/// padding, vertically centered. Content is added straight into the cell `ui`
/// (not a detached child) so `egui_extras` can measure it for auto-sizing.
fn cell(ui: &mut Ui, align: Align, pad: f32, add_contents: impl FnOnce(&mut Ui)) {
    match align {
        Align::Min => {
            ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                ui.add_space(pad);
                add_contents(ui);
            });
        }
        Align::Max => {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.add_space(pad);
                add_contents(ui);
            });
        }
        Align::Center => {
            ui.with_layout(
                Layout::centered_and_justified(egui::Direction::LeftToRight),
                add_contents,
            );
        }
    }
}
