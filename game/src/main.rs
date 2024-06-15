use std::clone;

use common::cards::*;
use eframe::{egui};
use egui::Vec2;
use game::{card::Card, image_database::image_database, View};

fn main() {
    
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "Secure Card Game", 
        native_options, 
        Box::new(
            |cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                Box::new(MyEguiApp::new(cc))
            }
        )
    );
}

#[derive(Default)]
struct MyEguiApp<'a> { 
    pub image_database: image_database<'a>,
}

impl MyEguiApp<'_> {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp<'_> {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut stack = game::stack::Stack::new(52, self.image_database.get_image("back_blue"));
        let mut cards = Vec::<Card>::new();
        for i in 5..15 {
            cards.push(game::card::Card::new(self.image_database.get_card_image(common::cards::card_from_index(i))));
        }
        let mut hand = game::hand::Hand::new(cards);
        let mut card = game::card::Card::new(self.image_database.get_card_image(common::cards::card_from_index(22)));
        let mut background = self.image_database.get_image("background");
        egui::CentralPanel::default().show(ctx, |ui| {
            let rect = ui.add(background).rect;
            println!("{} {} {} {}", rect.top(), rect.left(), rect.bottom(), rect.right());
            // card.ui(ui, rect.clone());
            stack.ui(ui, rect.clone());
            hand.ui(ui, rect.clone());
            // ui.heading("Hello World!");
        });
    }
}
