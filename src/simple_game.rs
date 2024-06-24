use std::ops::AddAssign;

use crate::moves::Moves;
use common::{
    cards::Card,
    game::{CardFromDeck, CardSelector, GamePrinter, GameState, ShowHandDeck},
};

#[derive(Debug)]
enum Turn {
    MeDraw(),
    OtherDraw(),
    Me(),
    Other(),
    OtherResponding {
        my_card: Card,
    },
    Response {
        other_card: Card,
    },
    Battle {
        my_card: Card,
        other_card: Card,
        me_start: bool,
    },
    Done(),
}

fn who_won(first_card: Card, second_card: Card) -> bool {
    if first_card.suit != second_card.suit {
        return true;
    }
    first_card.rank > second_card.rank
}

// works only for 2 players atm
pub struct SimpleGame<Printer, Selector, PlayerType>
where
    Printer: GamePrinter,
    Selector: CardSelector,
    PlayerType: Moves + ShowHandDeck,
{
    player: PlayerType,
    player_id: usize,
    turn: Turn,
    score: usize,
    scores: Vec<usize>,
    printer: Printer,
    selector: Selector,
}

const HAND_SIZE: usize = 5;

impl<Printer, Selector, PlayerType> SimpleGame<Printer, Selector, PlayerType>
where
    Printer: GamePrinter,
    Selector: CardSelector,
    PlayerType: Moves + ShowHandDeck,
{
    pub fn new(
        player_id: usize,
        num_players: usize,
        player: PlayerType,
        printer: Printer,
        selector: Selector,
    ) -> Self {
        let me_start = player_id == 0;
        SimpleGame::<Printer, Selector, PlayerType> {
            player,
            player_id,
            turn: if me_start {
                Turn::MeDraw()
            } else {
                Turn::OtherDraw()
            },
            score: 0,
            scores: vec![0; num_players - 1],
            printer,
            selector,
        }
    }

    fn is_done(&self) -> bool {
        matches!(self.turn, Turn::Done())
    }

    fn get_initial_cards(&mut self) {
        for _ in 0..HAND_SIZE {
            self.player.draw_from_deck();
        }
    }

    fn send_initial_cards(&mut self) {
        for _ in 0..HAND_SIZE {
            self.player.let_draw_from_deck(0);
        }
    }

    fn is_deck_empty(&self) -> bool {
        self.player.deck_size() == 0
    }

    fn is_hand_empty(&self) -> bool {
        self.player.show_hand().is_empty()
    }

    fn battle_cards(&mut self, card: Card, other_card: Card, me_first: bool) -> Turn {
        if card == other_card {
            panic!("Duplicate cards in deck");
        }
        let is_deck_empty = self.is_deck_empty();
        let result = if me_first {
            who_won(card, other_card)
        } else {
            !who_won(other_card, card)
        };
        if !is_deck_empty {
            if result {
                self.player.draw_from_deck();
                self.player.let_draw_from_deck(0);
            } else {
                self.player.let_draw_from_deck(0);
                self.player.draw_from_deck();
            }
        }
        if result {
            self.score.add_assign(1);
        } else {
            self.scores.get_mut(0).unwrap().add_assign(1);
        }
        let mut turn = if result { Turn::Me() } else { Turn::Other() };
        if self.is_hand_empty() {
            turn = Turn::Done();
        }
        turn
    }

    fn play_card(&mut self) -> Card {
        let CardFromDeck { card, ind } = self.selector.select_card(&self.player.show_hand());
        self.player.play_card(ind);
        card
    }

    fn let_play_card(&mut self) -> Card {
        self.player.let_play_card(0)
    }

    fn get_table_cards(&self) -> Vec<Card> {
        match self.turn {
            Turn::OtherResponding { my_card } => vec![my_card],
            Turn::Response { other_card } => vec![other_card],
            Turn::Battle {
                my_card,
                other_card,
                me_start,
            } => {
                if me_start {
                    vec![my_card, other_card]
                } else {
                    vec![other_card, my_card]
                }
            }
            _ => vec![],
        }
    }

    fn make_turn(&mut self) {
        let _ = self.printer.print_game(&GameState {
            hand: self.player.show_hand().iter().map(|f| f.card).collect(),
            table_cards: self.get_table_cards(),
            deck_cards: self.player.deck_size(),
        });
        self.turn = match self.turn {
            Turn::MeDraw() => {
                self.get_initial_cards();
                if self.player_id == 0 {
                    Turn::OtherDraw()
                } else {
                    Turn::Other()
                }
            }
            Turn::OtherDraw() => {
                self.send_initial_cards();
                if self.player_id == 0 {
                    Turn::Me()
                } else {
                    Turn::MeDraw()
                }
            }
            Turn::Me() => Turn::OtherResponding {
                my_card: self.play_card(),
            },
            Turn::OtherResponding { my_card } => Turn::Battle {
                my_card,
                other_card: self.let_play_card(),
                me_start: true,
            },
            Turn::Other() => Turn::Response {
                other_card: self.let_play_card(),
            },
            Turn::Response { other_card } => Turn::Battle {
                my_card: self.play_card(),
                other_card,
                me_start: false,
            },
            Turn::Battle {
                my_card,
                other_card,
                me_start,
            } => self.battle_cards(my_card, other_card, me_start),
            Turn::Done() => unreachable!("Game is done"),
        }
    }

    pub fn play_one_step(&mut self) {
        if !self.is_done() {
            self.make_turn();
        }
    }

    pub fn play(mut self) -> (usize, Vec<usize>) {
        while !self.is_done() {
            self.make_turn();
        }
        (self.score, self.scores)
    }
}

