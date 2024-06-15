use egui::Vec2;
use egui::Memory;
use eframe::egui;

#[derive(Clone, Default)]
pub struct Card<'a> {
    pub image: Option<egui::Image<'a>>,
}

impl<'a> Card<'a> {
    pub fn new(image: egui::Image<'a>) -> Card<'a> {
        Self {image: Some(image)}
    }
}

impl super::View for Card<'_> {
    fn ui(&mut self, ui: &mut ::egui::Ui, rect: egui::Rect) {
        ui.put(
            rect,
            self.image.clone().unwrap().rounding(5.0).max_size(Vec2::new(600.0, 300.0))
        );
    }
}