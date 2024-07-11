use clap::{ArgGroup, Parser};
// use cli::{CliPrinter, CliSelector};
use crypto::encryption::{basic_deck, short_deck};
use network::con_startup::ConStartup;
use player::{DeckPreparation, DeckPreparationBasic, DeckPreparationVerification, OtherPlayer};
use simple_game::SimpleGame;
use webapp::get_web_interface;

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

    /// Browser port
    #[clap(long)]
    port: u16,

    /// Connects to game
    #[clap(long, conflicts_with = "server")]
    client: bool,

    /// Hosts the game
    #[clap(long)]
    server: bool,

    /// Additional verify
    #[clap(long)]
    verify: bool,

    /// 52 instead of 16
    #[clap(long)]
    big_deck: bool,
}

fn main() {
    let Cli {
        address,
        port,
        client,
        server,
        verify,
        big_deck,
    } = Cli::parse();
    assert_ne!(client, server);
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    ctrlc::set_handler(|| std::process::exit(0)).expect("Setting handler should not fail");

    let web_interface = get_web_interface(port);

    let num_players = 2;
    let player_id = if server { 0 } else { 1 };
    let startup = ConStartup::new(num_players, player_id);

    let other = OtherPlayer::new(startup.initialize(&address));
    let name = if server {
        "server".to_string()
    } else {
        "client".to_string()
    };
    println!("Preparation start");
    let deck = if big_deck {
        basic_deck().to_vec()
    } else {
        short_deck().to_vec()
    };
    let player = if verify {
        DeckPreparationVerification::prepare(name, vec![other], server, deck)
    } else {
        DeckPreparationBasic::prepare(name, vec![other], server, deck)
    };
    println!("Preparation completed");

    println!("Player deck size: {}", player.deck.len());

    let game = SimpleGame::new(
        player_id,
        num_players,
        player,
        web_interface.clone(),
        web_interface.clone(),
    );

    println!("Starting game");

    let (score, scores) = game.play();

    println!();
    println!("Your score: {}", score);
    println!("Opponent's score: {:?}", scores.first().unwrap());

    web_interface.end_game(score, *scores.first().unwrap());
}
