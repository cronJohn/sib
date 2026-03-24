use std::fs;

use color_eyre::eyre::{eyre, Result};
use tracing::info;

use crate::config::{
    defaults::{DEFAULT_EDITOR, DEFAULT_NOTES_DIR},
    setup::initialize_directories,
    Config,
};

/// Loads the config, creates the config file if missing, and ensures all directories exist.
pub fn load_config() -> Result<Config> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| eyre!("Cannot determine config directory"))?
        .join("sib");
    let config_path = config_dir.join("config.toml");

    // Ensure config directory exists
    if !config_dir.exists() {
        info!(
            "Config directory {:?} doesn't exist. Creating it now...",
            config_dir
        );
        fs::create_dir_all(&config_dir)?;
    }

    // Load existing or create default config
    let config: Config = if config_path.exists() {
        info!("Loading config from file {:?}", config_path);
        let contents = fs::read_to_string(&config_path)?;
        toml::from_str(&contents)?
    } else {
        info!(
            "Config file not found. Creating default at {:?}",
            config_path
        );

        let default_config = Config {
            notes_dir: dirs::home_dir()
                .ok_or_else(|| eyre!("Cannot determine home directory"))?
                .join(DEFAULT_NOTES_DIR),
            editor: DEFAULT_EDITOR.into(),
        };

        fs::write(&config_path, toml::to_string_pretty(&default_config)?)?;
        info!("Default config written to {:?}", config_path);
        default_config
    };

    // Ensure all directories exist
    initialize_directories(&config)?;

    info!("Config loaded and directories initialized");
    Ok(config)
}
