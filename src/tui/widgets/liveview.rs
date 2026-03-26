use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::panels::liveview::LiveviewPanel;

/// Widget to show live file contents of a Note
pub fn render_liveview_widget(
    f: &mut Frame,
    area: Rect,
    _panel: &LiveviewPanel,
    _is_focused: bool,
) {
    let paragraph = Paragraph::new("Liveview coming soon")
        .block(Block::default().borders(Borders::ALL).title("Liveview"));
    f.render_widget(paragraph, area);
}
