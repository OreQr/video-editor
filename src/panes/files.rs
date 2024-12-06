use egui::Modifiers;
use std::sync::mpsc::{channel, Receiver, Sender};

struct File {
    file_name: String,
}

pub struct Files {
    files: Vec<File>,
    text_channel: (Sender<String>, Receiver<String>),
}
impl Files {
    pub fn default() -> Self {
        Self {
            files: Vec::new(),
            text_channel: channel(),
        }
    }

    pub const IMPORT_FILE_SHORTCUT: egui::KeyboardShortcut =
        egui::KeyboardShortcut::new(Modifiers::CTRL, egui::Key::O);

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        if let Ok(text) = self.text_channel.1.try_recv() {
            self.files.push(File { file_name: text });
        }

        ui.ctx().input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let dropped_files = &i.raw.dropped_files;
                for file in dropped_files {
                    let file_name = if let Some(path) = &file.path {
                        path.file_name()
                            .map(|name| name.to_string_lossy().to_string())
                            .unwrap_or_else(|| "???".to_owned())
                    } else if !file.name.is_empty() {
                        file.name.clone()
                    } else {
                        "???".to_owned()
                    };

                    self.files.push(File { file_name });
                }
            }
        });

        if ui.input_mut(|i| i.consume_shortcut(&Self::IMPORT_FILE_SHORTCUT)) {
            self.import_file(ui);
        }

        if ui.button("Open file…").clicked() {
            self.import_file(ui);
        };

        for file in &self.files {
            ui.label(format!("File: {}", file.file_name));
        }
    }

    pub fn import_file(&mut self, ui: &mut egui::Ui) {
        let sender = self.text_channel.0.clone();
        let ctx = ui.ctx().clone();
        async_std::task::block_on(async move {
            if let Some(file) = rfd::AsyncFileDialog::new().pick_file().await {
                let file_name = file.file_name();
                let _ = sender.send(file_name);
                ctx.request_repaint();
            }
        });
    }
}
