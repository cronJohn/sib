use ratatui::{
    Frame,
    crossterm::event::KeyEvent,
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::render_context::RenderContext, message::Message};

pub struct LiveviewPanel;

impl LiveviewPanel {
    pub fn handle_key(&self, _key: KeyEvent) -> Message {
        Message::Noop
    }

    pub fn render(&self, f: &mut Frame, area: Rect, _ctx: &RenderContext) {
        let paragraph = Paragraph::new("Liveview coming soon")
            .block(Block::default().borders(Borders::ALL).title("Liveview"));
        f.render_widget(paragraph, area);
    }
}
