use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::App;

/// Widget to render all Markdown notes
pub fn render_notes_widget(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .notes
        .iter()
        .map(|note| {
            let name = note.slug.as_os_str().to_string_lossy();
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
