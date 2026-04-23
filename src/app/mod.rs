pub mod effects;
mod keys;
pub mod render_context;
pub mod run;
mod update;
mod view;

use crate::config::Config;
use crate::domain::note::Note;
use crate::model::Model;
use crate::panels::filter::FilterPanel;
use crate::panels::input::InputPanel;
use crate::panels::liveview::LiveviewPanel;
use crate::panels::notes::NotesPanel;
use crate::ui::icons::IconMap;
use crate::ui::renderer::Renderer;

pub struct App {
    pub model: Model,
    pub renderer: Renderer,
    pub input_panel: InputPanel,
    pub filter_panel: FilterPanel,
    pub notes_panel: NotesPanel,
    pub liveview_panel: LiveviewPanel,
}

impl App {
    pub fn new(notes: Vec<Note>, config: Config) -> Self {
        let glyphs = config.glyph_mode.glyphs();
        let icons = IconMap::new(&config.glyph_mode);
        Self {
            model: Model::new(notes),
            renderer: Renderer { glyphs, icons },
            input_panel: InputPanel::default(),
            filter_panel: FilterPanel,
            notes_panel: NotesPanel::default(),
            liveview_panel: LiveviewPanel,
        }
    }
}
