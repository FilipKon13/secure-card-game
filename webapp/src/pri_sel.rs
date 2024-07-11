use std::{thread::sleep, time::Duration};

use actix::Addr;
use actix_web::dev::ServerHandle;
use common::{
    cards::{Card, Suit},
    game::{CardFromDeck, CardSelector, GamePrinter, GameState},
};
use serde::Serialize;

use crate::{
    server::{MyWebSocket, TextMessage},
    MutCondVarPair,
};

#[derive(Clone)]
pub struct WebInterface {
    pub(crate) mcv: MutCondVarPair,
    pub(crate) ws: Addr<MyWebSocket>,
    pub(crate) handle: ServerHandle,
}

impl WebInterface {
    pub fn end_game(self, _my_score: usize, _other_score: usize) {
        self.stop_server();
    }
    pub fn stop_server(self) {
        actix_web::rt::System::new().block_on(self.handle.stop(false));
    }
}

impl GamePrinter for WebInterface {
    fn print_game(&mut self, game_state: &GameState) {
        let lock = self.mcv.mutex().lock().unwrap();
        println!("Hand: {:?}", game_state.hand);
        println!("Table: {:?}", game_state.table_cards);
        self.ws.do_send(TextMessage {
            msg: serde_json::to_string(&SerializableGameState::new(game_state)).unwrap(),
        });
        if game_state.table_cards.len() == 2 {
            sleep(Duration::new(2, 0));
        }
        drop(lock);
    }
}

impl CardSelector for WebInterface {
    fn select_card(&mut self, hand: &[CardFromDeck]) -> CardFromDeck {
        let mut lock = self.mcv.mutex().lock().unwrap();
        *lock = None;
        self.ws.do_send(TextMessage {
            msg: "Select card".to_string(),
        });
        let lock = self
            .mcv
            .cond_var()
            .wait_while(lock, |opt| opt.is_none())
            .unwrap();
        let ind = lock.unwrap();
        println!("Selected: {ind}");
        *hand.get(ind).unwrap()
    }
}

#[derive(Serialize)]
struct SerializableGameState {
    hand: Vec<String>,
    table: Vec<String>,
}

fn card_to_string(cards: &[Card]) -> Vec<String> {
    cards
        .iter()
        .map(|card| {
            format!(
                "{}_{}",
                card.rank,
                match card.suit {
                    Suit::Clubs => "clubs",
                    Suit::Diamonds => "diamonds",
                    Suit::Hearts => "hearts",
                    Suit::Spades => "spades",
                }
            )
        })
        .collect()
}

impl SerializableGameState {
    fn new(game_state: &GameState) -> Self {
        SerializableGameState {
            hand: card_to_string(&game_state.hand),
            table: card_to_string(&game_state.table_cards),
        }
    }
}
