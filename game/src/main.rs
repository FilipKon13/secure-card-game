use game::{lobby_scene, table_scene, Scene};
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let application = Application::builder()
        .application_id("com.example.SecureCardGame")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("First GTK Program")
            .default_width(1200)
            .default_height(800)
            .resizable(false)
            .build();

        window.show_all();

        // Main game loop integrated with GTK's timeout_add
        let game_state = Rc::new(RefCell::new(GameState::new(window)));
        start_game_loop(game_state.clone());
    });

    application.run();
}

struct GameState {
    window: ApplicationWindow,
    current_scene: Box<dyn Scene>,
}

impl GameState {
    fn new(window: ApplicationWindow) -> Self {
        Self {
            window: window.clone(),
            current_scene: Box::new(table_scene::TableScene::new(window.clone())),
        }
    }

    fn update(&mut self) {
        self.current_scene.update();
    }
}

fn start_game_loop(game_state: Rc<RefCell<GameState>>) {
    // Using glib's timeout_add to schedule updates on the main GTK thread
    glib::timeout_add_local(std::time::Duration::from_millis(16), move || {
        game_state.borrow_mut().update();
        glib::Continue(true)
    });
}