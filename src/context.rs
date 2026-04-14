use crate::{
    config::Config,
    services::{
        editor::EditorService, note::NoteService, parse::ParseService, ranker::RankerService,
    },
};

pub struct Context {
    pub notes: NoteService,
    pub editor: EditorService,
    pub parser: ParseService,
    pub ranker: RankerService,
}

impl Context {
    pub fn new(cfg: &Config) -> Self {
        Self {
            notes: NoteService::new(cfg.base_notes_dir.clone()),
            editor: EditorService::new(cfg.editor.clone()),
            parser: ParseService::new(cfg.base_notes_dir.clone()),
            ranker: RankerService::new(cfg.usage_file.clone()),
        }
    }
}
