use gtk::DrawingArea;

use crate::{
    clickable::{Clickable},
    image_database::ImageDatabase,
};

pub struct Stack {
    cards: Vec<Clickable>,
}

impl Stack {
    pub fn new(size: usize, image_database: &ImageDatabase) -> Self {
        let mut clickable_cards = Vec::<Clickable>::new();
        let pixbuf = image_database.get_image("back_blue");

        for i in 0..size {
            let pos_x = 150.0 - ((i as f64) * 1.0);
            let pos_y = 400.0 - ((i as f64) * 1.0);
            clickable_cards.push(Clickable::new(
                "stack".to_string(),
                pos_x,
                pos_y,
                0.0,
                pixbuf.clone(),
            ));
        }
        Self {
            cards: clickable_cards,
        }
    }

    pub fn draw(&self, drawing_area: DrawingArea) {
        for card in self.cards.clone() {
            card.draw(drawing_area.clone());
        }
    }

    pub fn clicked(&self, x: f64, y: f64) -> String {
        for card in self.cards.iter().rev() {
            let tmp = card.clicked(x, y);
            if tmp != *"" {
                return tmp;
            }
        }
        "".to_string()
    }
}
