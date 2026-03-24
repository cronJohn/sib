pub mod defaults;
pub mod loader;

pub use loader::load_config;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub notes_dir: PathBuf,
    pub editor: String,
}
