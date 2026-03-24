use std::{fs, path::PathBuf};

use color_eyre::eyre::{eyre, Result};
use tracing::info;

use crate::config::{
    defaults::{DEFAULT_EDITOR, DEFAULT_NOTES_DIR},
    Config,
};

pub fn load_config() -> Result<Config> {
    let dir = dirs::config_dir()
        .ok_or_else(|| eyre!("Could not find config directory"))?
        .join("sib");
    let path = dir.join("config.toml");

    if !path.exists() {
        info!("Config not found at {:?}, creating default...", path);

        fs::create_dir_all(&dir)?;

        let default = Config {
            notes_dir: PathBuf::from(DEFAULT_NOTES_DIR),
            editor: DEFAULT_EDITOR.into(),
        };

        fs::write(&path, toml::to_string_pretty(&default)?)?;

        info!("Default config written to {:?}", path);
        return Ok(default);
    }

    info!("Loading config from {:?}", path);
    let contents = fs::read_to_string(&path)?;
    let config: Config = toml::from_str(&contents)?;
    info!("Config loaded successfully");

    Ok(config)
}
