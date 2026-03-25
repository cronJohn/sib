use std::collections::BTreeMap;

#[derive(Default)]
pub struct TreeNode {
    pub children: BTreeMap<String, TreeNode>, // directories
    pub notes: Vec<usize>,                    // indices into App.notes
}

pub enum Row {
    Directory {
        name: String,
        depth: usize,
    },
    Note {
        name: String,
        depth: usize,
        index: usize,
    },
}
