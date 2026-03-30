use std::collections::HashMap;

use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::{
    domain::{note::Note, selection::Selection},
    message::Message,
};

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

#[derive(Default)]
pub struct FilterPanel {
    pub items: Vec<FilterItem>,
    pub selection: Selection,

    /// Active filtering criteria
    pub criteria: FilterCriteria,
}

impl FilterPanel {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selection: Selection::new(),
            criteria: FilterCriteria::default(),
        }
    }
}

impl FilterPanel {
    /// Apply filters to notes and return matching indices
    pub fn apply_filters(&self, notes: &[Note]) -> Vec<usize> {
        notes
            .iter()
            .enumerate()
            .filter(|(_, note)| {
                let mut keep = true;

                // slug match
                if !self.criteria.slug_query.is_empty() {
                    keep &= note
                        .slug
                        .to_string_lossy()
                        .contains(&self.criteria.slug_query);
                }

                // tags (AND)
                for tag in &self.criteria.tags {
                    keep &= note.metadata.tags().contains(tag);
                }

                // metadata
                for (k, v) in &self.criteria.metadata {
                    keep &= note.metadata.get(k).map(|val| val == v).unwrap_or(false);
                }

                keep
            })
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Rebuild cached UI items from criteria
    pub fn rebuild_items(&mut self) {
        let mut items = Vec::new();

        if !self.criteria.slug_query.is_empty() {
            items.push(FilterItem::Slug);
        }

        for tag in &self.criteria.tags {
            items.push(FilterItem::Tag(tag.clone()));
        }

        for (k, v) in &self.criteria.metadata {
            items.push(FilterItem::Meta(k.clone(), v.clone()));
        }

        self.items = items;

        self.selection.clamp(self.items.len());
    }

    pub fn handle_key(&self, key: KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Up => Some(Message::FilterUp),
            KeyCode::Down => Some(Message::FilterDown),

            KeyCode::Char('d') => Some(Message::DeleteSelectedFilter),

            _ => None,
        }
    }
}
