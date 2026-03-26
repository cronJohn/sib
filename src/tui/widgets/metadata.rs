use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{filter::FilterItem, mode::Focus, App};

/// Widget to render and filter by Note metadata
pub fn render_metadata_widget(f: &mut Frame, area: Rect, app: &App) {
    let items_data = app.build_filter_items();

    let items: Vec<ListItem> = items_data
        .iter()
        .map(|item| match item {
            FilterItem::Slug => ListItem::new(format!("Path: {}", app.filter.slug_query)),
            FilterItem::Tag(tag) => ListItem::new(format!("Tag: {}", tag)),
            FilterItem::Meta(k, v) => ListItem::new(format!("{}: {}", k, v)),
        })
        .collect();

    let mut state = ListState::default();

    if matches!(app.focus, Focus::Filters) {
        app.selected_filter.apply_to_list_state(&mut state);
    }

    let border_style = if matches!(app.focus, Focus::Filters) {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Filters")
                .border_style(border_style),
        )
        .highlight_symbol(">> ")
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    f.render_stateful_widget(list, area, &mut state);
}
