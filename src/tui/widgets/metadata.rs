use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

/// Widget to render and filter by Note metadata
pub fn render_metadata_widget(f: &mut Frame, area: Rect, _app: &App) {
    let paragraph = Paragraph::new("Right column text")
        .block(Block::default().borders(Borders::ALL).title("Metadata"));
    f.render_widget(paragraph, area);
}
