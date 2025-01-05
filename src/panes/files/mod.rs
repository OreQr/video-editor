mod filters;

use egui_taffy::{
    taffy::{
        self,
        prelude::{fr, length, percent},
        Overflow,
    },
    tui, TuiBuilderLogic,
};
use filters::{Filters, IMAGE_FILTER, SOUND_FILTER, VIDEO_FILTER};
use std::fs;

use super::PaneBehavior;

#[derive(Debug)]
pub enum FileType {
    Image,
    Video,
    Sound,
}
struct File {
    name: String,
    bytes: Vec<u8>,
    r#type: FileType,
}

struct FileData {
    name: String,
    bytes: Vec<u8>,
    mime: Option<String>,
}

pub struct Files {
    files: Vec<File>,
}
impl Files {
    pub fn default() -> Self {
        Self { files: Vec::new() }
    }

    pub const IMPORT_FILE_SHORTCUT: egui::KeyboardShortcut =
        egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::O);

    pub fn import_file_dialog(&mut self) {
        async_std::task::block_on(async move {
            if let Some(files) = rfd::AsyncFileDialog::new()
                .add_filter(
                    "All",
                    &[&IMAGE_FILTER[..], &VIDEO_FILTER[..], &SOUND_FILTER[..]].concat(),
                )
                .add_filter("Image", &IMAGE_FILTER)
                .add_filter("Video", &VIDEO_FILTER)
                .add_filter("Sound", &SOUND_FILTER)
                .pick_files()
                .await
            {
                for file in files {
                    let name = file.file_name();
                    let bytes = file.read().await;
                    if bytes.is_empty() {
                        continue;
                    }

                    self.handle_file(FileData {
                        name,
                        bytes,
                        mime: None,
                    });
                }
            }
        });
    }

    fn import_ui(&mut self, ui: &mut egui::Ui) {
        ui.ctx().input(|i| {
            if !i.raw.dropped_files.is_empty() {
                let dropped_files = &i.raw.dropped_files;
                for file in dropped_files {
                    let name = if let Some(path) = &file.path {
                        let name = path
                            .file_name()
                            .map(|name| name.to_string_lossy().to_string());
                        if name.is_none() {
                            continue;
                        }
                        name.unwrap()
                    } else if !file.name.is_empty() {
                        file.name.clone()
                    } else {
                        continue;
                    };

                    let bytes = if let Some(path) = &file.path {
                        fs::read(path).unwrap_or_else(|_| Vec::new())
                    } else {
                        file.bytes.clone().unwrap_or_else(|| [].into()).to_vec()
                    };
                    if bytes.is_empty() {
                        continue;
                    }

                    self.handle_file(FileData {
                        name,
                        bytes,
                        mime: Some(file.mime.clone()),
                    });
                }
            }
        });

        if ui.input_mut(|i| i.consume_shortcut(&Self::IMPORT_FILE_SHORTCUT)) {
            self.import_file_dialog();
        }
    }

    fn handle_file(&mut self, file_data: FileData) {
        let file_type = Filters::determinate_type(&file_data);

        match file_type {
            Some(file_type) => println!("File: {}, type: {:?}", file_data.name, file_type),
            None => println!("File type not found: {}", file_data.name),
        }
    }
}

impl PaneBehavior for Files {
    fn ui(&mut self, ui: &mut egui::Ui) {
        // Import UI logic
        self.import_ui(ui);

        // Show imported files
        // TODO: Refactor this in future
        tui(ui, "files-grid")
            .reserve_available_space()
            // Do padding on parent because it overflows
            .style(taffy::Style {
                padding: length(8.),
                size: percent(1.),

                ..Default::default()
            })
            .show(|tui| {
                tui.style(taffy::Style {
                    display: taffy::Display::Grid,

                    grid_template_columns: vec![fr(1.), fr(1.), fr(1.)],
                    grid_auto_rows: vec![length(140.)],
                    padding: length(1.), // For border

                    gap: length(8.),
                    size: percent(1.),

                    overflow: taffy::Point {
                        x: Overflow::Scroll,
                        y: Overflow::Scroll,
                    },

                    ..Default::default()
                })
                .add(|tui| {
                    for file in &self.files {
                        tui.style(taffy::Style {
                            display: taffy::Display::Flex,
                            flex_direction: taffy::FlexDirection::Column,

                            justify_content: Some(taffy::JustifyContent::Center),
                            align_items: Some(taffy::AlignItems::Center),
                            padding: length(4.),
                            gap: length(8.),

                            ..Default::default()
                        })
                        .add_with_border(|tui| {
                            let x = tui.egui_ui().available_width();
                            if x <= 0. {
                                return;
                            }

                            tui.ui(|ui| {
                                // Thumbnail
                                match file.r#type {
                                    FileType::Image => {
                                        ui.add_sized(
                                            [(x - 16.), 100.],
                                            egui::Image::from_bytes(
                                                format!("bytes://{}", file.name),
                                                file.bytes.clone(),
                                            )
                                            .maintain_aspect_ratio(true),
                                        );
                                    }
                                    FileType::Video => {}
                                    FileType::Sound => {}
                                }

                                ui.add_sized(
                                    [(x - 16.), 10.],
                                    egui::Label::new(&file.name).truncate(),
                                );
                            });
                        });
                    }
                })
            })
    }

    fn top_bar_ui(&mut self, ui: &mut egui::Ui) {
        if ui.button("Import file").clicked() {
            self.import_file_dialog();
        };
    }
}