use crate::{app::App, context::Context, domain::tree::Row, message::Message};

impl App {
    pub fn update(&mut self, msg: Message, _ctx: &Context) {
        match msg {
            Message::Quit => self.should_quit = true,
            Message::InputChar(c) => self.path_filter.push(c),
            Message::NoteSelectionUp => {
                let mut i = self.selected_note_entry.saturating_sub(1);
                while i > 0 {
                    if matches!(self.flattened_rows[i], Row::Note { .. }) {
                        self.selected_note_entry = i;
                        break;
                    }
                    i = i.saturating_sub(1);
                }
            }

            Message::NoteSelectionDown => {
                let mut i = self.selected_note_entry + 1;
                while i < self.flattened_rows.len() {
                    if matches!(self.flattened_rows[i], Row::Note { .. }) {
                        self.selected_note_entry = i;
                        break;
                    }
                    i += 1;
                }
            }

            Message::DeleteChar => {
                self.path_filter.pop();
            }
        }
    }
}
