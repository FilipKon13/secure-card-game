use crypto::encryption::basic_deck;
use crypto::encryption::Translator;
use crypto::shuffle::PartyBasic;
use crypto::types::EncryptedValue;
use crypto::types::KeyType;

use network::connection::Connection;
use network::connection::TcpConnection;

pub struct OtherPlayer {
    connection: TcpConnection,
}

impl OtherPlayer {
    pub fn new(conn: TcpConnection) -> Self {
        OtherPlayer { connection: conn }
    }
}

impl Connection for OtherPlayer {
    fn send<T: serde::Serialize>(&mut self, message: &T) {
        self.connection.send(message)
    }
    fn receive<T: serde::de::DeserializeOwned>(&mut self) -> T {
        self.connection.receive()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Owner {
    Me,
    Other(usize),
}

pub struct Player {
    pub deck: Vec<EncryptedValue>,
    pub keys: Vec<KeyType>,
    pub players: Vec<OtherPlayer>,
    pub owners: Vec<Option<Owner>>,
    pub name: String,
    pub translator: Translator,
}

pub struct DeckPreparation {
    players: Vec<OtherPlayer>,
    name: String,
}

impl DeckPreparation {
    pub fn prepare(name: String, others: Vec<OtherPlayer>, start: bool) -> Player {
        let mut preparation = DeckPreparation {
            players: others,
            name,
        };
        let (deck, keys) = if start {
            preparation.prepare_deck_start(basic_deck().to_vec())
        } else {
            preparation.prepare_deck_join()
        };
        let len = preparation.players.len();
        Player {
            deck,
            keys,
            players: preparation.players,
            owners: vec![None; len],
            name: preparation.name,
            translator: Translator::new(&basic_deck()),
        }
    }
    fn prepare_deck_start(
        &mut self,
        mut deck: Vec<EncryptedValue>,
    ) -> (Vec<EncryptedValue>, Vec<KeyType>) {
        let mut party = PartyBasic::new();
        let mut start = true;
        while !party.is_done() {
            if !start {
                deck = self.get_deck();
            }
            start = false;
            dbg!((&party.state, &self.name));
            party.make_turn(&mut deck);
            dbg!((&party.state, &self.name));
            self.send_deck(&deck);
        }
        self.get_deck(); // wait for the rest
        party.retrieve_deck()
    }
    fn prepare_deck_join(&mut self) -> (Vec<EncryptedValue>, Vec<KeyType>) {
        let mut party = PartyBasic::new();
        while !party.is_done() {
            let mut deck = self.get_deck();
            dbg!((&party.state, &self.name));
            party.make_turn(&mut deck);
            dbg!((&party.state, &self.name));
            self.send_deck(&deck);
        }
        party.retrieve_deck()
    }
    fn get_deck(&mut self) -> Vec<EncryptedValue> {
        self.players
            .first_mut()
            .unwrap()
            .connection
            .receive::<Vec<_>>()
    }
    fn send_deck(&mut self, deck: &Vec<EncryptedValue>) {
        self.players.last_mut().unwrap().send(&deck);
    }
}

#[cfg(test)]
mod test {
    const ADDRESS: &str = "localhost:1234";

    use super::*;
    use crypto::encryption::{basic_deck, decrypt, Translator};
    use network::con_startup::ConStartup;
    use std::thread;

    #[test]
    fn encrypt_over_network() {
        println!("Starting game for two players");
        let t1 = thread::spawn(|| {
            let startup = ConStartup::new(2, 0);
            let opponent = OtherPlayer::new(startup.initialize(&ADDRESS.to_string()));
            let player_1 = DeckPreparation::prepare("P1".to_string(), vec![opponent], true);
            println!("DONE {}", player_1.name);
            (player_1.deck, player_1.keys)
        });
        let t2 = thread::spawn(|| {
            let startup = ConStartup::new(2, 1);
            let opponent = OtherPlayer::new(startup.initialize(&ADDRESS.to_string()));
            let player_2 = DeckPreparation::prepare("P2".to_string(), vec![opponent], false);
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
}
