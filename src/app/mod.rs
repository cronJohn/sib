use crate::domain::note::Note;

mod update;

#[derive(Default)]
pub struct App {
    pub notes: Vec<Note>,
    pub path_filter: String,
    pub filtered_notes: Vec<Note>,
    pub selected_note_entry: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            path_filter: String::new(),
            filtered_notes: Vec::new(),
            selected_note_entry: 0,
            should_quit: false,
        }
    }

    pub fn selected_note(&self) -> Option<&Note> {
        self.filtered_notes.get(self.selected_note_entry)
    }
}
