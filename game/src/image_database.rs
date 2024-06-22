use std::collections::HashMap;

pub struct ImageDatabase {
    pub image_map: HashMap<String, gtk::gdk_pixbuf::Pixbuf>,
}

impl Default for ImageDatabase {
    fn default() -> Self {
        let mut images = HashMap::<String, gtk::gdk_pixbuf::Pixbuf>::new();
        for index in 0..52 {
            let card = common::cards::card_from_index(index);
            let card_name = format!("{}", card);
            let path = format!("game/assets/fronts/{}.svg", card_name);

            let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(path, 200, 400, true).unwrap();

            images.insert(card_name, pixbuf);
        }

        {
            let path = "game/assets/backs/abstract_clouds.svg";
            let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(path, 200, 400, true).unwrap();

            images.insert("back_abstract".to_string(), pixbuf);
        }

        {
            let path = "game/assets/backs/blue.svg";
            let pixbuf = gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(path, 200, 400, true).unwrap();

            images.insert("back_blue".to_string(), pixbuf);
        }

        {
            let path = "game/assets/background.jpg";

            let pixbuf =
                gtk::gdk_pixbuf::Pixbuf::from_file_at_scale(path, 1200, 800, true).unwrap();

            images.insert("background".to_string(), pixbuf);
        }

        Self { image_map: images }
    }
}

impl ImageDatabase {
    pub fn get_card_image(&self, card: common::cards::Card) -> gtk::gdk_pixbuf::Pixbuf {
        let card_name = format!("{}", card);
        return self.image_map.get(&card_name).unwrap().clone();
    }

    pub fn get_image(&self, name: &str) -> gtk::gdk_pixbuf::Pixbuf {
        return self.image_map.get(name).unwrap().clone();
    }
}
