use std::io;

use color_eyre::Result;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};

use crate::{app::App, message::Message};

pub fn run_tui(app: &mut App) -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| render(f, app))?;

        // Exit condition from App state
        if app.should_quit {
            break;
        }

        // Read input
        if let Event::Key(key) = event::read()? {
            let msg = match key.code {
                KeyCode::Up => Some(Message::MoveUp),
                KeyCode::Down => Some(Message::MoveDown),
                KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),
                _ => None,
            };

            if let Some(msg) = msg {
                app.update(msg);
            }
        }
    }

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    Ok(())
}

fn render(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    // Top: list
    let items: Vec<ListItem> = app.items.iter().map(|i| ListItem::new(i.clone())).collect();

    let list = List::new(items)
        .block(Block::default().title("Items").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, chunks[0]);

    // Bottom: preview
    let preview_text = app
        .selected_item()
        .map(|s| format!("Selected: {}", s))
        .unwrap_or_else(|| "Nothing selected".into());

    let preview =
        Paragraph::new(preview_text).block(Block::default().title("Preview").borders(Borders::ALL));

    f.render_widget(preview, chunks[1]);
}
