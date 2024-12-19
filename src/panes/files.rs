use egui::Modifiers;
use std::{
    fs,
    sync::mpsc::{channel, Receiver, Sender},
};

struct File {
    file_name: String,
    bytes: Vec<u8>,
}

pub struct Files {
    files: Vec<File>,
    text_channel: (Sender<File>, Receiver<File>),
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
        if let Ok(file) = self.text_channel.1.try_recv() {
            self.files.push(file);
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

                    let bytes = if let Some(path) = &file.path {
                        fs::read(path).unwrap_or_else(|_| Vec::new())
                    } else {
                        file.bytes.clone().unwrap_or_else(|| [].into()).to_vec()
                    };

                    self.files.push(File { file_name, bytes });
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
            ui.label(format!(
                "File: {}, bytes: {}",
                file.file_name,
                file.bytes.len()
            ));
        }
    }

    pub fn import_file(&mut self, ui: &mut egui::Ui) {
        let sender = self.text_channel.0.clone();
        let ctx = ui.ctx().clone();
        async_std::task::block_on(async move {
            if let Some(file) = rfd::AsyncFileDialog::new().pick_file().await {
                let file_name = file.file_name();
                let _ = sender.send(File {
                    file_name,
                    bytes: file.read().await,
                });
                ctx.request_repaint();
            }
        });
    }
}
