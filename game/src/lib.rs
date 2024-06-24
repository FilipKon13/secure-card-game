pub mod clickable;
pub mod hand;
pub mod image_database;
pub mod stack;

pub mod game_state;
pub mod gui_printer;
pub mod lobby_scene;
pub mod table_scene;
pub mod window;
pub trait Scene {
    fn update(&mut self);
}
