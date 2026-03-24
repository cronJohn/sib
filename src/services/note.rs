use crate::domain::note::Note;
use std::fs;
use std::path::PathBuf;

pub struct NoteService {
    base_dir: PathBuf,
}

impl NoteService {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn delete_note(&self, path: &PathBuf) -> std::io::Result<()> {
        fs::remove_file(self.base_dir.join(path))
    }

    pub fn list_notes(&self) -> std::io::Result<Vec<Note>> {
        // Walk filesystem, parse markdown frontmatter, return Vec<Note>
        Ok(vec![])
    }
}
