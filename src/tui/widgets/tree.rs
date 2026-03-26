use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::{app::panels::tree::TreePanel, domain::tree::Row};

/// Renders the notes sidebar as a tree-like list.
/// Directories are visual only; only notes are selectable.
pub fn render_tree_widget(f: &mut Frame, area: Rect, panel: &TreePanel, is_focused: bool) {
    let border_style = if is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let items: Vec<ListItem> = panel
        .flattened_rows
        .iter()
        .map(|row| match row {
            Row::Directory { name, depth } => {
                let indent = "  ".repeat(*depth);
                ListItem::new(format!("{}📁 {}", indent, name))
            }
            Row::Note { name, depth, .. } => {
                let indent = "  ".repeat(*depth);
                ListItem::new(format!("{}{}", indent, name))
            }
        })
        .collect();

    let mut state = ListState::default();

    if !panel.flattened_rows.is_empty() {
        state.select(Some(panel.selection.get()));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Notes")
                .border_style(border_style),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, &mut state);
}
