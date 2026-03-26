pub mod mode;
pub mod panels;

use crate::{
    app::{
        mode::Focus,
        panels::{
            filter::FilterPanel, input::InputPanel, liveview::LiveviewPanel, tree::TreePanel,
        },
    },
    domain::note::Note,
    services::parse::ParseService,
};

mod update;

#[derive(Default)]
pub struct App {
    /// All notes collected from parser
    pub notes: Vec<Note>,

    // Panels
    pub filter_panel: FilterPanel,
    pub tree_panel: TreePanel,
    pub input_panel: InputPanel,
    pub liveview_panel: LiveviewPanel,

    /// Which panel is currently focused?
    pub panel_focus: Focus,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            filter_panel: FilterPanel::default(),
            tree_panel: TreePanel::default(),
            input_panel: InputPanel::default(),
            liveview_panel: LiveviewPanel,
            panel_focus: Focus::default(),
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
        let indices = self.filter_panel.apply_filters(&self.notes);

        self.tree_panel.rebuild(&self.notes, &indices);

        self.filter_panel.rebuild_items();
    }
}
