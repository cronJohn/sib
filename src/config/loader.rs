use std::fs;

use color_eyre::eyre::{Result, WrapErr};
use tracing::{info, warn};

use crate::config::{setup::initialize_paths, Config};

/// Loads the config, creates the config file if missing, and ensures all directories exist.
pub fn load_config() -> Result<Config> {
    let config_file_path = Config::get_config_file_path();
    let default_config = Config::default();

    initialize_paths(&default_config).wrap_err("Failed to initialize required paths")?;

    let config = fs::read_to_string(&config_file_path)
        .ok()
        .and_then(|contents| toml::from_str(&contents).ok())
        .unwrap_or_else(|| {
            warn!("Config file missing or invalid. Using defaults.");
            let _ = write_config_file(&config_file_path, &default_config);
            default_config
        });

    info!("Config loaded and all paths initialized");
    Ok(config)
}

fn write_config_file(path: &std::path::Path, config: &Config) -> Result<()> {
    let toml_content = toml::to_string_pretty(config).wrap_err("Failed to serialize config")?;
    fs::write(path, toml_content)
        .wrap_err_with(|| format!("Failed to write config to {:?}", path))?;
    Ok(())
}
