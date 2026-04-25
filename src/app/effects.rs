use std::io::Stdout;

use ratatui::{Terminal, prelude::CrosstermBackend};
use tracing::info;

use crate::{
    app::{
        App,
        run::{resume_tui, suspend_tui},
    },
    context::Context,
    effect::Effect,
};

impl App {
    pub fn run_effects(
        &mut self,
        ctx: &mut Context,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> std::io::Result<()> {
        use Effect::*;

        while let Some(effect) = self.model.pending_effects.pop() {
            match effect {
                OpenEditor(note) => {
                    info!(
                        note_slug = ?note.slug,
                        "Opening editor"
                    );
                    suspend_tui();

                    if ctx.editor.open(&note).is_ok() {
                        ctx.ranker.record_open(&note);
                    }

                    resume_tui();
                    terminal.clear()?;
                    terminal.draw(|f| self.render(f))?;
                }
            }
        }
        Ok(())
    }
}
