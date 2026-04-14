use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub struct NoteWidgetOptions {
    pub selected_index: usize,
    pub is_focused: bool,
    pub scroll_offset: usize,
    pub max_visible_items: usize,
    pub items: Vec<String>, // precomputed lines for display
}

/// Renders the notes sidebar
pub fn render_notes_widget(f: &mut Frame, area: Rect, options: &NoteWidgetOptions) {
    let border_style = if options.is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let start = options.scroll_offset;
    let end = options.items.len();

    // Reverse the slice to display from bottom to top
    let reversed_items: Vec<&String> = options.items[start..end].iter().rev().collect();

    let items: Vec<ListItem> = reversed_items
        .iter()
        .enumerate()
        .map(|(i, line)| {
            // Calculate actual index in original array (going up from bottom)
            let actual_index = end - 1 - i;
            let is_selected = actual_index == options.selected_index;

            // Cursor and highlight always show regardless of focus
            let cursor = if is_selected { " > " } else { "   " };
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(format!("{}{}", cursor, line), style))
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title("Notes"),
    );

    f.render_widget(list, area);
}
