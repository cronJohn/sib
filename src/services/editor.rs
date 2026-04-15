use std::{path::PathBuf, process::Command};

use crate::domain::note::Note;

pub struct EditorService {
    editor: String,
    base_notes_dir: PathBuf,
}

impl EditorService {
    pub fn new(editor: String, base_notes_dir: PathBuf) -> Self {
        Self {
            editor,
            base_notes_dir,
        }
    }

    pub fn open(&self, note: &Note) -> std::io::Result<()> {
        let path = self.base_notes_dir.join(note.slug.clone());
        Command::new(&self.editor).arg(path).status()?;
        Ok(())
    }
}
