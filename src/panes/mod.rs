mod files;
pub use files::Files;
mod timeline;
pub use timeline::Timeline;
mod video;
pub use video::Video;

pub trait PaneBehavior {
    fn ui(&mut self, ui: &mut egui::Ui);
    fn top_bar_ui(&mut self, _ui: &mut egui::Ui) {}
}
