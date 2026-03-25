use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Widget to let user input string to filter notes by slug
pub fn render_input_widget(f: &mut Frame, area: Rect, app: &App) {
    let paragraph = Paragraph::new(app.path_filter.as_str())
        .block(Block::default().borders(Borders::ALL).title("Path Input"));
    f.render_widget(paragraph, area);
}
