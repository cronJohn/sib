pub mod defaults;
pub mod loader;
pub mod setup;

pub use loader::load_config;
use tracing::warn;

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::ui::glyphs::GlyphMode;

#[derive(Debug)]
pub struct Config {
    pub base_notes_dir: PathBuf,
    pub usage_file: PathBuf,
    pub editor: String,
    pub glyph_mode: GlyphMode,
}

#[derive(Deserialize)]
pub struct RawConfig {
    pub base_notes_dir: Option<PathBuf>,
    pub usage_file: Option<PathBuf>,
    pub editor: Option<String>,
    pub glyph_mode: Option<GlyphMode>,
}

impl Config {
    pub fn from_raw(raw: RawConfig) -> Self {
        Self {
            base_notes_dir: raw
                .base_notes_dir
                .map(|p| normalize_path(&p))
                .unwrap_or_else(|| {
                    warn!("base notes directory missing, using default");
                    Self::default_base_notes_dir()
                }),

            usage_file: raw
                .usage_file
                .map(|p| normalize_path(&p))
                .unwrap_or_else(|| {
                    warn!("usage file missing, using default");
                    Self::default_usage_file()
                }),

            editor: raw.editor.unwrap_or_else(|| {
                warn!("editor missing, using default");
                Self::default_editor()
            }),

            glyph_mode: raw.glyph_mode.unwrap_or_else(|| {
                warn!("glyph_mode missing, using default");
                Self::default_glyph_mode()
            }),
        }
    }
}

pub fn normalize_path(input: &Path) -> PathBuf {
    // Expand ~
    match input.strip_prefix("~") {
        Ok(stripped) => dirs::home_dir()
            .map(|h| h.join(stripped))
            .unwrap_or_else(|| input.to_path_buf()),
        Err(_) => input.to_path_buf(),
    }
}
