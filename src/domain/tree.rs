use crate::domain::note::Note;

/// Each node in the virtual tree
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub name: String,              // component name (directory or note)
    pub children: Vec<TreeNode>,   // nested nodes
    pub note_index: Option<usize>, // Some(idx) if a note, None if a directory
}

impl TreeNode {
    pub fn is_selectable(&self) -> bool {
        self.note_index.is_some()
    }

    /// Flatten tree for rendering (depth = indentation)
    pub fn flatten(&self, depth: usize, out: &mut Vec<TreeItem>) {
        if !self.name.is_empty() {
            out.push(TreeItem {
                display_name: self.name.clone(),
                depth,
                note_index: self.note_index,
                selectable: self.is_selectable(),
            });
        }
        for child in &self.children {
            child.flatten(depth + 1, out);
        }
    }
}

/// Flattened item used in the widget
#[derive(Debug, Clone)]
pub struct TreeItem {
    pub display_name: String,
    pub depth: usize,
    pub note_index: Option<usize>, // None for directories
    pub selectable: bool,
}

impl TreeItem {
    pub fn is_selectable(&self) -> bool {
        self.selectable
    }
}

/// Tree helper: create virtual tree from notes
pub struct VirtualTree;

impl VirtualTree {
    pub fn build_from_notes(notes: &[Note], filtered_indices: &[usize]) -> TreeNode {
        let mut root = TreeNode {
            name: "".to_string(),
            children: Vec::new(),
            note_index: None,
        };

        for &idx in filtered_indices {
            let note = &notes[idx];
            let mut current = &mut root;

            for component in note.slug.iter() {
                let comp_str = component.to_string_lossy().to_string();

                // Find the position of an existing child
                let pos = current.children.iter().position(|c| c.name == comp_str);

                current = match pos {
                    Some(i) => &mut current.children[i],
                    None => {
                        current.children.push(TreeNode {
                            name: comp_str.clone(),
                            children: Vec::new(),
                            note_index: None,
                        });
                        current.children.last_mut().unwrap()
                    }
                };
            }

            current.note_index = Some(idx);
        }

        root
    }

    pub fn flatten(root: &TreeNode) -> Vec<TreeItem> {
        let mut out = Vec::new();
        root.flatten(0, &mut out);
        out
    }
}
