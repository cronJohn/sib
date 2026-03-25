use crate::{app::App, context::Context, message::Message};

impl App {
    pub fn update(&mut self, msg: Message, _ctx: &Context) {
        match msg {
            Message::Quit => self.should_quit = true,
            Message::InputChar(c) => self.path_filter.push(c),
            Message::NoteSelectionUp => {
                if self.selected_note_entry > 0 {
                    self.selected_note_entry -= 1;
                }
            }

            Message::NoteSelectionDown => {
                if self.selected_note_entry + 1 < self.notes.len() {
                    self.selected_note_entry += 1;
                }
            }
            Message::DeleteChar => {
                self.path_filter.pop();
            }
        }
    }
}
