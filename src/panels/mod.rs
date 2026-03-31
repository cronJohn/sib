pub mod filter;
pub mod input;
pub mod liveview;
pub mod tree;

/// Which TUI panel are we currently focusing?
#[derive(Default)]
pub enum Focus {
    #[default]
    Input,
    Tree,
    Filter,
    Liveview,
}

impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Focus::Input => Focus::Tree,
            Focus::Tree => Focus::Filter,
            Focus::Filter => Focus::Input,
            Focus::Liveview => Focus::Input,
        }
    }
}
