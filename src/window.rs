use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use cli::CliSelector;
use common::game::{GamePrinter, GameState};
use crypto::encryption::short_deck;
use game::gui_printer::{GuiPrinter, GuiPrinterWrap};
// use game::window::{GuiPrinter};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use game::game_state::{Data, GameStateLobby, GameStateTable, SceneUpdate};
use network::con_startup::ConStartup;

use crate::player::{DeckPreparation, OtherPlayer};
use crate::simple_game::SimpleGame;

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
        let game_state = Rc::new(RefCell::new(GameStateLobby::new(window, data_clone)));
        start_game_loop(game_state.clone());
    });

    application.run();

    let num_players = data_clone.borrow_mut().num_players;
    let player_id = data_clone.borrow_mut().player_id;
    println!("data {} {}", num_players, player_id);
    (num_players, player_id)
}

pub fn table_window(num_players: u32, player_id: u32) {
    let application = Application::builder()
        .application_id("com.example.SecureCardGame")
        .build();

        let printer = Rc::new(RefCell::new(GuiPrinter {table: None}));
        let printer_clone = Rc::clone(&printer);

    application.connect_activate(move |app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(1200)
            .default_height(800)
            .resizable(false)
            .build();

        window.show_all();


        // Main game loop integrated with GTK's timeout_add
        let game_state = Rc::new(RefCell::new(GameStateTable::new(window, printer_clone.clone())));
        start_game_loop_table(num_players, player_id, game_state.clone(), printer.clone());

    });

    application.run();
}

fn start_game_loop_table(num_players: u32, player_id: u32, game_state: Rc<RefCell<GameStateTable>>, printer: Rc<RefCell<GuiPrinter>>) {

    let address = String::from("127.0.0.1:6700");

    let startup = ConStartup::new(num_players, player_id);
    let server: bool = player_id == 0;



    let other = OtherPlayer::new(startup.initialize(&address));
    let name = if server {
        "server".to_string()
    } else {
        "client".to_string()
    };
    println!("Preparation start");
    let player = DeckPreparation::prepare(name, vec![other], server, short_deck().to_vec());
    println!("Preparation completed");

    println!("Player deck size: {}", player.deck.len());

    let game = SimpleGame::new(
        player_id as usize,
        num_players as usize,
        player,
        GuiPrinterWrap {
            gui_printer: printer.clone(),
        },
        CliSelector {},
    );

    println!("Starting game");

    // let (score, scores) = game.play();

    

    // Using glib's timeout_add to schedule updates on the main GTK thread
    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        // game.play_one_step();
        game_state.borrow_mut().update();
        glib::Continue(true)
    });

    // println!("Your score: {}", score);
    // println!("Others: {:?}", scores);

    // println!("DONE");
}

fn start_game_loop<T: SceneUpdate + 'static>(game_state: Rc<RefCell<T>>) {

    // Using glib's timeout_add to schedule updates on the main GTK thread
    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        game_state.borrow_mut().update();
        glib::Continue(true)
    });
}
