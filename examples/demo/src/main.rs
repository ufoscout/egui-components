//! Showcase application for `egui-components`.
//!
//! Run with `cargo run -p demo --release`.

use eframe::egui;
use egui_components as sc;
use egui_components_theme::{Theme, ThemeMode};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([960.0, 720.0])
            .with_title("egui-components — demo"),
        ..Default::default()
    };
    eframe::run_native(
        "egui-components-demo",
        options,
        Box::new(|cc| {
            Theme::light().install(&cc.egui_ctx);
            Ok(Box::new(DemoApp::default()))
        }),
    )
}

struct DemoApp {
    mode: ThemeMode,
    text: String,
    password: String,
    checked: bool,
    checked_disabled: bool,
    toggle: bool,
    volume: f32,
    show_tag1: bool,
    show_tag2: bool,
    show_tag3: bool,
    click_counter: usize,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Light,
            text: String::new(),
            password: String::new(),
            checked: true,
            checked_disabled: false,
            toggle: true,
            volume: 0.4,
            show_tag1: true,
            show_tag2: true,
            show_tag3: true,
            click_counter: 0,
        }
    }
}

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let theme = Theme::get(&ctx);

        egui::Panel::top("top")
            .frame(
                egui::Frame::new()
                    .fill(theme.colors.background)
                    .inner_margin(egui::Margin::symmetric(16, 12))
                    .stroke(theme.border_stroke()),
            )
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(
                        sc::Label::new("egui-components")
                            .strong()
                            .size(sc::Size::Large),
                    );
                    ui.add_space(8.0);
                    ui.add(sc::Badge::new("v0.1.0").variant(sc::Variant::Secondary));
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let mut dark = matches!(self.mode, ThemeMode::Dark);
                        let was = dark;
                        ui.add(sc::Switch::new(&mut dark));
                        ui.add(sc::Label::new("Dark mode").muted());
                        if dark != was {
                            self.mode =
                                if dark { ThemeMode::Dark } else { ThemeMode::Light };
                            let new_theme = if dark { Theme::dark() } else { Theme::light() };
                            new_theme.install(&ctx);
                        }
                    });
                });
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme.colors.background)
                    .inner_margin(egui::Margin::same(20)),
            )
            .show_inside(ui, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.section(ui, "Buttons", |this, ui| this.buttons(ui));
                    ui.add_space(20.0);
                    self.section(ui, "Inputs & Toggles", |this, ui| this.inputs_toggles(ui));
                    ui.add_space(20.0);
                    self.section(ui, "Labels & Badges", |this, ui| this.labels_badges(ui));
                    ui.add_space(20.0);
                    self.section(ui, "Tags", |this, ui| this.tags(ui));
                    ui.add_space(20.0);
                    self.section(ui, "Alerts", |this, ui| this.alerts(ui));
                });
            });
    }
}

impl DemoApp {
    fn section(
        &mut self,
        ui: &mut egui::Ui,
        title: &str,
        body: impl FnOnce(&mut Self, &mut egui::Ui),
    ) {
        ui.add(sc::Label::new(title).strong().size(sc::Size::Large));
        ui.add(sc::Separator::horizontal());
        ui.add_space(10.0);
        body(self, ui);
    }

