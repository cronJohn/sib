use crate::domain::note::Note;
use crate::domain::tokenizer::Token;
use crate::effect::Effect;
use crate::panels::Focus;
use crate::services::ranker::ResultItem;

pub struct Model {
    /// All notes inside base_notes_dir
    pub notes: Vec<Note>,
    /// Collection of indices into notes based on rank score
    pub ranked_notes: Vec<ResultItem>,
    /// Collection of Tokens the user wants to filter/rank notes by
    pub token_filters: Vec<Token>,
    /// Collection of Events that need to run
    pub pending_effects: Vec<Effect>,

    pub panel_focus: Focus,
    pub should_quit: bool,
}

impl Model {
    pub fn new(notes: Vec<Note>) -> Self {
        Self {
            notes,
            ranked_notes: vec![],
            token_filters: vec![],
            pending_effects: vec![],
            panel_focus: Focus::Input,
            should_quit: false,
        }
    }
}
