use crate::domain::{
    note::Note,
    selection::Selection,
    tree::{Row, TreeNode},
};

/// Panel state for the notes tree
#[derive(Default)]
pub struct TreePanel {
    /// Hierarchical tree of directories and notes
    pub tree_root: TreeNode,
    /// Flattened tree for rendering and selection
    pub flattened_rows: Vec<Row>,
    /// Currently selected note (skips directories automatically)
    pub selection: Selection,
}

impl TreePanel {
    pub fn new() -> Self {
        Self {
            tree_root: TreeNode::default(),
            flattened_rows: Vec::new(),
            selection: Selection::default(),
        }
    }

    pub fn rebuild(&mut self, notes: &[Note], filtered_indices: &[usize]) {
        let mut root = TreeNode::default();

        // Build tree
        for &idx in filtered_indices {
            let note = &notes[idx];
            let mut node = &mut root;

            let parts: Vec<_> = note
                .slug
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect();

            for dir in &parts[..parts.len() - 1] {
                node = node.children.entry(dir.clone()).or_default();
            }

            node.notes.push(idx);
        }

        // Flatten
        self.flattened_rows.clear();
        Self::flatten_recursive(&root, 0, notes, &mut self.flattened_rows);

        self.tree_root = root;

        // Fix selection
        self.selection
            .select_first(&self.flattened_rows, |r| r.is_selectable());
    }

    fn flatten_recursive(node: &TreeNode, depth: usize, notes: &[Note], rows: &mut Vec<Row>) {
        for (dir_name, child) in &node.children {
            rows.push(Row::Directory {
                name: dir_name.clone(),
                depth,
            });

            Self::flatten_recursive(child, depth + 1, notes, rows);
        }

        for &note_idx in &node.notes {
            let note = &notes[note_idx];

            let name = note.slug.file_name().unwrap().to_string_lossy().to_string();

            rows.push(Row::Note {
                name,
                depth,
                index: note_idx,
            });
        }
    }
}
