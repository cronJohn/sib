use crate::app::mode::InputMode;

/// Panel state for user input
#[derive(Default)]
pub struct InputPanel {
    /// Current input mode: Path, Tag, or Meta
    pub mode: InputMode,
    /// Text buffer being typed by the user
    pub buffer: String,
}

impl InputPanel {
    pub fn new() -> Self {
        Self {
            mode: InputMode::Path,
            buffer: String::new(),
        }
    }

    pub fn add_char(&mut self, char: char) {
        self.buffer.push(char);
    }

    pub fn remove_char(&mut self) {
        self.buffer.pop();
    }

    pub fn clear_buf(&mut self) {
        self.buffer.clear();
    }

    pub fn get_buf(&self) -> &str {
        &self.buffer
    }

    pub fn change_mode(&mut self, mode: InputMode) {
        self.mode = mode;
    }

    pub fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            InputMode::Path => InputMode::Tag,
            InputMode::Tag => InputMode::Meta,
            InputMode::Meta => InputMode::Path,
        }
    }
}
