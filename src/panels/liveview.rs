use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{message::Message, model::Model, panels::Focus, widgets::render_liveview_widget};

pub struct LiveviewPanel;

impl LiveviewPanel {
    pub fn handle_key(&self, _key: KeyEvent) -> Message {
        Message::Noop
    }

    pub fn render(&self, f: &mut Frame, area: Rect, model: &Model) {
        render_liveview_widget(f, area, matches!(model.panel_focus, Focus::Liveview));
    }
}
