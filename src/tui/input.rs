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
    match app.panel_focus {
        Focus::Input => app.input_panel.handle_key(key),
        Focus::Tree => app.tree_panel.handle_key(key),
        Focus::Filters => app.filter_panel.handle_key(key),
        Focus::Liveview => app.liveview_panel.handle_key(key),
    }
}
