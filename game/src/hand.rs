use egui::Vec2;
use egui::Memory;
use eframe::egui;

use crate::card::Card;

#[derive(Clone, Default)]
pub struct Hand<'a> {
    cards: Option<Vec<Card<'a>>>,
}

impl<'a> Hand<'a> {
    pub fn new(cards: Vec<Card<'a>>) -> Hand<'a> {
        // let mut cards = Vec::<Card>::new();
        // for image in images {
        //     cards.push(Card::new(image));
        // }
        Self {cards: Some(cards)}
    }
}

impl super::View for Hand<'_> {
    fn ui(&mut self, ui: &mut ::egui::Ui, rect: egui::Rect) {
        let rect_down = rect.clone().split_top_bottom_at_fraction(0.7).1;
        let center = (self.cards.as_ref().unwrap().len() as f32) / 2.0;
        for (i, card) in self.cards.as_ref().unwrap().iter().enumerate() {
            ui.put(
                rect_down.clone().translate(Vec2::new(30.0 * ((i as f32) - center), 0.0)),
                card.image.clone().unwrap().rounding(5.0).max_size(Vec2::new(600.0, 300.0)).rotate(0.1 * ((i as f32) - center), Vec2::new(0.5, 1.0))
            );
        }
    }
}