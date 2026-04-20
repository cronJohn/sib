pub mod constants;
pub mod score;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::{Deserialize, Serialize};
use tracing::info;

use crate::domain::{note::Note, tokenizer::Token};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStats {
    pub open_count: u32,
    /// Unix timestamp (seconds)
    pub last_opened: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct ResultItem {
    pub note_index: usize,
    pub score: i32,
}

pub struct RankerService {
    usage: HashMap<String, UsageStats>,
    usage_file: PathBuf,
}

impl Default for RankerService {
    fn default() -> Self {
        Self {
            usage: HashMap::new(),
            usage_file: PathBuf::new(),
        }
    }
}

impl RankerService {
    pub fn new(usage_path: PathBuf) -> Self {
        let usage = Self::load_usage(&usage_path);

        Self {
            usage,
            usage_file: usage_path,
        }
    }

    /// Loads UsageStats from a TOML file
    fn load_usage(path: &Path) -> HashMap<String, UsageStats> {
        if !path.exists() {
            return HashMap::new();
        }

        let data = match fs::read_to_string(path) {
            Ok(d) => d,
            Err(_) => return HashMap::new(),
        };

        toml::from_str(&data).unwrap_or_else(|_| HashMap::new())
    }

    /// Saves the service's UsageStats to its usage file
    fn save_usage(&self) {
        if let Ok(toml_str) = toml::to_string_pretty(&self.usage) {
            let _ = fs::write(&self.usage_file, &toml_str);
            info!(
                "Successfully wrote `{:?}` to usage file: {:?}",
                &toml_str, &self.usage_file
            );
        }
    }

    pub fn compute_results(&self, notes: &[Note], tokens: &[Token]) -> Vec<ResultItem> {
        let mut results: Vec<ResultItem> = notes
            .iter()
            .enumerate()
            .filter_map(|(idx, note)| {
                let usage = self.usage.get(&note.slug.to_string_lossy().to_string());
                let score = self.score(note, tokens, usage);

                if score <= 0 {
                    None
                } else {
                    Some(ResultItem {
                        note_index: idx,
                        score,
                    })
                }
            })
            .collect();

        // Sort by score ascending (lowest first, highest last)
        results.sort_by_key(|r| r.score);
        results
    }

    pub fn record_open(&mut self, note: &Note) {
        let entry = self
            .usage
            .entry(note.slug.to_string_lossy().to_string())
            .or_insert(UsageStats {
                open_count: 0,
                last_opened: None,
            });

        entry.open_count += 1;
        entry.last_opened = Some(now_ts());

        self.save_usage();
    }
}

fn now_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
