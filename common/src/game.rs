use std::io;

use crate::cards::Card;
pub trait ShowHand {
    fn show_hand(&self) -> Vec<CardFromDeck>;
}

pub struct GameState {
    pub hand: Vec<Card>,
    pub table_cards: Vec<Card>,
    pub deck_cards: usize,
}

#[derive(Debug, Clone)]
pub struct PlayerBasic {
    pub cards: Vec<Card>,
}

pub trait GamePrinter {
    fn print_game(&mut self, game_state: &GameState) -> io::Result<()>;
}

#[derive(Debug, Clone, Copy)]
pub struct CardFromDeck {
    pub card: Card,
    pub ind: usize,
}

pub trait CardSelector {
    fn select_card(&mut self, hand: &[CardFromDeck]) -> CardFromDeck;
}
