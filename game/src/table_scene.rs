use gtk::prelude::{ContainerExt, OverlayExt, WidgetExt};
use gtk::{ApplicationWindow, DrawingArea, EventBox, Image, Inhibit, Overlay};

use common::cards::Card;

use crate::{hand::Hand, image_database::ImageDatabase, stack::Stack};

pub struct TableScene {
    pub window: ApplicationWindow,
    pub redraw: bool,
    pub image_database: ImageDatabase,
    pub hand_cards: Vec<Card>,
}

impl TableScene {
    pub fn new(window: ApplicationWindow) -> Self {
        Self {
            window,
            redraw: true,
            image_database: ImageDatabase::default(),
            hand_cards: Vec::<Card>::new(),
        }
    }
}

impl super::Scene for TableScene {
    fn update(&mut self) {
        if self.redraw {
            for child in self.window.children() {
                self.window.remove(&child);
            }

            for i in 0..15 {
                let _ = &self
                    .hand_cards
                    .push(common::cards::Card::try_from(i).unwrap());
            }
            let hand = Hand::new(&self.hand_cards, &self.image_database);
            let stack = Stack::new(52, &self.image_database);

            let overlay = Overlay::new();
            {
                //background
                let background_image_pixbuf = self.image_database.get_image("background");
                let background = Image::from_pixbuf(Some(&background_image_pixbuf));
                overlay.add(&background);
            }
            let drawing_area = DrawingArea::new();
            overlay.add_overlay(&drawing_area);
            {
                //cards
                hand.draw(drawing_area.clone());
                stack.draw(drawing_area.clone());
            }
            {
                //event box
                let event_box = EventBox::new();
                event_box.connect_button_press_event(move |_event_box, event| {
                    let (x, y) = event.position();
                    let mut clicked = hand.clicked(x, y);
                    if clicked == *"" {
                        clicked = stack.clicked(x, y);
                    }
                    println!("{}", clicked);
                    Inhibit(false)
                });
                overlay.add_overlay(&event_box);
            }
            self.window.add(&overlay);
            self.window.show_all();

            self.redraw = false;
        }
    }
}
