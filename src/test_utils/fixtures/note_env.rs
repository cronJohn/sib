use crate::{
    domain::note::{NoteMetadata, NoteMetadataBuilder},
    services::parse::ParseService,
};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use rand::{rngs::StdRng, seq::IndexedRandom, RngExt, SeedableRng};

pub const TEST_DIFFICULTIES: [&str; 3] = ["easy", "medium", "hard"];
pub const TEST_TAG_POOL: [&str; 3] = ["web", "pwn", "forensics"];

enum FixtureEntry {
    Structured {
        path: PathBuf,
        metadata: Option<NoteMetadata>,
        content: String,
    },
    Raw {
        path: PathBuf,
        contents: String,
    },
}

pub struct NoteEnvFixture {
    tmp_dir: TempDir,
    files: Vec<FixtureEntry>,
}

impl Default for NoteEnvFixture {
    fn default() -> Self {
        Self {
            tmp_dir: TempDir::new().expect("Temp dir should be created"),
            files: vec![],
        }
    }
}

impl NoteEnvFixture {
    pub fn file(mut self, path: &str, meta: Option<NoteMetadata>, content: &str) -> Self {
        self.files.push(FixtureEntry::Structured {
            path: PathBuf::from(path),
            metadata: meta,
            content: content.to_string(),
        });
        self
    }

    pub fn raw(mut self, path: &str, contents: &str) -> Self {
        self.files.push(FixtureEntry::Raw {
            path: PathBuf::from(path),
            contents: contents.to_string(),
        });
        self
    }

    pub fn malformed(self, path: &str) -> Self {
        self.raw(
            path,
            r#"---
difficulty: [unclosed
tags: ["oops"
---
broken content"#,
        )
    }

    /// Generates N notes with randomized metadata
    pub fn bulk_random(mut self, count: usize) -> Self {
        let mut rng = StdRng::seed_from_u64(69);

        for i in 0..count {
            let difficulty = TEST_DIFFICULTIES.choose(&mut rng).unwrap();
            let tag_count = rng.random_range(1..=3);

            let tags: Vec<&str> = TEST_TAG_POOL.sample(&mut rng, tag_count).cloned().collect();

            let metadata = NoteMetadataBuilder::default()
                .tags(tags.into_iter().map(|s| s.to_string()).collect())
                .field("difficulty", *difficulty)
                .build();

            let content = format!("# Note {}\nRandom content {}", i, rng.random::<u32>());

            let path = format!("topic_{}/note_{}.md", i % 5, i);

            self.files.push(FixtureEntry::Structured {
                path: PathBuf::from(path),
                metadata: Some(metadata),
                content,
            });
        }

        self
    }

    pub fn build(self) -> (TempDir, ParseService) {
        let base = self.tmp_dir.path();

        for entry in &self.files {
            let (path, contents) = match entry {
                FixtureEntry::Structured {
                    path,
                    metadata,
                    content,
                } => {
                    let final_content = if let Some(meta) = metadata {
                        format!("{}{}", meta.to_frontmatter(), content)
                    } else {
                        content.clone()
                    };

                    (path, final_content)
                }

                FixtureEntry::Raw { path, contents } => (path, contents.clone()),
            };

            let full_path = base.join(path);

            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            fs::write(full_path, contents).unwrap();
        }

        let service = ParseService::new(base.to_path_buf());
        (self.tmp_dir, service)
    }
}
