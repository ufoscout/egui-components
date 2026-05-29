//! `Form` — vertical labeled-field layout.
//!
//! Render fields inside the [`show`](Form::show) closure; each gets a label
//! (with an optional required marker) above the input, plus an optional
//! description line below.
//!
//! ```ignore
//! sc::Form::new().show(ui, |form| {
//!     form.field("Email", |ui| { ui.add(sc::Input::new(&mut email)); });
//!     form.field_with_hint("Password", "At least 8 characters", |ui| {
//!         ui.add(sc::Input::new(&mut pw).password(true));
//!     });
//!     form.required("Full name", |ui| { ui.add(sc::Input::new(&mut name)); });
//! });
//! ```

use egui::Ui;

use crate::common::Size;
use crate::label::{Label, LabelTone};

pub struct Form {
    field_gap: f32,
}

impl Default for Form {
    fn default() -> Self {
        Self::new()
    }
}

impl Form {
    pub fn new() -> Self {
        Self { field_gap: 14.0 }
    }
    /// Vertical space between fields.
    pub fn field_gap(mut self, gap: f32) -> Self {
        self.field_gap = gap;
        self
    }

    pub fn show(self, ui: &mut Ui, build: impl FnOnce(&mut FormUi)) {
        let mut form = FormUi {
            ui,
            field_gap: self.field_gap,
            first: true,
        };
        build(&mut form);
    }
}

/// Builder handed to the [`Form::show`] closure.
pub struct FormUi<'a> {
    ui: &'a mut Ui,
    field_gap: f32,
    first: bool,
}

impl FormUi<'_> {
    fn header(&mut self, label: &str, required: bool) {
        if !self.first {
            self.ui.add_space(self.field_gap);
        }
        self.first = false;
        self.ui.horizontal(|ui| {
            ui.add(Label::new(label.to_string()).strong().size(Size::Small));
            if required {
                ui.add(
                    Label::new("*")
                        .tone(LabelTone::Danger)
                        .size(Size::Small),
                );
            }
        });
        self.ui.add_space(4.0);
    }

    /// A labeled field.
    pub fn field(&mut self, label: impl AsRef<str>, content: impl FnOnce(&mut Ui)) {
        self.header(label.as_ref(), false);
        content(self.ui);
    }

    /// A labeled field with a required marker.
    pub fn required(&mut self, label: impl AsRef<str>, content: impl FnOnce(&mut Ui)) {
        self.header(label.as_ref(), true);
        content(self.ui);
    }

    /// A labeled field with a muted hint line below the input.
    pub fn field_with_hint(
        &mut self,
        label: impl AsRef<str>,
        hint: impl Into<String>,
        content: impl FnOnce(&mut Ui),
    ) {
        self.header(label.as_ref(), false);
        content(self.ui);
        self.ui.add_space(3.0);
        self.ui.add(Label::new(hint.into()).muted().size(Size::Small));
    }

    /// Escape hatch: arbitrary content between fields (e.g. a separator or a
    /// submit-button row), with the standard field gap applied.
    pub fn raw(&mut self, content: impl FnOnce(&mut Ui)) {
        if !self.first {
            self.ui.add_space(self.field_gap);
        }
        self.first = false;
        content(self.ui);
    }

    /// Access the underlying [`Ui`] directly.
    pub fn ui(&mut self) -> &mut Ui {
        self.ui
    }
}
