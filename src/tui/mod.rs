pub mod input;
pub mod render;
pub mod widgets;

use std::io;

use ratatui::{
    Terminal, crossterm::{
        event::{self, Event}, execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}
    }, prelude::CrosstermBackend
};

use crate::{app::App, context::Context};

pub fn run_tui(app: &mut App, ctx: &Context) -> color_eyre::Result<()> {
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    app.initialize(&ctx.parser);

    loop {
        terminal.draw(|f| render::draw(f, app))?;

        if let Event::Key(key) = event::read()?
            && let Some(msg) = input::handle_key(key, app)
        {
            app.update(msg, ctx);
        }

        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}
