use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Note {
    pub path: PathBuf,
    pub tags: Vec<String>,
    pub metadata: Option<Metadata>,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub difficulty: Option<String>,
    pub target: Option<String>,
}
