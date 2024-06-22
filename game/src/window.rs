use std::cell::RefCell;
use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use crate::game_state::GameState;

pub static mut NUM_PLAYERS: u32 = 0;
pub static mut PLAYER_ID: u32 = 0;

pub fn lobby_window() -> (u32, u32) {
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
        let game_state = Rc::new(RefCell::new(GameState::new(window, String::from("lobby"))));
        start_game_loop(game_state.clone());
    });

    application.run();

    let mut num_players = 0;
    let mut player_id = 0;
    unsafe {
        num_players = NUM_PLAYERS;
        player_id = PLAYER_ID;
    }
    println!("data {} {}", num_players, player_id);
    (num_players, player_id)
}

pub fn table_window() {
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
        let game_state = Rc::new(RefCell::new(GameState::new(window, String::from("table"))));
        start_game_loop(game_state.clone());
    });

    application.run();
}

fn start_game_loop(game_state: Rc<RefCell<GameState>>) {
    // Using glib's timeout_add to schedule updates on the main GTK thread
    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        game_state.borrow_mut().update();
        glib::Continue(true)
    });
}
