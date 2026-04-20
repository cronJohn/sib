use color_eyre::Result;
use std::fs;
use std::path::Path;
use tracing::info;

use crate::config::{
    defaults::{CONFIG_DIR, CONFIG_FILE},
    Config,
};

/// Ensure all directories in the config exist, creating them if needed.
pub fn initialize_paths(config: &Config) -> Result<()> {
    let paths_to_init: Vec<RequiredPath> = vec![
        RequiredPath::Dir("config dir", &CONFIG_DIR),
        RequiredPath::File("config file", &CONFIG_FILE),
        RequiredPath::Dir("notes dir", &config.base_notes_dir),
        RequiredPath::File("usage file", &config.usage_file),
    ];

    for required in paths_to_init {
        match required {
            RequiredPath::Dir(name, path) => {
                if !path.exists() {
                    info!("{} at {:?} missing. Creating directory...", name, path);
                    fs::create_dir_all(path)?;
                }
            }
            RequiredPath::File(name, path) => {
                if !path.exists() {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    info!("{} at {:?} missing. Creating file...", name, path);
                    fs::File::create(path)?;
                }
            }
        }
    }

    Ok(())
}

pub enum RequiredPath<'a> {
    Dir(&'static str, &'a Path),
    File(&'static str, &'a Path),
}
