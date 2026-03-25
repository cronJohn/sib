use crate::app::App;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Split vertically into 3 rows: top 10%, middle 80%, bottom 10%
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(size);

    let top_row = vertical_chunks[0];
    let middle_row = vertical_chunks[1];
    let bottom_row = vertical_chunks[2];

    // Split middle row horizontally into 2 columns
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(middle_row);

    let left_col = middle_chunks[0];
    let right_col = middle_chunks[1];

    // Top row
    let top_widget = Paragraph::new(app.path_filter.as_str())
        .block(Block::default().borders(Borders::ALL).title("Path Input"));
    f.render_widget(top_widget, top_row);

    render_notes_widget(f, left_col, app);

    // Right column in middle row
    let right_widget = Paragraph::new("Right column text")
        .block(Block::default().borders(Borders::ALL).title("Right"));
    f.render_widget(right_widget, right_col);

    // Bottom row
    let bottom_widget = Paragraph::new("Bottom row text")
        .block(Block::default().borders(Borders::ALL).title("Bottom"));
    f.render_widget(bottom_widget, bottom_row);
}

fn render_notes_widget(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .notes
        .iter()
        .map(|note| {
            let name = note.path.as_os_str().to_str().unwrap_or_default();

            ListItem::new(name)
        })
        .collect();

    let mut state = ListState::default();
    state.select(Some(app.selected_note_entry));

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Notes"))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut state);
}
