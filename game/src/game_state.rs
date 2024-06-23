use std::{cell::RefCell, rc::Rc};

use gtk::ApplicationWindow;

use crate::{lobby_scene, table_scene, Scene};

pub struct Data {
    pub num_players: u32,
    pub player_id: u32,
}

pub trait SceneUpdate {
    fn update(&mut self);
}

#[allow(dead_code)]
pub struct GameStateLobby {
    window: ApplicationWindow,
    current_scene: Box<dyn Scene>,
}

impl GameStateLobby {
    pub fn new(window: ApplicationWindow, data: Rc<RefCell<Data>>) -> Self {
        Self {
            window: window.clone(),
            current_scene: Box::new(lobby_scene::LobbyScene::new(window.clone(), data)),
        }
    }
}

impl SceneUpdate for GameStateLobby {
    fn update(&mut self) {
        self.current_scene.update();
    }
}

#[allow(dead_code)]
pub struct GameStateTable {
    window: ApplicationWindow,
    current_scene: Box<Rc<RefCell<dyn Scene>>>,
}

impl GameStateTable {
    pub fn new(window: ApplicationWindow) -> Self {
        let tt = Rc::new(RefCell::new(table_scene::TableScene::new(window.clone())));
        Self {
            window: window.clone(),
            current_scene: Box::new(tt.clone()),
        }
    }
}

impl SceneUpdate for GameStateTable {
    fn update(&mut self) {
        self.current_scene.borrow_mut().update();
    }
}
