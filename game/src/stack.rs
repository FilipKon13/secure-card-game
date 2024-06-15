use egui::Vec2;
use eframe::egui;

#[derive(Clone, Default)]
pub struct Stack<'a> {
    size: usize,
    image: Option<egui::Image<'a>>,
}

impl<'a> Stack<'a> {
    pub fn new(size: usize, image: egui::Image<'a>) -> Stack<'a> {
        Self {size: size, image: Some(image)}
    }
}

impl super::View for Stack<'_> {
    fn ui(&mut self, ui: &mut ::egui::Ui, rect: egui::Rect) {
        for i in 1..self.size {
            ui.put(
                rect.clone().translate(Vec2::new(-1.0 * (i as f32),-1.0 * (i as f32))),
                self.image.clone().unwrap().rounding(5.0).max_size(Vec2::new(600.0, 300.0))
            );
        }
    }
}