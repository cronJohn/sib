use ratatui::{
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    Frame,
};
use thiserror::Error;
use tracing::error;

use crate::{
    message::Message,
    panels::filter::FilterItem,
    widgets::{input::InputWidgetOptions, render_input_widget},
};

#[derive(Clone, Copy, Debug, Default)]
pub enum InputMode {
    #[default]
    Path,
    Tag,
    Meta,
}

impl InputMode {
    pub fn next(self) -> Self {
        match self {
            Self::Path => Self::Tag,
            Self::Tag => Self::Meta,
            Self::Meta => Self::Path,
        }
    }
}

#[derive(Default)]
pub struct InputPanel {
    pub buffer: String,
    pub mode: InputMode,
}

impl InputPanel {
    pub fn handle_key(&self, key: KeyEvent) -> Message {
        use KeyCode::*;

        match (key.code, key.modifiers) {
            (Char('t'), KeyModifiers::CONTROL) => Message::SwitchInputMode(InputMode::Path),
            (Char('s'), KeyModifiers::CONTROL) => Message::SwitchInputMode(InputMode::Tag),
            (Char('r'), KeyModifiers::CONTROL) => Message::SwitchInputMode(InputMode::Meta),

            (Char(c), _) => Message::InputChar(c),
            (Backspace, _) => Message::InputBackspace,
            (Enter, _) => Message::AddFilter,

            _ => Message::Noop,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        let mode_str = match self.mode {
            InputMode::Path => "Path",
            InputMode::Tag => "Tag",
            InputMode::Meta => "Meta",
        };

        render_input_widget(
            f,
            area,
            InputWidgetOptions {
                mode: mode_str,
                buffer: &self.buffer,
                is_focused,
            },
        );
    }

    pub fn get_filter_item(&self) -> Result<FilterItem, InputPanelError> {
        use InputMode::*;

        if self.buffer.is_empty() {
            return Err(InputPanelError::EmptyBuffer);
        }

        match self.mode {
            Path => Ok(FilterItem::Slug(self.buffer.clone())),
            Tag => Ok(FilterItem::Tag(self.buffer.clone())),
            Meta => {
                if let Some((k, v)) = self.buffer.split_once(':') {
                    Ok(FilterItem::Meta(k.trim().to_owned(), v.trim().to_owned()))
                } else {
                    error!("Unable to handle input data");
                    Err(InputPanelError::InvalidMetadata)
                }
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum InputPanelError {
    #[error("Metadata is improperly formatted")]
    InvalidMetadata,
    #[error("Input buffer is empty")]
    EmptyBuffer,
}
