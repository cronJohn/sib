use tracing::debug;

use crate::message::Message;

#[derive(Default)]
pub struct App {
    pub items: Vec<String>,
    pub selected: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            items: vec![
                "Hello".into(),
                "World".into(),
                "SIB".into(),
                "Simple Is Best".into(),
                "Search Index Browse".into(),
            ],
            selected: 0,
            should_quit: false,
        }
    }

    pub fn selected_item(&self) -> Option<&String> {
        self.items.get(self.selected)
    }

    pub fn update(&mut self, msg: Message) {
        debug!("Handling message: {:?}", msg);

        match msg {
            Message::MoveUp => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            Message::MoveDown => {
                if self.selected + 1 < self.items.len() {
                    self.selected += 1;
                }
            }
            Message::Quit => {
                self.should_quit = true;
            }
        }
    }
}
