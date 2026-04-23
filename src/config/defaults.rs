use std::{path::PathBuf, sync::LazyLock};

use crate::{config::Config, ui::glyphs::GlyphMode};

pub const DEFAULT_BASE_NOTES_DIR: &str = "notes";
pub const DEFAULT_USAGE_FILE: &str = "usage.toml";
pub const DEFAULT_EDITOR: &str = "nvim";

pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("sib")
});

pub static CONFIG_FILE: LazyLock<PathBuf> = LazyLock::new(|| CONFIG_DIR.join("config.toml"));

impl Default for Config {
    fn default() -> Self {
        Self {
            base_notes_dir: Self::default_base_notes_dir(),
            usage_file: Self::default_usage_file(),
            editor: Self::default_editor(),
            glyph_mode: Self::default_glyph_mode(),
        }
    }
}

impl Config {
    pub fn default_base_notes_dir() -> PathBuf {
        dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sib")
            .join(DEFAULT_BASE_NOTES_DIR)
    }

    pub fn default_usage_file() -> PathBuf {
        CONFIG_DIR.join(DEFAULT_USAGE_FILE)
    }

    pub fn default_editor() -> String {
        DEFAULT_EDITOR.to_string()
    }

    pub fn default_glyph_mode() -> GlyphMode {
        GlyphMode::default()
    }
}
