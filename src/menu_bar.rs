use crate::{panes::Files, tiles, App};

impl App {
    pub fn menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
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
                        Files::import_file_dialog(&mut self.behavior.files, ui);
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
    }
}
