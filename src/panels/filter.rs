use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

use crate::{
    domain::selection::Selection,
    message::Message,
    widgets::{filter::FilterWidgetOptions, render_filter_widget},
};

#[derive(Debug)]
pub enum FilterItem {
    Slug(String),
    Tag(String),
    Meta(String, String),
}

#[derive(Default)]
pub struct FilterPanel {
    pub selection_index: Selection,
}

impl FilterPanel {
    pub fn handle_key(&self, key: KeyEvent) -> Message {
        match key.code {
            KeyCode::Up => Message::FilterSelectionUp,
            KeyCode::Down => Message::FilterSelectionDown,
            KeyCode::Char('d') => Message::DeleteSelectedFilter,
            _ => Message::Noop,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, items: &[FilterItem], is_focused: bool) {
        render_filter_widget(
            f,
            area,
            FilterWidgetOptions {
                items,
                selected_index: Some(self.selection_index.get()),
                is_focused,
            },
        );
    }
}
