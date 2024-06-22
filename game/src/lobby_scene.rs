use gtk::{Align, ApplicationWindow, Button, Box, Entry, Label};
use gtk::prelude::{ButtonExt, ContainerExt, EntryExt, WidgetExt};


pub struct LobbyScene {
    pub window: ApplicationWindow,
    pub redraw: bool,    
}

impl LobbyScene {
    pub fn new(window: ApplicationWindow) -> Self {
        Self {
            window: window,
            redraw: true,
        }
    }
}

impl super::Scene for LobbyScene {
    fn update(&mut self) {
        if self.redraw {
            for child in self.window.children() {
                self.window.remove(&child);
            }
            
            let vbox = Box::new(gtk::Orientation::Vertical, 10);

            self.window.add(&vbox);

            let some_text = Label::new(Some("Player id or something, idk"));

            let hbox = Box::new(gtk::Orientation::Horizontal, 0);
            hbox.set_halign(Align::Center);
            let some_input = Entry::new();
            some_input.set_width_request(200);
            hbox.add(&some_input);
            // some_input.set_width_chars(5);

            vbox.add(&some_text);
            vbox.add(&hbox);
            
            let hbox = Box::new(gtk::Orientation::Horizontal, 0);
            hbox.set_halign(Align::Center);

            let btn = Button::with_label("Start game");
            btn.connect_clicked(move |_| {
                println!("clicked, {}", some_input.text());
            });
            hbox.add(&btn);
            vbox.add(&hbox);

            self.window.show_all();
            self.redraw = false;
        }    
    }
}