use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::{app::App, domain::tree::Row};

/// Renders the notes sidebar as a tree-like list.
/// Directories are visual only; only notes are selectable.
pub fn render_notes_widget(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = app
        .flattened_rows
        .iter()
        .map(|row| match row {
            Row::Directory { name, depth } => {
                // indent directories and prepend folder icon
                let indent = "  ".repeat(*depth);
                ListItem::new(format!("{}📁 {}", indent, name))
            }
            Row::Note { name, depth, .. } => {
                // indent notes
                let indent = "  ".repeat(*depth);
                ListItem::new(format!("{}{}", indent, name))
            }
        })
        .collect();

    // Highlight the currently selected note
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
