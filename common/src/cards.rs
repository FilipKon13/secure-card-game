#[derive(Clone, Copy, Debug)]
pub enum Suit {
    Spades = 0,
    Hearths,
    Diamonds,
    Clubs,
}

#[derive(Clone, Copy, Debug)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

#[derive(Clone, Copy, Debug)]
pub enum PlayingCard {
    Hidden,
    Up(Card),
}
