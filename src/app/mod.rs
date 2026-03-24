use crate::domain::note::Note;

mod update;

#[derive(Default)]
pub struct App {
    pub notes: Vec<Note>,
    pub filtered_notes: Vec<Note>,
    pub selected: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            filtered_notes: Vec::new(),
            selected: 0,
            should_quit: false,
        }
    }

    pub fn selected_note(&self) -> Option<&Note> {
        self.filtered_notes.get(self.selected)
    }
}
