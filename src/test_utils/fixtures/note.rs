use std::path::PathBuf;

use crate::{
    domain::note::{Note, NoteMetadata},
    services::parse::NoteMetadataState,
};

#[derive(Default)]
pub struct NoteFixture {
    slug: String,
    tags: Vec<String>,
    metadata: Vec<(String, String)>,
}

impl NoteFixture {
    pub fn slug(mut self, slug: &str) -> Self {
        self.slug = slug.to_owned();
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_owned());
        self
    }

    pub fn field(mut self, key: &str, value: &str) -> Self {
        self.metadata.push((key.to_owned(), value.to_owned()));
        self
    }

    pub fn build(self) -> Note {
        Note {
            slug: PathBuf::from(self.slug),
            content: String::new(),
            metadata: NoteMetadataState::Valid(NoteMetadata {
                tags: self.tags,
                extra: self.metadata.into_iter().collect(),
            }),
        }
    }
}
