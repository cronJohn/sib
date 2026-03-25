use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, message::Message};

pub fn handle_key(key: KeyEvent, _app: &mut App) -> Option<Message> {
    match key.code {
        KeyCode::Esc => Some(Message::Quit),
        KeyCode::Char(c) => Some(Message::InputChar(c)),
        KeyCode::Backspace => Some(Message::DeleteChar),
        KeyCode::Up => Some(Message::NoteSelectionUp),
        KeyCode::Down => Some(Message::NoteSelectionDown),
        _ => None,
    }
}
