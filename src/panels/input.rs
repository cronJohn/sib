use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::render_context::RenderContext, message::Message, panels::Focus};

#[derive(Default)]
pub struct InputPanel {
    pub buffer: String,
}

impl InputPanel {
    pub fn handle_key(&self, key: KeyEvent) -> Message {
        use KeyCode::*;

        match (key.code, key.modifiers) {
            (Char(c), _) => Message::InputChar(c),
            (Backspace, _) => Message::InputBackspace,

            _ => Message::Noop,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, ctx: &RenderContext) {
        let border_style = if matches!(ctx.model.panel_focus, Focus::Input) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let paragraph = Paragraph::new(self.buffer.to_owned()).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Input")
                .border_style(border_style),
        );

        f.render_widget(paragraph, area);
    }
}
