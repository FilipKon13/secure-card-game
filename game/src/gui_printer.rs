use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

use common::game::{GamePrinter, GameState};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};

use crate::game_state::{Data, GameStateLobby, GameStateTable, SceneUpdate};
use crate::table_scene::TableScene;

pub struct GuiPrinter {
    pub table: Option<Rc<RefCell<TableScene>>>,
}

#[derive(Clone)]
pub struct GuiPrinterWrap {
    pub gui_printer: Rc<RefCell<GuiPrinter>>,
}

impl GamePrinter for GuiPrinter {
    fn print_game(&mut self, game_state: &GameState) {
        let GameState {
            hand,
            table_cards,
            deck_cards,
        } = game_state;
        self.table.as_mut().unwrap().borrow_mut().hand_cards = hand.to_vec();
        println!("Game State:");
        println!("Deck: {}", deck_cards);
        println!("Table: {}", format_cards(table_cards));
        println!("Hand: {}", format_cards(hand));
    }
}

impl GamePrinter for GuiPrinterWrap {
    fn print_game(&mut self, game_state: &GameState) {
        self.gui_printer.borrow_mut().print_game(game_state);
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
