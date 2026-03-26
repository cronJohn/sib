use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tracing::info;

use crate::{
    app::{
        mode::{Focus, InputMode},
        App,
    },
    message::Message,
};

pub fn handle_key(key: KeyEvent, app: &mut App) -> Option<Message> {
    use KeyCode::*;

    info!(
        "{}{:?}",
        if key.modifiers.is_empty() {
            "".to_string()
        } else {
            format!("{:?}-", key.modifiers)
                .replace("KeyModifiers(", "")
                .replace(")", "")
        },
        key.code
    );
    match (key.code, key.modifiers) {
        (Esc, _) => return Some(Message::Quit),
        (Tab, _) => return Some(Message::CycleFocusForward),

        // Mode switching (Ctrl + key)
        (Char('t'), KeyModifiers::CONTROL) => return Some(Message::SwitchMode(InputMode::Path)),
        (Char('s'), KeyModifiers::CONTROL) => return Some(Message::SwitchMode(InputMode::Tag)),
        (Char('r'), KeyModifiers::CONTROL) => return Some(Message::SwitchMode(InputMode::Meta)),

        _ => {}
    }

    // Pane specific keybindings
    match app.focus {
        Focus::Input => handle_input_keys(key),
        Focus::Notes => handle_notes_keys(key),
        Focus::Filters => handle_filter_keys(key),
    }
}

fn handle_input_keys(key: KeyEvent) -> Option<Message> {
    use KeyCode::*;

    match (key.code, key.modifiers) {
        (Char(c), _) => Some(Message::InputChar(c)),
        (Backspace, _) => Some(Message::DeleteChar),
        (Enter, _) => Some(Message::SubmitInput),
        _ => None,
    }
}

fn handle_notes_keys(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Up => Some(Message::NoteSelectionUp),
        KeyCode::Down => Some(Message::NoteSelectionDown),
        _ => None,
    }
}

fn handle_filter_keys(key: KeyEvent) -> Option<Message> {
    match key.code {
        KeyCode::Up => Some(Message::FilterUp),
        KeyCode::Down => Some(Message::FilterDown),

        KeyCode::Char('d') => Some(Message::DeleteSelectedFilter),

        _ => None,
    }
}
