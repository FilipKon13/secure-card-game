use std::{fmt::Display, io};

use common::game::{CardFromDeck, CardSelector, GamePrinter, GameState};

pub struct CliPrinter {}

impl GamePrinter for CliPrinter {
    fn print_game(&mut self, game_state: &GameState) {
        let GameState {
            hand,
            table_cards,
            deck_cards,
        } = game_state;
        println!("Game State:");
        println!("Deck: {}", deck_cards);
        println!("Table: {}", format_cards(table_cards));
        println!("Hand: {}", format_cards(hand));
    }
}

fn format_cards<T>(cards: &[T]) -> String
where
    T: Display,
{
    cards
        .iter()
        .map(|f| format!("{}", f))
        .collect::<Vec<String>>()
        .join(", ")
}

pub struct CliSelector {}

impl CardSelector for CliSelector {
    fn select_card(&mut self, hand: &[CardFromDeck]) -> common::game::CardFromDeck {
        let mut buffer = String::new();
        println!("Choose a card");
        io::stdin().read_line(&mut buffer).unwrap();
        let index = buffer.trim().parse::<usize>().expect("Wrong index");
        dbg!(&index);
        assert!(index < hand.len(), "Wrong index");
        *hand.get(index).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use common::{
        cards::*,
        game::{GamePrinter, GameState},
    };

    use crate::CliPrinter;

    #[test]
    fn it_works() {
        let game = GameState {
            hand: vec![
                Card {
                    suit: Suit::Clubs,
                    rank: Rank::Ace,
                },
                Card {
                    suit: Suit::Diamonds,
                    rank: Rank::Ten,
                },
                Card {
                    suit: Suit::Hearths,
                    rank: Rank::King,
                },
            ],
            table_cards: vec![Card {
                suit: Suit::Clubs,
                rank: Rank::Six,
            }],
            deck_cards: 48,
        };
        let mut printer = CliPrinter {};
        printer.print_game(&game);
    }
}
