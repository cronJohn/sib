use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct Note {
    pub path: PathBuf,
    pub content: String,
    pub metadata: Option<NoteMetadata>,
}

/// Yaml frontmatter metadata for Markdown notes
#[derive(Debug, Clone, Deserialize, Default)]
pub struct NoteMetadata {
    pub difficulty: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}
