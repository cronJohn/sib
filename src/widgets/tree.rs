use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::domain::tree::TreeItem;

pub struct TreeWidgetOptions<'a> {
    pub items: &'a [TreeItem],
    pub selection_index: usize,
    pub is_focused: bool,
}

/// Renders the notes sidebar as a tree-like list.
/// Directories are visual only; only notes are selectable.
pub fn render_tree_widget(f: &mut Frame, area: Rect, options: TreeWidgetOptions) {
    // Border style depends on focus
    let border_style = if options.is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    // Map TreeItem to ListItem with indentation and folder icon for directories
    let list_items: Vec<ListItem> = options
        .items
        .iter()
        .map(|item| {
            let indent = "  ".repeat(item.depth);
            let text = if item.selectable {
                // Note (leaf)
                format!("{}{}", indent, item.display_name)
            } else {
                // Directory
                format!("{}📁 {}", indent, item.display_name)
            };
            ListItem::new(text)
        })
        .collect();

    // Stateful selection
    let mut state = ListState::default();
    if !options.items.is_empty() {
        state.select(Some(options.selection_index));
    }

    let list = List::new(list_items)
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
