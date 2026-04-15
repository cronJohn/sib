use tracing::error;

use crate::app::App;
use crate::context::Context;
use crate::domain::tokenizer::parse_query;
use crate::message::Message;

impl App {
    pub fn update(&mut self, msg: Message, ctx: &mut Context) {
        use Message::*;

        match msg {
            // Global events
            Quit => self.model.should_quit = true,
            CycleFocusForward => {
                self.model.panel_focus = self.model.panel_focus.next();
            }

            // Input panel events
            InputChar(c) => {
                self.input_panel.buffer.push(c);
                self.recompute_results(ctx);
            }
            InputBackspace => {
                self.input_panel.buffer.pop();
                self.recompute_results(ctx);
            }

            // Note panel events
            NoteSelectionUp => {
                if self.notes_panel.selection_index > 0 {
                    self.notes_panel.selection_index -= 1;
                }
            }

            NoteSelectionDown => {
                let max_index = self.model.filtered_results.len().saturating_sub(1);
                if self.notes_panel.selection_index < max_index {
                    self.notes_panel.selection_index += 1;
                }
            }

            OpenSelected => {
                // Do nothing if no results
                if self.model.filtered_results.is_empty() {
                    return;
                }

                // Get the selected result item
                let result_item = &self.model.filtered_results[self.notes_panel.selection_index];

                // Get the actual note
                let note = &self.model.notes[result_item.note_index];

                // Open the file
                match ctx.editor.open(note) {
                    Ok(_) => ctx.ranker.record_open(note),
                    Err(e) => error!("Unable to open note: {e}"),
                }
            }

            Noop => {}
        }
    }

    fn recompute_results(&mut self, ctx: &Context) {
        let tokens = parse_query(&self.input_panel.buffer);
        self.model.filter_criteria.clear();
        self.model.filter_criteria.extend_from_slice(&tokens);
        self.model.filtered_results.clear();
        self.model
            .filtered_results
            .extend_from_slice(&ctx.ranker.compute_results(&self.model.notes, &tokens));
        // Reset selection to highest score item
        self.notes_panel.selection_index = self.model.filtered_results.len().saturating_sub(1);
    }
}
