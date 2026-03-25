use crate::{
    domain::{
        note::Note,
        tree::{Row, TreeNode},
    },
    services::parse::ParseService,
};

mod update;

#[derive(Default)]
pub struct App {
    pub notes: Vec<Note>,
    pub path_filter: String,
    pub selected_note_entry: usize,
    pub should_quit: bool,

    // Tree-related fields
    pub tree_root: TreeNode,
    pub flattened_rows: Vec<Row>,
}

impl App {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            path_filter: String::new(),
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
        self.build_tree();
        self.select_first_note(); // So we don't start on a directory
    }

    pub fn build_tree(&mut self) {
        let mut root = TreeNode::default();

        for (idx, note) in self.notes.iter().enumerate() {
            let mut node = &mut root;
            let parts: Vec<_> = note
                .slug
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect();

            // All path components except last = directories
            for dir in &parts[..parts.len() - 1] {
                node = node.children.entry(dir.clone()).or_default();
            }

            node.notes.push(idx);
        }

        self.tree_root = root;

        // Flatten tree for UI rendering
        self.flattened_rows.clear();
        flatten_tree(&self.tree_root, 0, &mut self.flattened_rows, &self.notes);
    }

    pub fn select_first_note(&mut self) {
        self.selected_note_entry = self
            .flattened_rows
            .iter()
            .position(|row| matches!(row, Row::Note { .. }))
            .unwrap_or(0);
    }
}

fn flatten_tree(node: &TreeNode, depth: usize, rows: &mut Vec<Row>, notes: &[Note]) {
    for (dir_name, child) in &node.children {
        rows.push(Row::Directory {
            name: dir_name.clone(),
            depth,
        });
        flatten_tree(child, depth + 1, rows, notes);
    }

    for &note_idx in &node.notes {
        let note = &notes[note_idx];
        let name = note
            .slug
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        rows.push(Row::Note {
            name,
            depth,
            index: note_idx,
        });
    }
}
