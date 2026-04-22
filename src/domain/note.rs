use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_yaml_ng::Value;

use crate::services::parse::NoteMetadataState;

/// Domain entity representing a Markdown file
#[derive(Debug, Clone)]
pub struct Note {
    /// Path relative to base_notes_dir
    pub slug: PathBuf,
    /// Content in the file not including frontmatter
    pub content: String,
    /// Frontmatter content state in file
    pub metadata: NoteMetadataState,
}

/// YAML frontmatter metadata for Markdown notes
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NoteMetadata {
    /// Required field that will be universally used
    #[serde(default)]
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

    /// Get a value as a string (converts if needed)
    pub fn get_as_string(&self, key: &str) -> Option<String> {
        self.extra.get(key).map(|v| match v {
            serde_yaml_ng::Value::String(s) => s.clone(),
            serde_yaml_ng::Value::Number(n) => n.to_string(),
            serde_yaml_ng::Value::Bool(b) => b.to_string(),
            serde_yaml_ng::Value::Null => "null".to_string(),
            _ => serde_yaml_ng::to_string(v).unwrap_or_default(),
        })
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

    pub fn field<T: Into<String>>(mut self, key: T, value: T) -> Self {
        self.extra.insert(key.into(), Value::String(value.into()));
        self
    }

    pub fn fields(mut self, pairs: &[(&str, &str)]) -> Self {
        for (k, v) in pairs.iter() {
            self.extra
                .insert(k.to_string(), Value::String(v.to_string()));
        }
        self
    }

    pub fn build(self) -> NoteMetadata {
        NoteMetadata {
            tags: self.tags,
            extra: self.extra,
        }
    }
}
