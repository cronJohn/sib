pub mod run;
mod update;
mod view;

use crate::domain::note::Note;
use crate::model::Model;

pub struct App {
    pub model: Model,
}

impl App {
    pub fn new(notes: Vec<Note>) -> Self {
        Self {
            model: Model::new(notes),
        }
    }
}
