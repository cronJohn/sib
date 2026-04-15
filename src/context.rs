use crate::{
    config::Config,
    services::{editor::EditorService, parse::ParseService, ranker::RankerService},
};

pub struct Context {
    pub editor: EditorService,
    pub parser: ParseService,
    pub ranker: RankerService,
}

impl Context {
    pub fn new(cfg: &Config) -> Self {
        Self {
            editor: EditorService::new(cfg.editor.clone(), cfg.base_notes_dir.clone()),
            parser: ParseService::new(cfg.base_notes_dir.clone()),
            ranker: RankerService::new(cfg.usage_file.clone()),
        }
    }
}
