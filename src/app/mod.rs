pub mod filter;
pub mod mode;
pub mod tree;

use crate::{
    app::{
        filter::{FilterCriteria, FilterItem},
        mode::{Focus, InputMode},
    },
    domain::{
        note::Note,
        selection::Selection,
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

    pub filter_items: Vec<FilterItem>,

    /// Focus for the TUI panels
    pub panel_focus: Focus,
    pub selected_filter_item: Selection,

    // Tree-related state
    pub tree_root: TreeNode,
    pub flattened_rows: Vec<Row>,
    pub selected_note_item: Selection,

    // input UI
    pub input_mode: InputMode,
    pub input_buffer: String, // What the user is currently typing

    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            filter: FilterCriteria::default(),
            tree_root: TreeNode::default(),
            flattened_rows: Vec::default(),
            filter_items: Vec::default(),
            panel_focus: Focus::default(),
            selected_filter_item: Selection::default(),
            input_mode: InputMode::default(),
            input_buffer: String::default(),
            selected_note_item: Selection::default(),
            should_quit: false,
        }
    }
}

impl App {
    pub fn initialize(&mut self, parser: &ParseService) {
        // Get initial notes
        self.notes = parser.collect_notes();

        // Sort them due to parallel processing
        self.notes.sort_by(|a, b| a.slug.cmp(&b.slug));

        self.recompute_view();
    }

    pub fn recompute_view(&mut self) {
        let indices = self.apply_filters();
        self.rebuild_tree(&indices);
        self.rebuild_filter_items();
    }
}