    fn buttons(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            if ui.add(sc::Button::primary("Primary")).clicked() {
                self.click_counter += 1;
            }
            ui.add(sc::Button::secondary("Secondary"));
            ui.add(sc::Button::ghost("Ghost"));
            ui.add(sc::Button::outline("Outline"));
            ui.add(sc::Button::danger("Delete"));
            ui.add(sc::Button::link("Learn more"));
            ui.add(sc::Button::primary("Disabled").disabled(true));
        });
        ui.add_space(8.0);
        ui.horizontal_wrapped(|ui| {
            ui.add(sc::Button::primary("Small").small());
            ui.add(sc::Button::primary("Medium"));
            ui.add(sc::Button::primary("Large").large());
            ui.add(sc::Button::primary("Wide").min_width(160.0));
            ui.add(sc::Button::secondary("Success").variant(sc::Variant::Success));
            ui.add(sc::Button::secondary("Warning").variant(sc::Variant::Warning));
            ui.add(sc::Button::secondary("Info").variant(sc::Variant::Info));
        });
        ui.add_space(6.0);
        ui.add(sc::Label::new(format!("Primary clicks: {}", self.click_counter)).muted());
    }

    fn inputs_toggles(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.add(sc::Label::new("Email"));
                ui.add(
                    sc::Input::new(&mut self.text)
                        .placeholder("you@example.com")
                        .width(260.0),
                );
            });
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.add(sc::Label::new("Password"));
                ui.add(
                    sc::Input::new(&mut self.password)
                        .placeholder("••••••••")
                        .password(true)
                        .width(220.0),
                );
            });
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.add(sc::Label::new("Disabled"));
                let mut s = String::from("read-only value");
                ui.add(sc::Input::new(&mut s).disabled(true).width(180.0));
            });
        });

        ui.add_space(14.0);
        ui.horizontal(|ui| {
            ui.add(sc::Checkbox::new(&mut self.checked, "Accept terms"));
            ui.add_space(20.0);
            ui.add(
                sc::Checkbox::new(&mut self.checked_disabled, "Unavailable").disabled(true),
            );
            ui.add_space(20.0);
            ui.add(sc::Switch::new(&mut self.toggle));
            ui.add(sc::Label::new(if self.toggle {
                "Notifications on"
            } else {
                "Notifications off"
            }));
        });

        ui.add_space(14.0);
        ui.horizontal(|ui| {
            ui.add(sc::Label::new("Volume").muted());
            ui.add(sc::Slider::new(&mut self.volume, 0.0..=1.0).width(280.0));
            ui.add(sc::Label::new(format!("{:>3.0}%", self.volume * 100.0)));
        });
    }

    fn labels_badges(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.add(sc::Label::new("Default"));
            ui.add(sc::Label::new("Muted text").muted());
            ui.add(sc::Label::new("Strong").strong());
            ui.add(sc::Label::new("Italic").italic());
            ui.add(sc::Label::new("Danger").tone(sc::LabelTone::Danger));
            ui.add(sc::Label::new("Success").tone(sc::LabelTone::Success));
            ui.add(sc::Label::new("Warning").tone(sc::LabelTone::Warning));
        });
        ui.add_space(10.0);
        ui.horizontal_wrapped(|ui| {
            ui.add(sc::Badge::new("Primary"));
            ui.add(sc::Badge::new("Secondary").variant(sc::Variant::Secondary));
            ui.add(sc::Badge::new("Success").variant(sc::Variant::Success));
            ui.add(sc::Badge::new("Warning").variant(sc::Variant::Warning));
            ui.add(sc::Badge::new("Danger").variant(sc::Variant::Danger));
            ui.add(sc::Badge::new("Info").variant(sc::Variant::Info));
            ui.add(
                sc::Badge::new("Outlined")
                    .variant(sc::Variant::Danger)
                    .outlined(),
            );
        });
    }

    fn tags(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            if self.show_tag1 {
                let r = sc::Tag::new("rust")
                    .variant(sc::Variant::Info)
                    .closable()
                    .show(ui);
                if r.close_clicked {
                    self.show_tag1 = false;
                }
            }
            if self.show_tag2 {
                let r = sc::Tag::new("egui")
                    .variant(sc::Variant::Success)
                    .closable()
                    .show(ui);
                if r.close_clicked {
                    self.show_tag2 = false;
                }
            }
            if self.show_tag3 {
                let r = sc::Tag::new("gpui")
                    .variant(sc::Variant::Warning)
                    .closable()
                    .show(ui);
                if r.close_clicked {
                    self.show_tag3 = false;
                }
            }
            sc::Tag::new("permanent")
                .variant(sc::Variant::Secondary)
                .show(ui);
            if !(self.show_tag1 && self.show_tag2 && self.show_tag3)
                && ui.add(sc::Button::ghost("Restore").small()).clicked()
            {
                self.show_tag1 = true;
                self.show_tag2 = true;
                self.show_tag3 = true;
            }
        });
    }

    fn alerts(&mut self, ui: &mut egui::Ui) {
        ui.add(
            sc::Alert::new("This is a plain informational alert.")
                .title("Heads up")
                .info(),
        );
        ui.add_space(8.0);
        ui.add(
            sc::Alert::new("Your changes have been saved.")
                .title("Success")
                .success(),
        );
        ui.add_space(8.0);
        ui.add(
            sc::Alert::new("Disk space is running low.")
                .title("Warning")
                .warning(),
        );
        ui.add_space(8.0);
        ui.add(
            sc::Alert::new("Could not connect to the server.")
                .title("Error")
                .danger(),
        );
    }
}
