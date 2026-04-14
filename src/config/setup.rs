use color_eyre::Result;
use std::fs;
use tracing::info;

use crate::config::{Config, RequiredPath};

/// Ensure all directories in the config exist, creating them if needed.
pub fn initialize_paths(config: &Config) -> Result<()> {
    for required in config.required_paths() {
        match required {
            RequiredPath::Dir(name, path) => {
                if !path.exists() {
                    info!("{} at {:?} missing. Creating directory...", name, path);
                    fs::create_dir_all(&path)?;
                }
            }
            RequiredPath::File(name, path) => {
                if !path.exists() {
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    info!("{} at {:?} missing. Creating file...", name, path);
                    fs::File::create(&path)?;
                }
            }
        }
    }

    Ok(())
}
