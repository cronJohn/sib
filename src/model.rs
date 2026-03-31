use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::domain::note::Note;
use crate::domain::tree::{TreeItem, VirtualTree};
use crate::message::Message;
use crate::panels::filter::{FilterItem, FilterPanel};
use crate::panels::input::InputPanel;
use crate::panels::liveview::LiveviewPanel;
use crate::panels::tree::TreePanel;
use crate::panels::Focus;

#[derive(Default)]
pub struct FilterCriteria {
    pub slug: Option<String>,
    pub tags: Vec<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

pub struct Model {
    pub notes: Vec<Note>,
    pub filtered_indices: Vec<usize>,
    pub filter_criteria: FilterCriteria,

    pub input_panel: InputPanel,
    pub filter_panel: FilterPanel,
    pub tree_panel: TreePanel,
    pub liveview_panel: LiveviewPanel,

    pub panel_focus: Focus,
    pub should_quit: bool,
}

impl Model {
    pub fn new(notes: Vec<Note>) -> Self {
        Self {
            notes,
            filtered_indices: vec![],
            filter_criteria: FilterCriteria::default(),
            input_panel: Default::default(),
            filter_panel: Default::default(),
            tree_panel: Default::default(),
            liveview_panel: Default::default(),
            panel_focus: Focus::Input,
            should_quit: false,
        }
    }

    pub fn route_key(&mut self, key: KeyEvent) -> Message {
        use KeyCode::*;

        // Global keybinds
        match (key.code, key.modifiers) {
            (Esc, _) => return Message::Quit,
            (Tab, _) => return Message::CycleFocusForward,
            _ => {}
        }

        // Panel specific keybinds
        match self.panel_focus {
            Focus::Input => self.input_panel.handle_key(key),
            Focus::Tree => self.tree_panel.handle_key(key),
            Focus::Filter => self.filter_panel.handle_key(key),
            Focus::Liveview => self.liveview_panel.handle_key(key),
        }
    }

    pub fn get_selected_note(&self) -> Option<&Note> {
        self.filtered_indices
            .get(self.tree_panel.selection_index.get())
            .map(|&i| &self.notes[i])
    }

    pub fn build_filter_items(&self) -> Vec<FilterItem> {
        let mut items = Vec::new();

        if let Some(slug) = &self.filter_criteria.slug {
            items.push(FilterItem::Slug(slug.clone()));
        }

        for tag in &self.filter_criteria.tags {
            items.push(FilterItem::Tag(tag.clone()));
        }

        for (k, v) in &self.filter_criteria.metadata {
            items.push(FilterItem::Meta(k.clone(), v.clone()));
        }

        items
    }

    pub fn recompute_filtered(&mut self) {
        self.filtered_indices = self
            .notes
            .iter()
            .enumerate()
            .filter(|(_, note)| {
                let mut keep = true;
                if let Some(slug) = &self.filter_criteria.slug {
                    keep &= note.slug.to_str().unwrap().contains(slug);
                }
                for tag in &self.filter_criteria.tags {
                    keep &= note.metadata.tags().contains(tag);
                }
                for (k, v) in &self.filter_criteria.metadata {
                    keep &= note.metadata.get(k).map(|val| val == v).unwrap_or(false);
                }
                keep
            })
            .map(|(idx, _)| idx)
            .collect();
    }

    pub fn tree_items(&self) -> Vec<TreeItem> {
        let root = VirtualTree::build_from_notes(&self.notes, &self.filtered_indices);
        VirtualTree::flatten(&root)
    }

    pub fn tree_move_up(&mut self) {
        let items = self.tree_items();

        self.tree_panel
            .selection_index
            .move_up(&items, |item| item.is_selectable());
    }

    pub fn tree_move_down(&mut self) {
        let items = self.tree_items();

        self.tree_panel
            .selection_index
            .move_down(&items, |item| item.is_selectable());
    }
}
