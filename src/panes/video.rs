use super::PaneBehavior;

pub struct Video {}
impl PaneBehavior for Video {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Video");
    }
}
