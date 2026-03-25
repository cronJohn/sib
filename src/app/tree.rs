use crate::{
    app::App,
    domain::{
        note::Note,
        tree::{Row, TreeNode},
    },
};

impl App {
    pub fn rebuild_tree(&mut self, filtered_indices: &[usize]) {
        let mut root = TreeNode::default();

        for &idx in filtered_indices {
            let note = &self.notes[idx];
            let mut node = &mut root;
            let parts: Vec<_> = note
                .slug
                .iter()
                .map(|s| s.to_string_lossy().to_string())
                .collect();

            // directories = all but last component
            for dir in &parts[..parts.len() - 1] {
                node = node.children.entry(dir.clone()).or_default();
            }

            node.notes.push(idx);
        }

        self.tree_root = root;

        // Flatten tree for rendering
        self.flattened_rows.clear();
        flatten_tree(&self.tree_root, 0, &mut self.flattened_rows, &self.notes);

        // Ensure selected note points to first leaf
        self.selected_note_entry = self
            .flattened_rows
            .iter()
            .position(|r| matches!(r, Row::Note { .. }))
            .unwrap_or(0);
    }
}

/// Recursively flatten tree to Vec<Row>
pub fn flatten_tree(node: &TreeNode, depth: usize, rows: &mut Vec<Row>, notes: &[Note]) {
    for (dir_name, child) in &node.children {
        rows.push(Row::Directory {
            name: dir_name.clone(),
            depth,
        });
        flatten_tree(child, depth + 1, rows, notes);
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
