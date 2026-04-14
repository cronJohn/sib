use ratatui::{crossterm::event::KeyEvent, layout::Rect, Frame};

use crate::{
    message::Message,
    model::Model,
    panels::Focus,
    widgets::{notes::NoteWidgetOptions, render_notes_widget},
};

#[derive(Default)]
pub struct NotesPanel {
    pub selection_index: usize,
}

impl NotesPanel {
    pub fn handle_key(&self, _key: KeyEvent) -> Message {
        Message::Noop
    }

    pub fn render(&self, f: &mut Frame, area: Rect, model: &Model) {
        let items: Vec<String> = model
            .filtered_results
            .iter()
            .map(|r| {
                let note = &model.notes[r.note_index];
                format!("{}", note.slug.to_string_lossy())
            })
            .collect();

        let options = NoteWidgetOptions {
            selected_index: self.selection_index,
            is_focused: matches!(model.panel_focus, Focus::Notes),
            items,
        };

        render_notes_widget(f, area, &options);
    }
}
