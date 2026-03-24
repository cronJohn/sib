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
        let mut lines = raw.lines();

        let (metadata, content) = if lines.next() == Some("---") {
            let mut fm_lines = Vec::new();
            let mut content_lines = Vec::new();
            let mut in_frontmatter = true;

            for line in lines {
                if in_frontmatter {
                    if line.trim() == "---" {
                        in_frontmatter = false;
                        continue;
                    }
                    fm_lines.push(line);
                } else {
                    content_lines.push(line);
                }
            }

            let metadata = if !fm_lines.is_empty() {
                let fm_str = fm_lines.join("\n");
                match serde_yaml_ng::from_str::<NoteMetadata>(&fm_str) {
                    Ok(meta) => Some(meta),
                    Err(err) => {
                        warn!("Failed to parse frontmatter in {:?}: {:?}", path, err);
                        None
                    }
                }
            } else {
                None
            };

            (metadata, content_lines.join("\n"))
        } else {
            (None, raw)
        };

        Ok(Note {
            path: path.to_path_buf(),
            content,
            metadata,
        })
    }
}
