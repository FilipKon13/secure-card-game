use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, DrawingArea, EventBox, Image, Overlay};
use std::cell::RefCell;
use std::rc::Rc;

use common::{cards::Card};
use game::{hand::Hand, image_database::image_database, stack::Stack};

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstGtkApp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(1200)
            .default_height(800)
            .resizable(false)
            .build();

        window.show_all();

        // Main game loop integrated with GTK's timeout_add
        let game_state = Rc::new(RefCell::new(GameState::new(window)));
        start_game_loop(game_state.clone());
    });

    application.run();
}

struct GameState {
    redraw: bool,
    window: ApplicationWindow,
    image_database: image_database,
}

impl GameState {
    fn new(window: ApplicationWindow) -> Self {
        Self {
            redraw: true,
            window: window,
            image_database: image_database::default(),
        }
    }

    fn update(&mut self) {
        
        
        
        if self.redraw {
            for child in self.window.children() {
                self.window.remove(&child);
            }
            
            // let card = common::cards::card_from_index(15);
            // let card2 = common::cards::card_from_index(40);
            // let clickableCard = Clickable::new(card.to_string(), 100.0, 0.0, 45.0, self.image_database.get_card_image(card));
            // let clickableCard2 = Clickable::new(card2.to_string(), 100.0, 300.0, 45.0, self.image_database.get_card_image(card2));

            let mut hand_cards = Vec::<Card>::new();
            for i in 0..15 {
                hand_cards.push(common::cards::card_from_index(i));
            }
            let hand = Hand::new(hand_cards, &self.image_database);
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
                // clickableCard.draw(drawing_area.clone());
                // clickableCard2.draw(drawing_area.clone());
                hand.draw(drawing_area.clone());
                stack.draw(drawing_area.clone());
            }
            {
                //event box
                let event_box = EventBox::new();
                event_box.connect_button_press_event(move |event_box, event| {
                    let (x,y) = event.position();
                    let mut clicked = hand.clicked(x,y);
                    if clicked == "".to_string() {
                        clicked = stack.clicked(x,y);
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
        // Clear the window
        // self.clear_window();
        
        
        // Add new widgets to the window
        // self.add_widgets();
        
        
    }
}

fn start_game_loop(game_state: Rc<RefCell<GameState>>) {
    // Using glib's timeout_add to schedule updates on the main GTK thread
    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        game_state.borrow_mut().update();
        glib::Continue(true)
    });
}