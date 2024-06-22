use gtk::ApplicationWindow;

use crate::{lobby_scene, table_scene, Scene};

pub struct Data {
    pub num_players: u32,
    pub player_id: u32,
}

pub struct GameState {
    window: ApplicationWindow,
    current_scene: Box<dyn Scene>,
}

impl GameState {
    pub fn new(window: ApplicationWindow, window_type: String) -> Self {
        if window_type.as_str() == "lobby" {
            return Self {
                window: window.clone(),
                current_scene: Box::new(lobby_scene::LobbyScene::new(window.clone())),
            };
        } else {
            Self {
                window: window.clone(),
                current_scene: Box::new(table_scene::TableScene::new(window.clone())),
            }
        }
    }

    pub fn update(&mut self) {
        self.current_scene.update();
    }

    pub fn set_table_scene(&mut self) {
        self.current_scene = Box::new(table_scene::TableScene::new(self.window.clone()));
    }
}
