use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{mode::InputMode, App};

/// Widget to let user input string to filter notes by slug
pub fn render_input_widget(f: &mut Frame, area: Rect, app: &App) {
    let mode = match app.input_mode {
        InputMode::Path => "PATH",
        InputMode::Tag => "TAG",
        InputMode::Meta => "META",
    };

    let text = format!("[{}] {}", mode, app.input_buffer);

    let widget = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Input"));

    f.render_widget(widget, area);
}
