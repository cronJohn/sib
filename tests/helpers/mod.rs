use sib::domain::note::Note;

pub fn sort_notes(mut notes: Vec<Note>) -> Vec<Note> {
    notes.sort_by(|a, b| a.path.cmp(&b.path));
    notes
}
