
use eframe::egui;
use std::{collections::HashMap};


pub struct image_database<'a> {
    pub image_map: HashMap::<String, egui::Image<'a>>,
}

impl Default for image_database<'_> {
    fn default() -> Self {
        let mut images = HashMap::<String, egui::Image>::new();
        for index in 0..52 {
            let card = common::cards::card_from_index(index);
            let card_name = format!("{}", card);
            let path = format!("assets/fronts/{}.svg", card_name);
            let image = egui::Image::new(format!("file://{path}"));

            images.insert(card_name, image);
        }

        {
            let path = "assets/backs/abstract_clouds.svg";
            let image = egui::Image::new(format!("file://{path}"));

            images.insert("back_abstract".to_string(), image);
        }

        {
            let path = "assets/backs/blue.svg";
            let image = egui::Image::new(format!("file://{path}"));

            images.insert("back_blue".to_string(), image);
        }

        {
            let path = "assets/background.jpg";
            let image = egui::Image::new(format!("file://{path}"));

            images.insert("background".to_string(), image);
        }

        

        Self {
            image_map: images,
        }
    }
}

impl image_database<'_> {
    pub fn get_card_image(&self, card: common::cards::Card) -> egui::Image {
        let card_name = format!("{}", card);
        return self.image_map.get(&card_name).unwrap().clone();
    }

    pub fn get_image(&self, name: &str) -> egui::Image {
        return self.image_map.get(name).unwrap().clone();
    }
}