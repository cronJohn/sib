use crate::{app::App, context::Context, message::Message};

impl App {
    pub fn update(&mut self, msg: Message, ctx: &Context) {
        match msg {
            Message::Quit => self.should_quit = true,
            Message::OpenNote => {
                if let Some(note) = self.selected_note() {
                    let _ = ctx.editor.open(note.path.to_str().unwrap());
                }
            }
            Message::DeleteNote => {
                if let Some(note) = self.selected_note() {
                    let _ = ctx.notes.delete_note(&note.path);
                }
            }
            _ => {}
        }
    }
}
