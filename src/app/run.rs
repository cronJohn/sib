use ratatui::{
    crossterm::{
        event::{self, Event},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::CrosstermBackend,
    Terminal,
};

use crate::{
    app::{update::update, view::render_app, App},
    context::Context,
};

pub fn run_tui(mut app: App, ctx: Context) -> color_eyre::Result<()> {
    use std::io::stdout;

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Get init filtered notes
    app.model.recompute_filtered();

    loop {
        terminal.draw(|f| render_app(f, &app.model))?;

        if let Event::Key(key) = event::read()? {
            let msg = app.model.route_key(key);
            update(&mut app.model, msg, &ctx);
        }

        if app.model.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
