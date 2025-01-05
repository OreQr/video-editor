use super::PaneBehavior;

pub struct Timeline {}
impl PaneBehavior for Timeline {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Timeline");
    }
}
