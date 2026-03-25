use std::{
    fs,
    path::{Path, PathBuf},
};

use rayon::prelude::*;
use tracing::warn;
use walkdir::WalkDir;

use crate::domain::note::{Note, NoteMetadata};

pub struct ParseService {
    base_note_dir: PathBuf,
}

impl ParseService {
    pub fn new(base_note_dir: PathBuf) -> Self {
        Self { base_note_dir }
    }

    /// Walks the directory recursively and collects all Markdown files in parallel
    pub fn collect_notes(&self) -> Vec<Note> {
        WalkDir::new(&self.base_note_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_type().is_file()
                    && e.path().extension().map(|ext| ext == "md").unwrap_or(false)
            })
            .par_bridge()
            .filter_map(|entry| match Self::parse_markdown_file(entry.path()) {
                Ok(note) => Some(note),
                Err(err) => {
                    warn!("Failed to read {:?}: {:?}", entry.path(), err);
                    None
                }
            })
            .collect()
    }

    /// Reads a Markdown file, extracts frontmatter + content safely
    fn parse_markdown_file(path: &Path) -> std::io::Result<Note> {
        let raw = fs::read_to_string(path)?;

        let (fm_str, content) = Self::split_markdown_regions(&raw);

        let metadata = match fm_str {
            None => NoteMetadataState::None,
            Some(ref fm) if fm.trim().is_empty() => NoteMetadataState::None,
            Some(ref fm) => {
                let state = Self::parse_frontmatter(fm);
                if let NoteMetadataState::Invalid(ref raw_fm) = state {
                    warn!("Failed to parse frontmatter in {:?}: {:?}", path, raw_fm);
                }
                state
            }
        };

        Ok(Note {
            path: path.to_path_buf(),
            content,
            metadata,
        })
    }

    /// Parses raw frontmatter into appropriate state
    fn parse_frontmatter(raw: &str) -> NoteMetadataState {
        if raw.is_empty() {
            NoteMetadataState::None
        } else {
            match serde_yaml_ng::from_str::<NoteMetadata>(raw) {
                Ok(meta) => NoteMetadataState::Valid(meta),
                Err(_) => NoteMetadataState::Invalid(raw.to_owned()),
            }
        }
    }

    /// Returns the frontmatter and content regions of raw Markdown text
    fn split_markdown_regions(raw: &str) -> (Option<String>, String) {
        let mut lines = raw.lines();

        // Must start with ---
        if lines.next().map(str::trim) != Some("---") {
            return (None, raw.to_string());
        }

        let mut fm_lines = Vec::new();
        let mut content_lines = Vec::new();
        let mut closed = false;

        for line in lines {
            if !closed {
                if line.trim() == "---" {
                    closed = true;
                } else {
                    fm_lines.push(line);
                }
            } else {
                content_lines.push(line);
            }
        }

        if !closed {
            // Unclosed frontmatter → treat entire block as invalid FM
            return (Some((raw[3..]).to_owned()), String::new());
        }

        let fm = fm_lines.join("\n");
        let content = content_lines.join("\n");

        (Some(fm), content)
    }
}

/// Represents deserializing conversion status
/// from Markdown frontmatter string
#[derive(Debug, Clone)]
pub enum NoteMetadataState {
    /// No frontmatter was specified
    None,
    /// Frontmatter was specified and is valid
    Valid(NoteMetadata),
    /// Frontmatter was specified but is improperly formatted
    Invalid(String),
}
