use sib::services::parse::ParseService;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use rand::{rngs::StdRng, seq::IndexedRandom, RngExt, SeedableRng};

pub const TEST_DIFFICULTIES: [&str; 3] = ["easy", "medium", "hard"];
pub const TEST_TAG_POOL: [&str; 3] = ["web", "pwn", "forensics"];

pub struct NotesFixture {
    tmp_dir: TempDir,
    files: Vec<(PathBuf, String)>,
}

impl Default for NotesFixture {
    fn default() -> Self {
        Self {
            tmp_dir: TempDir::new().expect("Temp dir should be created"),
            files: vec![],
        }
    }
}

impl NotesFixture {
    pub fn file(mut self, path: &str, contents: &str) -> Self {
        self.files.push((PathBuf::from(path), contents.to_string()));
        self
    }

    /// Creates partial YAML data with difficulty missing
    pub fn partial(self, path: &str) -> Self {
        self.file(
            path,
            r#"---
tags: ["only-tags"]
---
Partial metadata"#,
        )
    }

    /// Creates intentionally malformed YAML data
    pub fn malformed(mut self, path: &str) -> Self {
        let bad = r#"---
                difficulty: [unclosed
                tags: ["oops"
                ---
                broken content"#;

        self.files.push((PathBuf::from(path), bad.to_string()));
        self
    }

    /// Generates N notes with randomized metadata
    pub fn bulk_random(mut self, count: usize) -> Self {
        // Hardcode randomness to ensure consistency across tests
        let mut rng = StdRng::seed_from_u64(69);

        for i in 0..count {
            let difficulty = TEST_DIFFICULTIES.choose(&mut rng).unwrap();
            let tag_count = rng.random_range(1..=3);

            let tags: Vec<&str> = TEST_TAG_POOL.sample(&mut rng, tag_count).cloned().collect();

            let content = format!(
                r#"---
difficulty: "{}"
tags: {:?}
---
# Note {}
Random content {}
                "#,
                difficulty,
                tags,
                i,
                rng.random::<u32>()
            );

            let path = format!("topic_{}/note_{}.md", i % 5, i);
            self.files.push((PathBuf::from(path), content));
        }

        self
    }

    pub fn build(self) -> (TempDir, ParseService) {
        let base = self.tmp_dir.path();

        for (rel_path, contents) in &self.files {
            let full_path = base.join(rel_path);

            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            fs::write(full_path, contents).unwrap();
        }

        let service = ParseService::new(base.to_path_buf());
        (self.tmp_dir, service)
    }
}
