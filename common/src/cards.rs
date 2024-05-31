use std::fmt::Display;

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

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = match self.rank {
            x if (x as isize) <= 10 => (x as isize).to_string(),
            Rank::Jack => "J".to_string(),
            Rank::Queen => "Q".to_string(),
            Rank::King => "K".to_string(),
            Rank::Ace => "A".to_string(),
            _ => unreachable!(),
        };
        let suit = match self.suit {
            Suit::Spades => "♠",
            Suit::Hearths => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
        };
        write!(f, "{}{}", rank, suit)
    }
}

impl Display for PlayingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayingCard::Up(card) => card.fmt(f),
            PlayingCard::Hidden => write!(f, "#"),
        }
    }
}
