use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::services::parse::NoteMetadataState;

/// Domain entity representing a Markdown file
#[derive(Debug, Clone)]
pub struct Note {
    /// Path to Markdown file
    pub path: PathBuf,
    /// Content in the file not including frontmatter
    pub content: String,
    /// Frontmatter content state in file
    pub metadata: NoteMetadataState,
}

/// YAML frontmatter metadata for Markdown notes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NoteMetadata {
    /// Required field that will be universally used
    pub tags: Vec<String>,
    /// Extra metadata about the file
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

impl NoteMetadata {
    /// Serialize the NoteMetadata struct into formatted YAML frontmatter
    pub fn to_frontmatter(&self) -> String {
        let yaml = serde_yaml_ng::to_string(self).expect("Failed to serialize metadata");
        format!("---\n{}---\n", yaml)
    }
}

#[derive(Default)]
pub struct NoteMetadataBuilder {
    tags: Vec<String>,
    extra: HashMap<String, Value>,
}

impl NoteMetadataBuilder {
    pub fn new() -> Self {
        NoteMetadataBuilder::default()
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    pub fn tags<T: Into<String>>(mut self, tags: Vec<T>) -> Self {
        self.tags = tags.into_iter().map(|t| t.into()).collect();
        self
    }

    pub fn field(mut self, key: impl Into<String>, value: Value) -> Self {
        self.extra.insert(key.into(), value);
        self
    }

    // Convenience helpers (optional but practical)
    pub fn string_field(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.extra.insert(key.into(), Value::String(value.into()));
        self
    }

    pub fn build(self) -> NoteMetadata {
        NoteMetadata {
            tags: self.tags,
            extra: self.extra,
        }
    }
}
