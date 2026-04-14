use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

use crate::{
    message::Message,
    model::Model,
    panels::Focus,
    widgets::{input::InputWidgetOptions, render_input_widget},
};

#[derive(Default)]
pub struct InputPanel {
    pub buffer: String,
}

impl InputPanel {
    pub fn handle_key(&self, key: KeyEvent) -> Message {
        use KeyCode::*;

        match (key.code, key.modifiers) {
            (Char(c), _) => Message::InputChar(c),
            (Backspace, _) => Message::InputBackspace,

            _ => Message::Noop,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, model: &Model) {
        render_input_widget(
            f,
            area,
            InputWidgetOptions {
                buffer: &self.buffer,
                is_focused: matches!(model.panel_focus, Focus::Input),
            },
        );
    }
}
