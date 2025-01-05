use serde::{Deserialize, Serialize};

use crate::panes::{Files, PaneBehavior as _, Timeline, Video};

#[derive(Serialize, Deserialize)]
pub enum Pane {
    Files,
    Timeline,
    Video,
}

pub struct TreeBehavior {
    pub files: Files,
    timeline: Timeline,
    video: Video,
}
impl Default for TreeBehavior {
    fn default() -> Self {
        Self {
            files: Files::default(),
            timeline: Timeline {},
            video: Video {},
        }
    }
}

impl egui_tiles::Behavior<Pane> for TreeBehavior {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::Files => "Files",
            Pane::Timeline => "Timeline",
            Pane::Video => "Video",
        }
        .into()
    }

    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        match pane {
            Pane::Files => self.files.ui(ui),
            Pane::Timeline => self.timeline.ui(ui),
            Pane::Video => self.video.ui(ui),
        };

        Default::default()
    }

    fn top_bar_right_ui(
        &mut self,
        _tiles: &egui_tiles::Tiles<Pane>,
        ui: &mut egui::Ui,
        tile_id: egui_tiles::TileId,
        _tabs: &egui_tiles::Tabs,
        _scroll_offset: &mut f32,
    ) {
        ui.add_space(6.);
        match tile_id.0 {
            1 => self.files.top_bar_ui(ui),
            2 => self.timeline.top_bar_ui(ui),
            3 => self.video.top_bar_ui(ui),
            _ => {}
        }
    }

    fn simplification_options(&self) -> egui_tiles::SimplificationOptions {
        egui_tiles::SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }
}

pub fn create_tree() -> egui_tiles::Tree<Pane> {
    let mut tiles = egui_tiles::Tiles::default();

    let files = tiles.insert_pane(Pane::Files);
    let timeline = tiles.insert_pane(Pane::Timeline);
    let video = tiles.insert_pane(Pane::Video);

    let mut inner_top = egui_tiles::Linear {
        children: vec![files, video],
        dir: egui_tiles::LinearDir::Horizontal,
        ..Default::default()
    };
    inner_top.shares.set_share(files, 0.5);

    let top = tiles.insert_container(egui_tiles::Container::Linear(inner_top));

    let mut inner = egui_tiles::Linear {
        children: vec![top, timeline],
        dir: egui_tiles::LinearDir::Vertical,
        ..Default::default()
    };
    inner.shares.set_share(timeline, 0.3);

    let root = tiles.insert_container(egui_tiles::Container::Linear(inner));

    egui_tiles::Tree::new("tree", root, tiles)
}
