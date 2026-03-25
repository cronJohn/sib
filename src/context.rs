use crate::{
    config::Config,
    services::{editor::EditorService, note::NoteService, parse::ParseService},
};

pub struct Context {
    pub notes: NoteService,
    pub editor: EditorService,
    pub parser: ParseService,
}

impl Context {
    pub fn new(cfg: &Config) -> Self {
        Self {
            notes: NoteService::new(cfg.base_notes_dir.clone()),
            editor: EditorService::new(cfg.editor.clone()),
            parser: ParseService::new(cfg.base_notes_dir.clone()),
        }
    }
}
