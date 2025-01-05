mod filters;
mod handle_file;

use egui_taffy::{
    taffy::{
        self,
        prelude::{fr, length, percent},
        Overflow,
    },
    tui, TuiBuilderLogic,
};
use filters::{IMAGE_FILTER, SOUND_FILTER, VIDEO_FILTER};
use std::{
    fs,
    sync::mpsc::{channel, Receiver, Sender},
};

use super::PaneBehavior;

#[derive(Debug)]
pub enum FileType {
    Image,
    Video,
    Sound,
}

pub struct FileData {
    name: String,
    bytes: Vec<u8>,
    mime: Option<String>,
}

struct File {
    name: String,
    bytes: Vec<u8>,
    r#type: FileType,
    video_thumbnail: Option<Vec<u8>>,
}

pub struct Files {
    files: Vec<File>,
    channel: (Sender<FileData>, Receiver<FileData>),
}
impl Files {
    pub fn default() -> Self {
        Self {
            files: Vec::new(),
            channel: channel(),
        }
    }

    pub const IMPORT_FILE_SHORTCUT: egui::KeyboardShortcut =
        egui::KeyboardShortcut::new(egui::Modifiers::CTRL, egui::Key::O);

    pub fn import_file_dialog(&mut self, ui: &mut egui::Ui) {
        let sender = self.channel.0.clone();
        let ctx = ui.ctx().clone();
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

                    // Send file data by channel because of wasm compilation: E0521
                    let _ = sender.send(FileData {
                        name,
                        bytes,
                        mime: None,
                    });
                    ctx.request_repaint();
                }
            }
        });
    }

    fn import_ui(&mut self, ui: &mut egui::Ui) {
        if let Ok(file_data) = self.channel.1.try_recv() {
            self.handle_file(file_data);
        }

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
            self.import_file_dialog(ui);
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
                                ui.add_sized(
                                    [(x - 16.), 100.],
                                    match file.r#type {
                                        FileType::Image => egui::Image::from_bytes(
                                            format!("bytes://{}", file.name),
                                            file.bytes.clone(),
                                        ),
                                        FileType::Video => {
                                            if let Some(video_thumbnail) = &file.video_thumbnail {
                                                egui::Image::from_bytes(
                                                    format!("bytes://{}", file.name),
                                                    video_thumbnail.clone(),
                                                )
                                            } else {
                                                egui::Image::new(egui::include_image!(
                                                    "../../assets/video.png"
                                                ))
                                            }
                                        }
                                        FileType::Sound => egui::Image::new(egui::include_image!(
                                            "../../assets/sound.png"
                                        )),
                                    }
                                    .maintain_aspect_ratio(true),
                                );

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
            self.import_file_dialog(ui);
        };
    }
}
