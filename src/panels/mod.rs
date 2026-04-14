pub mod filter;
pub mod input;
pub mod liveview;
pub mod notes;

/// Which TUI panel are we currently focusing?
#[derive(Default)]
pub enum Focus {
    #[default]
    Input,
    Notes,
    Filter,
    Liveview,
}

impl Focus {
    pub fn next(&self) -> Self {
        match self {
            Focus::Input => Focus::Notes,
            Focus::Notes => Focus::Filter,
            Focus::Filter => Focus::Input,
            Focus::Liveview => Focus::Input,
        }
    }
}
