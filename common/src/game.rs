use crate::cards::Card;
pub trait ShowHandDeck {
    fn show_hand(&self) -> Vec<CardFromDeck>;
    fn deck_size(&self) -> usize;
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
    fn print_game(&mut self, game_state: &GameState);
}

#[derive(Debug, Clone, Copy)]
pub struct CardFromDeck {
    pub card: Card,
    pub ind: usize,
}

pub trait CardSelector {
    fn select_card(&mut self, hand: &[CardFromDeck]) -> CardFromDeck;
}
