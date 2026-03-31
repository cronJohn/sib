use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    layout::Rect,
    Frame,
};

use crate::{
    domain::{selection::Selection, tree::TreeItem},
    message::Message,
    widgets::{render_tree_widget, tree::TreeWidgetOptions},
};

#[derive(Default)]
pub struct TreePanel {
    pub selection_index: Selection,
}

impl TreePanel {
    pub fn handle_key(&self, key: KeyEvent) -> Message {
        match key.code {
            KeyCode::Up => Message::TreeSelectionUp,
            KeyCode::Down => Message::TreeSelectionDown,
            KeyCode::Enter => Message::OpenSelected,
            _ => Message::Noop,
        }
    }

    pub fn render(&self, f: &mut Frame, area: Rect, items: &[TreeItem], is_focused: bool) {
        render_tree_widget(
            f,
            area,
            TreeWidgetOptions {
                items,
                selection_index: self.selection_index.get(),
                is_focused,
            },
        );
    }
}
