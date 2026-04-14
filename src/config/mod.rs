pub mod defaults;
pub mod loader;
pub mod setup;

pub use loader::load_config;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::config::defaults::{DEFAULT_BASE_NOTES_DIR, DEFAULT_EDITOR, DEFAULT_USAGE_FILE};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub base_notes_dir: PathBuf,
    #[serde(default)]
    pub usage_file: PathBuf,
    #[serde(default)]
    pub editor: String,
}

impl Default for Config {
    fn default() -> Self {
        let config_dir = Self::get_config_dir();

        Self {
            base_notes_dir: PathBuf::from(DEFAULT_BASE_NOTES_DIR),
            usage_file: config_dir.join(DEFAULT_USAGE_FILE),
            editor: String::from(DEFAULT_EDITOR),
        }
    }
}

impl Config {
    /// Returns the application's config directory path
    pub fn get_config_dir() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sib")
    }

    /// Returns the config file path
    pub fn get_config_file_path() -> PathBuf {
        Self::get_config_dir().join("config.toml")
    }

    /// Returns all required paths that need to be initialized
    pub fn required_paths(&self) -> Vec<RequiredPath> {
        vec![
            RequiredPath::Dir("config dir", Self::get_config_dir()),
            RequiredPath::File("config file", Self::get_config_file_path()),
            RequiredPath::Dir("notes dir", self.base_notes_dir.clone()),
            RequiredPath::File("usage file", self.usage_file.clone()),
        ]
    }
}

pub enum RequiredPath {
    Dir(&'static str, PathBuf),
    File(&'static str, PathBuf),
}
