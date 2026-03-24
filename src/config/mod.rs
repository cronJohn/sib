pub mod defaults;
pub mod loader;
pub mod setup;

pub use loader::load_config;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub notes_dir: PathBuf,
    pub editor: String,
}

impl Config {
    pub fn required_config_dirs(&self) -> Vec<(&str, &PathBuf)> {
        vec![("notes dir", &self.notes_dir)]
    }
}
