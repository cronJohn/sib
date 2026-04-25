use std::fs;

use color_eyre::eyre::{Result, WrapErr};
use tracing::info;

use crate::config::{Config, RawConfig, defaults::CONFIG_FILE, setup::initialize_paths};

/// Loads the config, creates the config file if missing, and ensures all directories exist.
pub fn load_config() -> Result<Config> {
    let default_config = Config::default();

    initialize_paths(&default_config).wrap_err("Failed to initialize required paths")?;

    let contents = match fs::read_to_string(&*CONFIG_FILE) {
        Ok(c) => c,
        Err(err) => return Err(err.into()),
    };

    let raw: RawConfig = toml::from_str(&contents).unwrap_or(RawConfig {
        base_notes_dir: None,
        usage_file: None,
        editor: None,
        glyph_mode: None,
    });

    let config = Config::from_raw(raw);
    info!(
        base_notes_dir = ?config.base_notes_dir,
        usage_file = ?config.usage_file,
        editor = ?config.editor,
        glyph_mode = ?config.glyph_mode,
        "Successfully loaded config"
    );
    Ok(config)
}
