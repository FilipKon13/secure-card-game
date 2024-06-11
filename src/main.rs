use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

use crypto::encryption::basic_deck;
use crypto::encryption::decrypt;
use crypto::encryption::Translator;
use crypto::shuffle::PartyBasic;
use crypto::types::EncryptedValue;
use crypto::types::KeyType;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

struct Connection {
    stream: TcpStream,
}

const BUF_SIZE: usize = 1_000_000;

impl Connection {
    pub fn send<T: Serialize>(&mut self, msg: &T) {
        self.stream
            .write_all(&serde_json::to_vec(&msg).unwrap())
            .unwrap();
    }
    pub fn receive<T: DeserializeOwned>(&mut self) -> T {
        let mut buf = [0; BUF_SIZE];
        let s = self.stream.read(&mut buf).unwrap();
        dbg!(s);
        serde_json::from_slice::<T>(&buf[0..s]).unwrap()
    }
}

struct OtherPlayer {
    pub connection: Connection,
}

struct Player {
    deck: Vec<EncryptedValue>,
    keys: Vec<KeyType>,
    players: Vec<OtherPlayer>,
    owners: Vec<Option<((), u32)>>,
    name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            deck: vec![],
            keys: vec![],
            players: vec![],
            owners: vec![],
            name,
        }
    }
    pub fn add_player(&mut self, other: OtherPlayer) {
        self.players.push(other);
    }
    pub fn prepare_deck_start(&mut self) {
        let mut deck = basic_deck().into_iter().collect();
        let mut party = PartyBasic::new();
        dbg!((&party.state, &self.name));
        party.make_turn(&mut deck);
        dbg!((&party.state, &self.name));
        self.send_deck(&deck);
        if party.is_done() {
            return;
        }
        while !party.is_done() {
            let mut deck = self.get_deck();
            dbg!((&party.state, &self.name));
            party.make_turn(&mut deck);
            dbg!((&party.state, &self.name));
            self.send_deck(&deck);
        }
        (self.deck, self.keys) = party.retrieve_deck();
        self.init_owners();
    }
    pub fn prepare_deck_join(&mut self) {
        let mut party = PartyBasic::new();
        while !party.is_done() {
            let mut deck = self.get_deck();
            dbg!((&party.state, &self.name));
            party.make_turn(&mut deck);
            dbg!((&party.state, &self.name));
            self.send_deck(&deck);
        }
        (self.deck, self.keys) = party.retrieve_deck();
        self.init_owners();
    }
    fn get_deck(&mut self) -> Vec<EncryptedValue> {
        self.players
            .first_mut()
            .unwrap()
            .connection
            .receive::<Vec<EncryptedValue>>()
    }
    fn send_deck(&mut self, deck: &Vec<EncryptedValue>) {
        self.players.last_mut().unwrap().connection.send(&deck);
    }
    fn init_owners(&mut self) {
        self.owners = vec![None; self.deck.len()];
    }
}

const ADDRESS: &str = "localhost:1234";

fn get_sockets() -> (TcpStream, TcpStream) {
    let server = TcpListener::bind(ADDRESS).unwrap();
    let mut conn1 = TcpStream::connect(ADDRESS).unwrap();
    let mut conn2 = server.accept().unwrap().0;
    conn1.write(b"Hello").unwrap();
    let mut buf = [0; 512];
    let s = conn2.read(&mut buf).unwrap();
    dbg!(String::from_utf8_lossy(&buf[0..s]).to_string());
    (conn1, conn2)
}

fn main() {
    println!("Starting game for two players");
    let (conn1, conn2) = get_sockets();
    let p1 = OtherPlayer {
        connection: Connection { stream: conn1 },
    };
    let p2 = OtherPlayer {
        connection: Connection { stream: conn2 },
    };
    let mut player_1 = Player::new("P1".to_string());
    let mut player_2 = Player::new("P2".to_string());
    player_1.add_player(p2);
    player_2.add_player(p1);
    let t1 = thread::spawn(move || {
        player_1.prepare_deck_start();
        println!("DONE {}", player_1.name);
        (player_1.deck, player_1.keys)
    });
    let t2 = thread::spawn(move || {
        player_2.prepare_deck_join();
        println!("DONE {}", player_2.name);
        (player_2.deck, player_2.keys)
    });
    let (d1, k1) = t1.join().unwrap();
    let (d2, k2) = t2.join().unwrap();
    println!("Done");
    assert_eq!(d1, d2);
    println!("Same deck");
    use std::iter::zip;
    let translator = Translator::new(&basic_deck());
    let deck = zip(d1, k1)
        .map(|(c, k)| decrypt(c, k))
        .zip(k2)
        .map(|(c, k)| decrypt(c, k))
        .map(|c| translator.translate(c).unwrap())
        .collect::<Vec<_>>();
    println!("Deck: {:?}", deck);
}
