use crate::panes::{Files, Timeline, Video};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    tree: egui_tiles::Tree<Pane>,

    #[serde(skip)]
    behavior: TreeBehavior,
}

impl Default for App {
    fn default() -> Self {
        let tree = create_tree();

        Self {
            tree,
            behavior: TreeBehavior {
                files: Files {},
                timeline: Timeline {},
                video: Video {},
            },
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
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                ui.menu_button("File", |ui| {
                    if !is_web && ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Window", |ui| {
                    ui.menu_button("Theme", |ui| {
                        egui::widgets::global_theme_preference_buttons(ui);
                    });
                    if ui.button("Reset window layout").clicked() {
                        self.tree = create_tree();
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
                    .inner_margin(0.0)
                    .fill(ctx.style().visuals.window_fill()),
            )
            .show(ctx, |ui| {
                self.tree.ui(&mut self.behavior, ui);
            });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
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

#[derive(serde::Deserialize, serde::Serialize)]
enum Pane {
    Files,
    Timeline,
    Video,
}

struct TreeBehavior {
    files: Files,
    timeline: Timeline,
    video: Video,
}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::Files => "Files",
            Pane::Timeline => "Timeline",
            Pane::Video => "Video",
        }
        .into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        match pane {
            Pane::Files => self.files.ui(ui),
            Pane::Timeline => self.timeline.ui(ui),
            Pane::Video => self.video.ui(ui),
        };

        Default::default()
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }
}

fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut tiles = egui_tiles::Tiles::default();

    let files = tiles.insert_pane(Pane::Files);
    let timeline = tiles.insert_pane(Pane::Timeline);
    let video = tiles.insert_pane(Pane::Video);

    let mut inner_top = egui_tiles::Linear {
        children: vec![files, video],
        dir: egui_tiles::LinearDir::Horizontal,
        ..Default::default()
    };
    inner_top.shares.set_share(files, 0.5);

    let top = tiles.insert_container(egui_tiles::Container::Linear(inner_top));

    let mut inner = egui_tiles::Linear {
        children: vec![top, timeline],
        dir: egui_tiles::LinearDir::Vertical,
        ..Default::default()
    };
    inner.shares.set_share(timeline, 0.3);

    let root = tiles.insert_container(egui_tiles::Container::Linear(inner));

    egui_tiles::Tree::new("tree", root, tiles)
}
