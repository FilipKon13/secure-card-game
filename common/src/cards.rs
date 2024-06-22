use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Suit {
    Spades = 0,
    Hearths,
    Diamonds,
    Clubs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCardError {}

impl Error for ParseCardError {
    fn description(&self) -> &str {
        "Provided index does not represent a card"
    }
}

impl std::fmt::Display for ParseCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[allow(deprecated)]
        self.description().fmt(f)
    }
}

impl TryFrom<usize> for Card {
    type Error = ParseCardError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= 52 {
            return Err(ParseCardError {});
        }
        Ok(Card {
            suit: unsafe { ::std::mem::transmute((value / 13) as u8) },
            rank: unsafe { ::std::mem::transmute((value % 13 + 2) as u8) },
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn casting_cards() {
        assert_eq!(
            Card::try_from(0).unwrap(),
            Card {
                rank: Rank::Two,
                suit: Suit::Spades
            }
        );
        assert_eq!(
            Card::try_from(34).unwrap(),
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamonds
            }
        );
    }

    #[test]
    #[should_panic]
    fn casting_fail() {
        Card::try_from(52).unwrap();
    }

    #[test]
    #[should_panic]
    fn casting_fail2() {
        Card::try_from(113).unwrap();
    }

    #[test]
    fn card_display() {
        assert_eq!(
            format!(
                "{}",
                Card {
                    rank: Rank::Ace,
                    suit: Suit::Diamonds
                }
            ),
            "A♦"
        );
    }
}
