use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use common::game::{GamePrinter, GameState};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use crate::game_state::{Data, GameStateLobby, GameStateTable, SceneUpdate};

pub fn lobby_window() -> (u32, u32) {
    let application = Application::builder()
        .application_id("com.example.SecureCardGame")
        .build();

    let data = Rc::new(RefCell::new(Data {
        num_players: 0,
        player_id: 0,
    }));
    let data_clone = Rc::clone(&data);

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(1200)
            .default_height(800)
            .resizable(false)
            .build();

        window.show_all();

        let data_clone = Rc::clone(&data);

        // Main game loop integrated with GTK's timeout_add
        let game_state = Rc::new(RefCell::new(GameStateLobby::new(
            window,
            data_clone,
        )));
        start_game_loop(game_state.clone());
    });

    application.run();

    let num_players = data_clone.borrow_mut().num_players;
    let player_id = data_clone.borrow_mut().player_id;
    println!("data {} {}", num_players, player_id);
    (num_players, player_id)
}

pub fn table_window(gui_printer: GuiPrinter) {
    let application = Application::builder()
        .application_id("com.example.SecureCardGame")
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
        let game_state = Rc::new(RefCell::new(GameStateTable::new(
            window,
        )));
        start_game_loop(game_state.clone());
    });

    application.run();
}

fn start_game_loop<T: SceneUpdate + 'static> (game_state: Rc<RefCell<T>>) {
    // Using glib's timeout_add to schedule updates on the main GTK thread
    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        game_state.borrow_mut().update();
        glib::Continue(true)
    });
}

pub struct GuiPrinter {

}

impl GamePrinter for GuiPrinter {
    fn print_game(&mut self, game_state: &GameState) {
        let GameState {
            hand,
            table_cards,
            deck_cards,
        } = game_state;
        println!("Game State:");
        println!("Deck: {}", deck_cards);
        println!("Table: {}", format_cards(table_cards));
        println!("Hand: {}", format_cards(hand));
    }
}


fn format_cards<T>(cards: &[T]) -> String
where
    T: Display,
{
    cards
        .iter()
        .map(|f| format!("{}", f))
        .collect::<Vec<String>>()
        .join(", ")
}
