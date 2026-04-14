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

    let border_width = 2; // top and bottom borders
    let max_visible = area.height.saturating_sub(border_width) as usize;
    let mut scroll_offset = options.scroll_offset;

    // Calculate scroll_offset to keep selected item at the bottom
    if max_visible > 0 && options.items.len() > max_visible {
        let target_offset = options.selected_index.saturating_sub(max_visible - 1);
        let max_offset = options.items.len().saturating_sub(max_visible);
        scroll_offset = target_offset.clamp(0, max_offset);
    }

    let start = scroll_offset;
    let end = (scroll_offset + max_visible).min(options.items.len());

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
