// use std::net::TcpStream;

// use crypto::types::EncryptedValue;
// use crypto::types::KeyType;
// use serde_json;

// struct OtherPlayer {
//     pub connection: TcpStream,
//     pub name: String,
// }

// struct Player<'a> {
//     deck: Vec<EncryptedValue>,
//     keys: Vec<KeyType>,
//     players: Vec<OtherPlayer>,
//     owners: Vec<Option<&'a OtherPlayer>>,
// }

// impl<'a> Player<'a> {
//     pub fn new() -> Self {
//         Player {
//             deck: vec![],
//             keys: vec![],
//             players: vec![],
//             owners: vec![],
//         }
//     }
//     pub fn add_player(&mut self, other: OtherPlayer) {
//         self.players.push(other);
//     }
//     fn from_prev(&mut self) -> &mut TcpStream {
//         &mut self.players.last_mut().unwrap().connection
//     }
//     fn to_next(&mut self) -> &mut TcpStream {
//         &mut self.players.first_mut().unwrap().connection
//     }
//     fn pass_deck(&mut self) {
//         let json = serde_json::to_string(&self.deck);
//     }
//     pub fn prepare_deck_and_keys(&mut self, is_start: bool) {}
// }

fn main() {
    println!("Starting game for two players");
}
