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
    pub items: Vec<String>, // precomputed lines for display
}

/// Renders the notes sidebar
pub fn render_notes_widget(f: &mut Frame, area: Rect, options: &NoteWidgetOptions) {
    let border_style = if options.is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    //  Empty list early return
    if options.items.is_empty() {
        let list = List::new(Vec::<ListItem>::new()).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title("Notes"),
        );
        f.render_widget(list, area);
        return;
    }

    let border_width = 2; // top and bottom borders
    let max_visible = area.height.saturating_sub(border_width) as usize;

    // Calculate scroll window to keep selected item visible
    let scroll_offset = if max_visible > 0 && options.items.len() > max_visible {
        // Keep selected item at bottom when scrolling up
        let target_offset = options.selected_index.saturating_sub(max_visible - 1);
        let max_offset = options.items.len().saturating_sub(max_visible);
        target_offset.min(max_offset)
    } else {
        0
    };

    let start = scroll_offset;
    let end = (scroll_offset + max_visible).min(options.items.len());

    let items: Vec<ListItem> = options.items[start..end]
        .iter()
        .enumerate()
        .map(|(i, line)| {
            // Calculate actual index in original array
            let actual_index = start + i;
            let is_selected = actual_index == options.selected_index;

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
