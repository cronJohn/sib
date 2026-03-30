use ratatui::crossterm::event::KeyEvent;

use crate::message::Message;

#[derive(Default)]
pub struct LiveviewPanel;

impl LiveviewPanel {
    pub fn handle_key(&self, _key: KeyEvent) -> Option<Message> {
        todo!();
    }
}
