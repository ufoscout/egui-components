//! Showcase application for `egui-components`.
//!
//! Run with `cargo run -p demo --release`.

use eframe::egui;
use egui_components as sc;
use egui_components_theme::{presets, Theme, ThemeMode};

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
    /// Index into [`presets::ALL`]. `None` means a built-in `Theme::light/dark`
    /// is installed (from the dark-mode switch) rather than a bundled preset.
    preset_idx: Option<usize>,
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
    underline_tab: usize,
    pill_tab: usize,
    segmented_tab: usize,
    list_selected: Option<usize>,
    tree_state: sc::TreeState,
    tree_nodes: Vec<sc::TreeNode>,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Light,
            preset_idx: None,
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
            underline_tab: 0,
            pill_tab: 0,
            segmented_tab: 0,
            list_selected: Some(1),
            tree_state: {
                let mut s = sc::TreeState::new();
                s.expand(egui::Id::new("src"));
                s
            },
            tree_nodes: sample_tree(),
        }
    }
}

fn find_label(nodes: &[sc::TreeNode], id: egui::Id) -> Option<&str> {
    for n in nodes {
        if n.id == id {
            return Some(label_text(&n.label));
        }
        if let Some(s) = find_label(&n.children, id) {
            return Some(s);
        }
    }
    None
}

fn label_text(t: &egui::WidgetText) -> &str {
    match t {
        egui::WidgetText::RichText(rt) => rt.text(),
        _ => "",
    }
}

