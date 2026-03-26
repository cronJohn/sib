use crate::{
    app::{
        mode::{Focus, InputMode},
        panels::filter::FilterItem,
        App,
    },
    context::Context,
    message::Message,
};

impl App {
    pub fn update(&mut self, msg: Message, _ctx: &Context) {
        match msg {
            // -----------------------------
            // INPUT PANEL
            // -----------------------------
            Message::InputChar(c) => {
                self.input_panel.add_char(c);

                if matches!(self.input_panel.mode, InputMode::Path) {
                    self.filter_panel.criteria.slug_query = self.input_panel.get_buf();
                    self.recompute_view();
                }
            }

            Message::DeleteChar => {
                self.input_panel.remove_char();

                if matches!(self.input_panel.mode, InputMode::Path) {
                    self.filter_panel.criteria.slug_query = self.input_panel.get_buf();
                    self.recompute_view();
                }
            }

            Message::SubmitInput => {
                match self.input_panel.mode {
                    InputMode::Path => {
                        // already live-updated
                    }

                    InputMode::Tag => {
                        if !self.input_panel.buffer.is_empty() {
                            self.filter_panel
                                .criteria
                                .tags
                                .push(self.input_panel.get_buf());

                            self.input_panel.clear_buf();
                            self.recompute_view();
                        }
                    }

                    InputMode::Meta => {
                        if let Some((k, v)) = self.input_panel.buffer.split_once(':') {
                            self.filter_panel
                                .criteria
                                .metadata
                                .insert(k.trim().to_string(), v.trim().to_string());

                            self.input_panel.clear_buf();
                            self.recompute_view();
                        }
                    }
                }
            }

            Message::SwitchMode(mode) => {
                self.input_panel.change_mode(mode);
                self.input_panel.clear_buf();
            }

            // -----------------------------
            // FOCUS
            // -----------------------------
            Message::CycleFocusForward => {
                self.panel_focus = match self.panel_focus {
                    Focus::Input => Focus::Tree,
                    Focus::Tree => Focus::Filters,
                    Focus::Filters => Focus::Input,
                    _ => Focus::Input,
                };
            }

            // -----------------------------
            // FILTER PANEL NAVIGATION
            // -----------------------------
            Message::FilterUp => {
                self.filter_panel
                    .selection
                    .up(self.filter_panel.items.len());
            }

            Message::FilterDown => {
                self.filter_panel
                    .selection
                    .down(self.filter_panel.items.len());
            }

            Message::DeleteSelectedFilter => {
                if let Some(item) = self
                    .filter_panel
                    .items
                    .get(self.filter_panel.selection.get())
                {
                    match item {
                        FilterItem::Slug => {
                            self.filter_panel.criteria.slug_query = "".to_owned();
                        }

                        FilterItem::Tag(tag) => {
                            self.filter_panel.criteria.tags.retain(|t| t != tag);
                        }

                        FilterItem::Meta(k, _) => {
                            self.filter_panel.criteria.metadata.remove(k);
                        }
                    }

                    self.recompute_view();
                }
            }

            // -----------------------------
            // TREE PANEL NAVIGATION
            // -----------------------------
            Message::NoteSelectionUp => {
                self.tree_panel
                    .selection
                    .move_up(&self.tree_panel.flattened_rows, |r| r.is_selectable());
            }

            Message::NoteSelectionDown => {
                self.tree_panel
                    .selection
                    .move_down(&self.tree_panel.flattened_rows, |r| r.is_selectable());
            }

            // -----------------------------
            // EXIT
            // -----------------------------
            Message::Quit => self.should_quit = true,
        }
    }
}
