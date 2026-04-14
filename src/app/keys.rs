use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{app::App, message::Message, panels::Focus};

impl App {
    pub fn route_key(&self, key: KeyEvent) -> Message {
        use Focus::*;
        use KeyCode::*;

        // Global keybinds
        match (key.code, key.modifiers) {
            (Esc, _) => return Message::Quit,
            (Tab, _) => return Message::CycleFocusForward,
            (Up, _) => return Message::NoteSelectionUp,
            (Down, _) => return Message::NoteSelectionDown,
            (Enter, _) => return Message::OpenSelected,
            _ => {}
        }

        // Panel specific keybinds
        match self.model.panel_focus {
            Input => self.input_panel.handle_key(key),
            Notes => self.notes_panel.handle_key(key),
            Filter => self.filter_panel.handle_key(key),
            Liveview => self.liveview_panel.handle_key(key),
        }
    }
}
