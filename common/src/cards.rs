use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Suit {
    Spades = 0,
    Hearths,
    Diamonds,
    Clubs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

pub fn card_from_index(ind: usize) -> Card {
    assert!(ind < 52);
    Card {
        suit: unsafe { ::std::mem::transmute((ind / 13) as u8) },
        rank: unsafe { ::std::mem::transmute((ind % 13 + 2) as u8) },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn casting_cards() {
        assert_eq!(
            card_from_index(0),
            Card {
                rank: Rank::Two,
                suit: Suit::Spades
            }
        );
        assert_eq!(
            card_from_index(34),
            Card {
                rank: Rank::Ten,
                suit: Suit::Diamonds
            }
        );
    }

    #[test]
    #[should_panic]
    fn casting_fail() {
        card_from_index(52);
    }

    #[test]
    #[should_panic]
    fn casting_fail2() {
        card_from_index(113);
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
