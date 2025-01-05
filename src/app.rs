use serde::{Deserialize, Serialize};

use crate::tiles;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Serialize, Deserialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    pub tree: egui_tiles::Tree<tiles::Pane>,

    #[serde(skip)]
    pub behavior: tiles::TreeBehavior,
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
        self.menu_bar(ctx);

        self.footer(ctx);

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
