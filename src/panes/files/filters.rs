use std::path::Path;

use super::{FileData, FileType};

pub const IMAGE_FILTER: [&str; 5] = ["png", "jpg", "jpeg", "gif", "svg"];
pub const VIDEO_FILTER: [&str; 3] = ["mp4", "mov", "avi"];
pub const SOUND_FILTER: [&str; 3] = ["mp3", "wav", "ogg"];

pub struct Filters {}
impl Filters {
    pub fn determinate_type(file_data: &FileData) -> Option<FileType> {
        let extension = Path::new(&file_data.name)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase());

        // No mime for svgs
        if extension.as_deref() == Some("svg") {
            return Some(FileType::Image);
        }

        let mime_type = file_data
            .mime
            .clone()
            .filter(|m| !m.trim().is_empty())
            .or_else(|| infer::get(&file_data.bytes).map(|kind| kind.mime_type().to_string()));

        Self::check_file_type(extension.as_deref(), mime_type.as_deref())
    }

    fn check_file_type(extension: Option<&str>, mime_type: Option<&str>) -> Option<FileType> {
        match (extension, mime_type) {
            (Some(ext), Some(mime)) => {
                if IMAGE_FILTER.contains(&ext) && mime.starts_with("image/") {
                    Some(FileType::Image)
                } else if VIDEO_FILTER.contains(&ext) && mime.starts_with("video/") {
                    Some(FileType::Video)
                } else if SOUND_FILTER.contains(&ext) && mime.starts_with("audio/") {
                    Some(FileType::Sound)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
