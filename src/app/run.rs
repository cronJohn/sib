use std::io::stdout;

use crate::{app::App, context::Context, message::Message};
use ratatui::{
    crossterm::{
        event::{self, Event},
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
            LeaveAlternateScreen,
        },
    },
    prelude::CrosstermBackend,
    Terminal,
};

impl App {
    pub fn run(&mut self, mut ctx: Context) -> color_eyre::Result<()> {
        use std::io::stdout;

        resume_tui();

        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        self.update(Message::Init, &mut ctx);

        loop {
            terminal.draw(|f| self.render(f))?;

            if let Event::Key(key) = event::read()? {
                let msg = self.route_key(key);
                self.update(msg, &mut ctx);
            }

            self.run_effects(&mut ctx, &mut terminal)?;

            if self.model.should_quit {
                break;
            }
        }

        suspend_tui();
        Ok(())
    }
}

pub fn resume_tui() {
    let _ = execute!(stdout(), EnterAlternateScreen);
    let _ = enable_raw_mode();
}

pub fn suspend_tui() {
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen, Clear(ClearType::All));
}
