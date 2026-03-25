use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Widget to show live file contents of a Note
pub fn render_liveview_widget(f: &mut Frame, area: Rect, _app: &App) {
    let paragraph = Paragraph::new("Bottom row text")
        .block(Block::default().borders(Borders::ALL).title("Bottom"));
    f.render_widget(paragraph, area);
}
