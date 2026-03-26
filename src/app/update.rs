use crate::{
    app::{
        filter::FilterItem,
        mode::{Focus, InputMode},
        App,
    },
    context::Context,
    message::Message,
};

impl App {
    pub fn update(&mut self, msg: Message, _ctx: &Context) {
        match msg {
            Message::InputChar(c) => {
                self.input_buffer.push(c);

                // live filtering ONLY for Path mode
                if matches!(self.input_mode, InputMode::Path) {
                    self.filter.slug_query = self.input_buffer.clone();
                    self.recompute_view();
                }
            }

            Message::DeleteChar => {
                self.input_buffer.pop();

                // live filtering ONLY for Path mode
                if matches!(self.input_mode, InputMode::Path) {
                    self.filter.slug_query = self.input_buffer.clone();
                    self.recompute_view();
                }
            }

            Message::SubmitInput => {
                match self.input_mode {
                    InputMode::Path => {
                        // already handled live
                    }

                    InputMode::Tag => {
                        if !self.input_buffer.is_empty() {
                            self.filter.tags.push(self.input_buffer.clone());
                            self.input_buffer.clear();
                            self.recompute_view();
                        }
                    }

                    InputMode::Meta => {
                        if let Some((k, v)) = self.input_buffer.split_once(':') {
                            self.filter
                                .metadata
                                .insert(k.trim().to_string(), v.trim().to_string());
                            self.input_buffer.clear();
                            self.recompute_view();
                        }
                    }
                }
            }

            Message::SwitchMode(mode) => {
                self.input_mode = mode;
                self.input_buffer.clear();
            }

            Message::CycleFocusForward => {
                self.panel_focus = match self.panel_focus {
                    Focus::Input => Focus::Notes,
                    Focus::Notes => Focus::Filters,
                    Focus::Filters => Focus::Input,
                };
            }

            Message::FilterUp => {
                self.selected_filter_item
                    .move_up(&self.filter_items, |_| true);
            }

            Message::FilterDown => {
                self.selected_filter_item
                    .move_down(&self.filter_items, |_| true);
            }

            Message::DeleteSelectedFilter => {
                if let Some(item) = self.filter_items.get(self.selected_filter_item.get()) {
                    match item {
                        FilterItem::Slug => self.filter.slug_query.clear(),
                        FilterItem::Tag(tag) => {
                            self.filter.tags.retain(|t| t != tag);
                        }
                        FilterItem::Meta(k, _) => {
                            self.filter.metadata.remove(k);
                        }
                    }

                    self.recompute_view();

                    self.selected_filter_item.clamp(self.filter_items.len());
                }
            }

            Message::NoteSelectionUp => {
                self.selected_note_item
                    .move_up(&self.flattened_rows, |r| r.is_selectable());
            }

            Message::NoteSelectionDown => {
                self.selected_note_item
                    .move_down(&self.flattened_rows, |r| r.is_selectable());
            }
            Message::Quit => self.should_quit = true,
        }
    }
}
