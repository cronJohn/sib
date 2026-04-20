use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{
    message::Message,
    model::Model,
    panels::Focus,
    widgets::{filter::FilterWidgetOptions, render_filter_widget},
};

pub struct FilterPanel;

impl FilterPanel {
    pub fn handle_key(&self, _key: KeyEvent) -> Message {
        Message::Noop
    }

    pub fn render(&self, f: &mut Frame, area: Rect, model: &Model) {
        render_filter_widget(
            f,
            area,
            FilterWidgetOptions {
                items: &model.token_filters,
                is_focused: matches!(model.panel_focus, Focus::Filter),
            },
        );
    }
}
