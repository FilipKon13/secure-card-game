use std::ops::AddAssign;

use crate::{moves::Moves, player::Player};
use common::{
    cards::Card,
    game::{CardFromDeck, CardSelector, GamePrinter, GameState, ShowHand},
};

#[derive(Debug)]
enum Turn {
    MeDraw(),
    OtherDraw(),
    Me(),
    Other(),
    OtherResponding(),
    Response(),
    Battle(),
    Done(),
}

struct GameAndTable {
    score: usize,
    scores: Vec<usize>,
    my_card: Option<Card>,
    other_cards: Vec<Option<Card>>,
    me_start: bool,
}

impl GameAndTable {
    fn get_table_cards(&self) -> Vec<Card> {
        let mut res = vec![];
        if let Some(card) = self.my_card {
            res.push(card);
        }
        if let Some(card) = self.other_cards.first().unwrap() {
            res.push(*card);
        }
        res
    }
    fn clear_table(&mut self) {
        self.my_card = None;
        self.other_cards.iter_mut().for_each(|c| *c = None);
    }
}

fn who_won(first_card: Card, second_card: Card) -> bool {
    if first_card.suit != second_card.suit {
        return true;
    }
    first_card.rank > second_card.rank
}

// works only for 2 players atm
pub struct SimpleGame<Printer, Selector>
where
    Printer: GamePrinter,
    Selector: CardSelector,
{
    player: Player,
    player_id: usize,
    turn: Turn,
    game_and_table: GameAndTable,
    printer: Printer,
    selector: Selector,
}

const HAND_SIZE: usize = 5;

impl<Printer, Selector> SimpleGame<Printer, Selector>
where
    Printer: GamePrinter,
    Selector: CardSelector,
{
    pub fn new(
        player_id: usize,
        num_players: usize,
        player: Player,
        printer: Printer,
        selector: Selector,
    ) -> Self {
        let me_start = player_id == 0;
        SimpleGame::<Printer, Selector> {
            player,
            player_id,
            turn: if me_start {
                Turn::MeDraw()
            } else {
                Turn::OtherDraw()
            },
            game_and_table: GameAndTable {
                score: 0,
                scores: vec![0; num_players - 1],
                my_card: None,
                other_cards: vec![None; num_players - 1],
                me_start,
            },
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

    fn deck_size(&self) -> usize {
        self.player.owners.iter().filter(|e| e.is_none()).count()
    }

    fn is_deck_empty(&self) -> bool {
        self.deck_size() == 0
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
            self.game_and_table.score.add_assign(1);
        } else {
            self.game_and_table.scores.get_mut(0).unwrap().add_assign(1);
        }
        let mut turn = if result { Turn::Me() } else { Turn::Other() };
        if self.is_hand_empty() {
            turn = Turn::Done();
        }
        turn
    }

    fn play_card(&mut self) {
        let CardFromDeck { card, ind } = self.selector.select_card(&self.player.show_hand());
        self.player.play_card(ind);
        self.game_and_table.my_card = Some(card);
    }

    fn let_play_card(&mut self) {
        let other_card = self.player.let_play_card(0);
        *self.game_and_table.other_cards.get_mut(0).unwrap() = Some(other_card);
    }

    fn make_turn(&mut self) {
        // dbg!(&self.turn);
        let _ = self.printer.print_game(&GameState {
            hand: self.player.show_hand().iter().map(|f| f.card).collect(),
            table_cards: self.game_and_table.get_table_cards(),
            deck_cards: self.deck_size(),
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
            Turn::Me() => {
                self.play_card();
                self.game_and_table.me_start = true;
                Turn::OtherResponding()
            }
            Turn::OtherResponding() => {
                self.let_play_card();
                Turn::Battle()
            }
            Turn::Other() => {
                self.game_and_table.me_start = false;
                self.let_play_card();
                Turn::Response()
            }
            Turn::Response() => {
                self.play_card();
                Turn::Battle()
            }
            Turn::Battle() => {
                let card = self.game_and_table.my_card.unwrap();
                let other_card = self.game_and_table.other_cards.first().unwrap().unwrap();
                self.game_and_table.clear_table();
                self.battle_cards(card, other_card, self.game_and_table.me_start)
            }
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
        (self.game_and_table.score, self.game_and_table.scores)
    }
}