fn sample_tree() -> Vec<sc::TreeNode> {
    use sc::TreeNode;
    vec![
        TreeNode::new("src", "src").with_children(vec![
            TreeNode::new("src/components", "components").with_children(vec![
                TreeNode::new("src/components/button.rs", "button.rs"),
                TreeNode::new("src/components/tabs.rs", "tabs.rs"),
                TreeNode::new("src/components/tree.rs", "tree.rs"),
            ]),
            TreeNode::new("src/lib.rs", "lib.rs"),
            TreeNode::new("src/main.rs", "main.rs"),
        ]),
        TreeNode::new("themes", "themes").with_children(vec![
            TreeNode::new("themes/catppuccin.json", "catppuccin.json"),
            TreeNode::new("themes/gruvbox.json", "gruvbox.json"),
            TreeNode::new("themes/solarized.json", "solarized.json"),
        ]),
        TreeNode::new("Cargo.toml", "Cargo.toml"),
        TreeNode::new("README.md", "README.md"),
    ]
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
                        let opposite = if dark { ThemeMode::Light } else { ThemeMode::Dark };
                        let mode_switch_enabled = match self.preset_idx {
                            None => true,
                            Some(i) => {
                                let family = presets::ALL[i].family;
                                presets::ALL
                                    .iter()
                                    .any(|p| p.family == family && p.theme.mode == opposite)
                            }
                        };
                        ui.add(sc::Switch::new(&mut dark).disabled(!mode_switch_enabled));
                        ui.add(sc::Label::new("Dark mode").muted());
                        if dark != was {
                            let new_mode =
                                if dark { ThemeMode::Dark } else { ThemeMode::Light };
                            self.mode = new_mode;
                            let sibling = self.preset_idx.and_then(|i| {
                                let family = presets::ALL[i].family;
                                presets::ALL
                                    .iter()
                                    .position(|p| p.family == family && p.theme.mode == new_mode)
                            });
                            if let Some(j) = sibling {
                                presets::ALL[j].theme.install(&ctx);
                                self.preset_idx = Some(j);
                            } else {
                                let new_theme =
                                    if dark { Theme::dark() } else { Theme::light() };
                                new_theme.install(&ctx);
                                self.preset_idx = None;
                            }
                        }

                        ui.add_space(16.0);
                        let label = match self.preset_idx {
                            Some(i) => format!("Theme: {} →", presets::ALL[i].name),
                            None => "Theme: Default →".to_string(),
                        };
                        if ui.add(sc::Button::secondary(label)).clicked() {
                            let next = self
                                .preset_idx
                                .map(|i| (i + 1) % presets::ALL.len())
                                .unwrap_or(0);
                            let theme = presets::ALL[next].theme;
                            theme.install(&ctx);
                            self.preset_idx = Some(next);
                            self.mode = theme.mode;
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
                    ui.add_space(20.0);
                    self.section(ui, "Tabs", |this, ui| this.tabs(ui));
                    ui.add_space(20.0);
                    self.section(ui, "List", |this, ui| this.list(ui));
                    ui.add_space(20.0);
                    self.section(ui, "Tree", |this, ui| this.tree(ui));
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

    fn tabs(&mut self, ui: &mut egui::Ui) {
        ui.add(sc::Label::new("Underline").muted());
        ui.add(
            sc::Tabs::new(&mut self.underline_tab)
                .tab("Overview")
                .tab("Activity")
                .tab("Members")
                .disabled_tab("Billing")
                .underline(),
        );
        ui.add_space(10.0);
        ui.add(sc::Label::new("Pill").muted());
        ui.add(
            sc::Tabs::new(&mut self.pill_tab)
                .tabs(["Day", "Week", "Month", "Year"])
                .pill()
                .small(),
        );
        ui.add_space(10.0);
        ui.add(sc::Label::new("Segmented").muted());
        ui.add(
            sc::Tabs::new(&mut self.segmented_tab)
                .tabs(["List", "Board", "Calendar"])
                .segmented(),
        );
        ui.add_space(6.0);
        ui.add(sc::Label::new(format!(
            "Selected — underline: {}, pill: {}, segmented: {}",
            self.underline_tab, self.pill_tab, self.segmented_tab
        )).muted());
    }

    fn list(&mut self, ui: &mut egui::Ui) {
        let items = [
            ("Inbox", "12"),
            ("Drafts", "3"),
            ("Sent", ""),
            ("Spam", "97"),
            ("Trash", ""),
            ("Archive", ""),
        ];
        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(260.0, 240.0),
                egui::Layout::top_down_justified(egui::Align::LEFT),
                |ui| {
                    sc::List::new("demo-list-inbox").max_height(240.0).show(ui, |ui| {
                        for (i, (name, count)) in items.iter().enumerate() {
                            let mut item = sc::ListItem::new(*name)
                                .selected(self.list_selected == Some(i))
                                .confirmed(i == 2);
                            if !count.is_empty() {
                                item = item.secondary(*count);
                            }
                            if ui.add(item).clicked() {
                                self.list_selected = Some(i);
                            }
                        }
                    });
                },
            );
            ui.add_space(16.0);
            ui.vertical(|ui| {
                ui.add(sc::Label::new("Selected").muted());
                let label = self
                    .list_selected
                    .and_then(|i| items.get(i).map(|(n, _)| *n))
                    .unwrap_or("(none)");
                ui.add(sc::Label::new(label).strong().size(sc::Size::Large));
                ui.add_space(8.0);
                ui.add(sc::Label::new("Click a row to select. The 'Sent' row also has the `confirmed` check icon enabled to show off that slot."));
            });
        });
    }

    fn tree(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(300.0, 300.0),
                egui::Layout::top_down_justified(egui::Align::LEFT),
                |ui| {
                    sc::List::new("demo-list-tree").max_height(300.0).show(ui, |ui| {
                        ui.add(
                            sc::Tree::new(&self.tree_nodes, &mut self.tree_state),
                        );
                    });
                },
            );
            ui.add_space(16.0);
            ui.vertical(|ui| {
                ui.add(sc::Label::new("Selected").muted());
                let selected = match self.tree_state.selected {
                    Some(id) => find_label(&self.tree_nodes, id)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "(unknown)".to_string()),
                    None => "(none)".to_string(),
                };
                ui.add(sc::Label::new(selected).strong().size(sc::Size::Large));
                ui.add_space(8.0);
                ui.add(sc::Label::new("Click folders to expand/collapse. Leaves just select."));
                ui.add_space(8.0);
                if ui.add(sc::Button::secondary("Collapse all").small()).clicked() {
                    self.tree_state.expanded.clear();
                }
                if ui.add(sc::Button::secondary("Expand src/").small()).clicked() {
                    self.tree_state.expand(egui::Id::new("src"));
                    self.tree_state.expand(egui::Id::new("src/components"));
                }
            });
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
