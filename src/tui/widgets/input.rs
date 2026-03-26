use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::{
    mode::{Focus, InputMode},
    App,
};

/// Widget to let user input string to filter notes by slug
pub fn render_input_widget(f: &mut Frame, area: Rect, app: &App) {
    let mode = match app.input_mode {
        InputMode::Path => "PATH",
        InputMode::Tag => "TAG",
        InputMode::Meta => "META",
    };

    let border_style = if matches!(app.focus, Focus::Input) {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let text = format!("[{}] {}", mode, app.input_buffer);

    let widget = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Input")
            .border_style(border_style),
    );

    f.render_widget(widget, area);
}
