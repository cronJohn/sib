use color_eyre::Result;
use std::fs;
use tracing::info;

use crate::config::Config;

/// Ensure all directories in the config exist, creating them if needed.
pub fn initialize_directories(config: &Config) -> Result<()> {
    for (name, path) in config.required_config_dirs() {
        if !path.exists() {
            info!("{} directory at {:?} missing. Creating...", name, path);
            fs::create_dir_all(path)?;
        }
    }

    Ok(())
}
