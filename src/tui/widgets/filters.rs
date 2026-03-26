use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::{filter::FilterItem, mode::Focus, App};

/// Widget to render and filter by Note metadata
pub fn render_filters_widget(f: &mut Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = if app.filter_items.is_empty() {
        vec![ListItem::new("No filters")]
    } else {
        app.filter_items
            .iter()
            .map(|item| match item {
                FilterItem::Slug => ListItem::new(format!("Path: {}", app.filter.slug_query)),
                FilterItem::Tag(tag) => ListItem::new(format!("Tag: {}", tag)),
                FilterItem::Meta(k, v) => ListItem::new(format!("{}: {}", k, v)),
            })
            .collect()
    };

    let mut state = ListState::default();

    if matches!(app.panel_focus, Focus::Filters) {
        app.selected_filter_item.apply_to_list_state(&mut state);
    }

    let border_style = if matches!(app.panel_focus, Focus::Filters) {
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
