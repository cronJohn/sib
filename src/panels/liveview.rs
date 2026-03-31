use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{message::Message, widgets::render_liveview_widget};

#[derive(Default)]
pub struct LiveviewPanel;

impl LiveviewPanel {
    pub fn handle_key(&self, _key: KeyEvent) -> Message {
        Message::Noop
    }

    pub fn render(&self, f: &mut Frame, area: Rect, is_focused: bool) {
        render_liveview_widget(f, area, is_focused);
    }
}
