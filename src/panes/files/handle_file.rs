use super::{filters::Filters, File, FileData, Files};

impl Files {
    pub fn handle_file(&mut self, file_data: FileData) {
        let file_type = Filters::determinate_type(&file_data);

        if let Some(file_type) = file_type {
            self.files.push(File {
                name: file_data.name,
                bytes: file_data.bytes,
                r#type: file_type,
                video_thumbnail: None,
            });
        } else {
            // TODO: Notify user about wrong file type
        }
    }
}
