use std::{
    fmt::Display,
    io::{self, Write},
};

use common::{
    cards::{Card, Suit},
    game::{CardFromDeck, CardSelector, GamePrinter, GameState},
};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};

const WIDHT: u16 = 100;
const HEIGHT: u16 = 30;

pub struct CliPrinter {}

impl GamePrinter for CliPrinter {
    fn print_game(&mut self, game_state: &GameState) {
        (|| -> std::io::Result<()> {
            let GameState {
                hand,
                table_cards,
                deck_cards,
            } = game_state;

            let mut stdout = io::stdout();
            stdout.execute(terminal::Clear(terminal::ClearType::All))?;

            for y in 0..HEIGHT {
                for x in 0..WIDHT {
                    if (y == 0 || y == HEIGHT - 1) || (x == 0 || x == WIDHT - 1) {
                        stdout
                            .queue(cursor::MoveTo(x, y))?
                            .queue(style::PrintStyledContent("█".white()))?;
                    }
                }
            }

            stdout
                .queue(cursor::MoveTo(19, 3))?
                .queue(style::PrintStyledContent("Secure Card Game".white()))?;

            for x in 0..WIDHT {
                stdout
                    .queue(cursor::MoveTo(x, 6))?
                    .queue(style::PrintStyledContent("█".white()))?;
            }

            stdout
                .queue(cursor::MoveTo(10, 11))?
                .queue(style::PrintStyledContent(
                    format!("Cards left in the deck: {}", deck_cards).white(),
                ))?;

            stdout
                .queue(cursor::MoveTo(10, 14))?
                .queue(style::PrintStyledContent("Cards on the table: ".white()))?;
            print_cards(table_cards)?;

            stdout
                .queue(cursor::MoveTo(10, 17))?
                .queue(style::PrintStyledContent("Cards in the hand: ".white()))?;
            print_cards(hand)?;

            stdout.queue(cursor::MoveTo(0, HEIGHT))?;
            stdout.flush()?;
            Ok(())
        })()
        .unwrap();
    }
}

fn print_cards(cards: &Vec<Card>) -> io::Result<()> {
    let mut stdout = io::stdout();
    for card in cards {
        let card_str = card.to_string() + " ";
        if card.suit == Suit::Hearts || card.suit == Suit::Diamonds {
            stdout.queue(style::PrintStyledContent(card_str.red()))?;
        } else {
            stdout.queue(style::PrintStyledContent(card_str.white()))?;
        }
    }
    Ok(())
}

#[allow(dead_code)]
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
        let mut stdout = io::stdout();
        stdout
            .queue(cursor::MoveTo(20, 21))
            .unwrap()
            .queue(style::PrintStyledContent(
                "Choose a card".cyan().slow_blink(),
            ))
            .unwrap();
        stdout.queue(cursor::MoveTo(0, HEIGHT)).unwrap();
        stdout.flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let index = buffer.trim().parse::<usize>().expect("Wrong index");
        // dbg!(&index);
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
                    suit: Suit::Hearts,
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
