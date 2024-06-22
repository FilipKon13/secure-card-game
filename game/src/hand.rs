use gtk::DrawingArea;

use crate::{clickable::Clickable, image_database::ImageDatabase};

pub struct Hand {
    cards: Vec<Clickable>,
}

impl Hand {
    pub fn new(cards: Vec<common::cards::Card>, image_database: &ImageDatabase) -> Self {
        let size = cards.len();
        let angle_offset = f64::min(3.0, 60.0 / (size as f64));
        let offset_x = f64::min(30.0, 800.0 / (size as f64));
        let offset_y = f64::min(10.0, 100.0 / (size as f64));
        let center = (size as f64) / 2.0;
        let mut clickable_cards = Vec::<Clickable>::new();

        for (i, card) in cards.iter().enumerate() {
            let pixbuf = image_database.get_card_image(*card);
            let pos_x = 600.0 + offset_x * ((i as f64) - center);
            let pos_y =
                800.0 - (pixbuf.height() as f64) / 2.0 + (f64::abs((i as f64) - center)) * offset_y;
            let angle = angle_offset * ((i as f64) - center);

            clickable_cards.push(Clickable::new(
                card.to_string(),
                pos_x,
                pos_y,
                angle,
                pixbuf,
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
