pub mod filter;
pub mod tree;

use crate::{
    app::filter::FilterCriteria,
    domain::{
        note::Note,
        tree::{Row, TreeNode},
    },
    services::parse::ParseService,
};

mod update;

#[derive(Default)]
pub struct App {
    /// All notes collected from parser
    pub notes: Vec<Note>,

    pub filter: FilterCriteria,

    // Tree-related state
    pub tree_root: TreeNode,
    pub flattened_rows: Vec<Row>,

    // UI state
    pub selected_note_entry: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            filter: FilterCriteria::default(),
            selected_note_entry: 0,
            should_quit: false,
            tree_root: TreeNode::default(),
            flattened_rows: Vec::default(),
        }
    }
}

impl App {
    pub fn initialize(&mut self, parser: &ParseService) {
        // Get initial notes
        self.notes = parser.collect_notes();

        // Sort them due to parallel processing
        self.notes.sort_by(|a, b| a.slug.cmp(&b.slug));

        let indices = self.apply_filters();
        self.rebuild_tree(&indices);
    }
}
