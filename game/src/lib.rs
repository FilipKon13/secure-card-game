pub mod clickable;
pub mod hand;
pub mod image_database;
pub mod stack;

pub mod table_scene;
pub mod lobby_scene;

pub trait Scene {
    fn update(&mut self);
}