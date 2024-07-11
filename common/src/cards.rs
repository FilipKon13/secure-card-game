use std::{error::Error, fmt::Display};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Suit {
    Spades = 0,
    Hearts,
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

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            x if (x as isize) <= 10 => (x as isize).fmt(f),
            Rank::Jack => "J".fmt(f),
            Rank::Queen => "Q".fmt(f),
            Rank::King => "K".fmt(f),
            Rank::Ace => "A".fmt(f),
            _ => unreachable!(),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Suit::Spades => "♠",
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
        }
        .fmt(f)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Card { rank, suit } = self;
        write!(f, "{}{}", rank, suit)
    }
}

impl Display for PlayingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayingCard::Up(card) => card.fmt(f),
            PlayingCard::Hidden => "#".fmt(f),
        }
    }
}

#[derive(Debug)]
pub struct ParseCardError {
    index: usize,
}

impl Display for ParseCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Provided index {} does not represent a card", self.index)
    }
}

impl Error for ParseCardError {}

impl TryFrom<usize> for Card {
    type Error = ParseCardError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value >= 52 {
            return Err(ParseCardError { index: value });
        }
        Ok(Card {
            suit: unsafe { ::std::mem::transmute::<u8, Suit>((value / 13) as u8) },
            rank: unsafe { ::std::mem::transmute::<u8, Rank>((value % 13 + 2) as u8) },
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
