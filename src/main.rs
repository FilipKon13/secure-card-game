use std::{cell::RefCell, rc::Rc};

use clap::{ArgGroup, Parser};
use cli::{CliPrinter, CliSelector};
use crypto::encryption::{basic_deck, short_deck};
use game::window::{lobby_window, table_window, GuiPrinter, GuiPrinterWrap};
use network::con_startup::ConStartup;
use player::{DeckPreparation, OtherPlayer};
use simple_game::SimpleGame;

pub mod moves;
pub mod player;
pub mod simple_game;

#[derive(Parser, Debug)]
#[clap(group(
    ArgGroup::new("connect")
        .required(true)
        .args(&["client", "server"]),
))]
struct Cli {
    /// IP address
    #[clap(default_value = "localhost:1234")]
    address: String,

    /// Connects to game
    #[clap(long)]
    client: bool,

    /// Hosts the game
    #[clap(long)]
    server: bool,
}

fn main() {
    // let Cli {
    //     address,
    //     client,
    //     server,
    // } = Cli::parse();
    // assert_ne!(client, server);
    // let num_players = 2u32;
    // let player_id = if server { 0u32 } else { 1u32 };

    let (num_players, player_id) = lobby_window();
    let address = String::from("127.0.0.1:6700");

    let startup = ConStartup::new(num_players, player_id);
    let server: bool = player_id == 0;

    let printer = Rc::new(RefCell::new(GuiPrinter {}));
    table_window(printer.clone());

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

    let (score, scores) = game.play();

    println!("Your score: {}", score);
    println!("Others: {:?}", scores);

    println!("DONE");
}
