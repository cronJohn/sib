use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{mode::InputMode, panels::input::InputPanel};

/// Widget to let user input string to filter notes
pub fn render_input_widget(f: &mut Frame, area: Rect, panel: &InputPanel, is_focused: bool) {
    let title = match panel.mode {
        InputMode::Path => "Path",
        InputMode::Tag => "Tag",
        InputMode::Meta => "Meta",
    };

    let border_style = if is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let text = format!("[{}] {}", title, panel.buffer);

    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Input")
            .border_style(border_style),
    );

    f.render_widget(widget, area);
}
