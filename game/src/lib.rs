
pub mod card;
pub mod stack;
pub mod hand;
pub mod image_database;

pub trait View {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, rect: egui::Rect);
}

