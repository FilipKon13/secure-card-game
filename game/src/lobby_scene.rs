use gtk::prelude::{ButtonExt, ContainerExt, EntryExt, GtkWindowExt, WidgetExt};
use gtk::{Align, ApplicationWindow, Box, Button, Entry, Label};

use crate::image_database::ImageDatabase;
use crate::window;

pub struct LobbyScene {
    pub window: ApplicationWindow,
    pub redraw: bool,
    pub image_database: ImageDatabase,
}

impl LobbyScene {
    pub fn new(window: ApplicationWindow) -> Self {
        Self {
            window,
            redraw: true,
            image_database: ImageDatabase::default(),
        }
    }
}

impl super::Scene for LobbyScene {
    fn update(&mut self) {
        if self.redraw {
            for child in self.window.children() {
                self.window.remove(&child);
            }

            let vbox = Box::new(gtk::Orientation::Vertical, 20);

            self.window.add(&vbox);

            let num_players_text = Label::new(Some("Number of players"));

            let hbox_num_players = Box::new(gtk::Orientation::Horizontal, 0);
            hbox_num_players.set_halign(Align::Center);
            let num_players_input = Entry::new();
            num_players_input.set_width_request(200);
            hbox_num_players.add(&num_players_input);

            vbox.add(&num_players_text);
            vbox.add(&hbox_num_players);

            let player_id_text = Label::new(Some("Player id"));

            let hbox_player_id = Box::new(gtk::Orientation::Horizontal, 0);
            hbox_player_id.set_halign(Align::Center);
            let player_id_input = Entry::new();
            player_id_input.set_width_request(200);
            hbox_player_id.add(&player_id_input);

            vbox.add(&player_id_text);
            vbox.add(&hbox_player_id);

            let hbox_player_id = Box::new(gtk::Orientation::Horizontal, 0);
            hbox_player_id.set_halign(Align::Center);

            let btn = Button::with_label("Start game");

            let window_clone = self.window.clone();
            btn.connect_clicked(move |_| {
                let num_players: u32 = num_players_input.text().as_str().parse().unwrap();
                let player_id: u32 = player_id_input.text().as_str().parse().unwrap();
                println!("clicked, {} {}", num_players, player_id);
                window_clone.close();
                unsafe {
                    window::PLAYER_ID = player_id;
                }
                unsafe {
                    window::NUM_PLAYERS = num_players;
                }
            });
            hbox_player_id.add(&btn);
            vbox.add(&hbox_player_id);

            self.window.show_all();
            self.redraw = false;
        }
    }
}
