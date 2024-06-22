use clap::{ArgGroup, Parser};
use network::con_startup::ConStartup;
use player::{DeckPreparation, OtherPlayer};

pub mod player;

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
    let Cli {
        address,
        client,
        server,
    } = Cli::parse();
    assert_ne!(client, server);
    let startup = if server {
        ConStartup::new(2, 0)
    } else {
        ConStartup::new(2, 1)
    };

    let other = OtherPlayer::new(startup.initialize(&address));
    let name = if server {
        "server".to_string()
    } else {
        "client".to_string()
    };
    println!("Preparation start");
    DeckPreparation::prepare(name, vec![other], server);
    println!("Preparation completed");

    println!("DONE");
}
