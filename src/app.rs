use serde::{Deserialize, Serialize};

use crate::{panes::Files, tiles};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    tree: egui_tiles::Tree<tiles::Pane>,

    #[serde(skip)]
    behavior: tiles::TreeBehavior,
}

impl Default for App {
    fn default() -> Self {
        let tree = tiles::create_tree();

        Self {
            tree,
            behavior: tiles::TreeBehavior::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    if ui
                        .add(
                            egui::Button::new("Import file").shortcut_text(
                                ui.ctx().format_shortcut(&Files::IMPORT_FILE_SHORTCUT),
                            ),
                        )
                        .clicked()
                    {
                        Files::import_file_dialog(&mut self.behavior.files);
                        ui.close_menu();
                    };
                    if !is_web && ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Window", |ui| {
                    ui.menu_button("Theme", |ui| {
                        egui::widgets::global_theme_preference_buttons(ui);
                    });
                    if ui.button("Reset window layout").clicked() {
                        self.tree = tiles::create_tree();
                        ui.close_menu();
                    }
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::warn_if_debug_build(ui);
                if !cfg!(debug_assertions) {
                    ui.label(format!("v{}", env!("CARGO_PKG_VERSION")));
                }
                ui.separator();
                powered_by_egui_and_eframe(ui);
            });
        });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(0.)
                    .fill(ctx.style().visuals.window_fill()),
            )
            .show(ctx, |ui| {
                self.tree.ui(&mut self.behavior, ui);
            });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
