use crate::domain::note::Note;
use crate::domain::tokenizer::Token;
use crate::panels::Focus;
use crate::services::ranker::ResultItem;

pub struct Model {
    pub notes: Vec<Note>,
    pub filtered_results: Vec<ResultItem>,
    pub filter_criteria: Vec<Token>,

    pub panel_focus: Focus,
    pub should_quit: bool,
}

impl Model {
    pub fn new(notes: Vec<Note>) -> Self {
        Self {
            notes,
            filtered_results: vec![],
            filter_criteria: vec![],
            panel_focus: Focus::Input,
            should_quit: false,
        }
    }
}
