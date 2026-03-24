use crate::{
    config::Config,
    services::{editor::EditorService, note::NoteService},
};

pub struct Context {
    pub notes: NoteService,
    pub editor: EditorService,
}

impl Context {
    pub fn new(cfg: &Config) -> Self {
        Self {
            notes: NoteService::new(cfg.notes_dir.clone()),
            editor: EditorService::new(cfg.editor.clone()),
        }
    }
}