#[cfg(test)]
mod test {
    use std::ops::{AddAssign, SubAssign};

    use common::{
        cards::{Card, Rank, Suit},
        game::{CardFromDeck, CardSelector, GamePrinter, ShowHandDeck},
    };

    use crate::moves::Moves;

    use super::SimpleGame;

    struct MockPrinter {}

    impl GamePrinter for MockPrinter {
        fn print_game(&mut self, _game_state: &common::game::GameState) -> std::io::Result<()> {
            Ok(())
        }
    }

    struct MockSelector {}

    impl CardSelector for MockSelector {
        fn select_card(
            &mut self,
            hand: &[common::game::CardFromDeck],
        ) -> common::game::CardFromDeck {
            *hand.first().unwrap()
        }
    }

    struct MockPlayer {
        deck_size: usize,
        hand_size: usize,
    }

    impl Moves for MockPlayer {
        fn draw_from_deck(&mut self) -> Card {
            self.deck_size.sub_assign(1);
            self.hand_size.add_assign(1);
            Card {
                rank: Rank::Ace,
                suit: Suit::Clubs,
            }
        }
        fn let_draw_from_deck(&mut self, _other: usize) {
            self.deck_size.sub_assign(1);
        }
        fn let_play_card(&mut self, _other: usize) -> Card {
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            }
        }
        fn play_card(&mut self, _ind: usize) {
            self.hand_size.sub_assign(1);
        }
    }

    impl ShowHandDeck for MockPlayer {
        fn deck_size(&self) -> usize {
            self.deck_size
        }
        fn show_hand(&self) -> Vec<CardFromDeck> {
            vec![
                CardFromDeck {
                    card: Card {
                        rank: Rank::Ace,
                        suit: Suit::Clubs,
                    },
                    ind: 0
                };
                self.hand_size
            ]
        }
    }

    #[test]
    fn interaction() {
        let game = SimpleGame::new(
            0,
            2,
            MockPlayer {
                deck_size: 52,
                hand_size: 0,
            },
            MockPrinter {},
            MockSelector {},
        );
        let (score, scores) = game.play();
        assert_eq!(score, 26);
        assert_eq!(scores, vec![0]);
    }
}
