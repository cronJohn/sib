pub mod effects;
mod keys;
pub mod run;
mod update;
mod view;

use crate::domain::note::Note;
use crate::model::Model;
use crate::panels::filter::FilterPanel;
use crate::panels::input::InputPanel;
use crate::panels::liveview::LiveviewPanel;
use crate::panels::notes::NotesPanel;

pub struct App {
    pub model: Model,
    pub input_panel: InputPanel,
    pub filter_panel: FilterPanel,
    pub notes_panel: NotesPanel,
    pub liveview_panel: LiveviewPanel,
}

impl App {
    pub fn new(notes: Vec<Note>) -> Self {
        Self {
            model: Model::new(notes),
            input_panel: InputPanel::default(),
            filter_panel: FilterPanel,
            notes_panel: NotesPanel::default(),
            liveview_panel: LiveviewPanel,
        }
    }
}
