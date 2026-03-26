use std::collections::HashMap;

use crate::app::App;

#[derive(Default)]
pub struct FilterCriteria {
    pub slug_query: String,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

pub enum FilterItem {
    Slug,
    Tag(String),
    Meta(String, String),
}

impl App {
    /// Apply current filters and return indices of notes that match
    pub fn apply_filters(&self) -> Vec<usize> {
        self.notes
            .iter()
            .enumerate()
            .filter(|(_, note)| {
                let mut keep = true;

                // fuzzy match slug (placeholder; use crate fuzzy-matcher for real)
                if !self.filter.slug_query.is_empty() {
                    keep &= note
                        .slug
                        .to_string_lossy()
                        .contains(&self.filter.slug_query);
                }

                // tag filtering (AND)
                for tag in &self.filter.tags {
                    keep &= note.metadata.tags().contains(tag);
                }

                // arbitrary metadata key:value filter
                for (k, v) in &self.filter.metadata {
                    keep &= note.metadata.get(k).map(|val| val == v).unwrap_or(false);
                }

                keep
            })
            .map(|(idx, _)| idx)
            .collect()
    }
    pub fn build_filter_items(&self) -> Vec<FilterItem> {
        let mut items = Vec::new();

        if !self.filter.slug_query.is_empty() {
            items.push(FilterItem::Slug);
        }

        for tag in &self.filter.tags {
            items.push(FilterItem::Tag(tag.clone()));
        }

        for (k, v) in &self.filter.metadata {
            items.push(FilterItem::Meta(k.clone(), v.clone()));
        }

        items
    }
}
