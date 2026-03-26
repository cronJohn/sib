use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::panels::filter::{FilterItem, FilterPanel};

/// Widget to render and filter by Note metadata
pub fn render_filters_widget(f: &mut Frame, area: Rect, panel: &FilterPanel, is_focused: bool) {
    let border_style = if is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let items: Vec<ListItem> = if panel.items.is_empty() {
        vec![ListItem::new("No filters")]
    } else {
        panel
            .items
            .iter()
            .map(|item| match item {
                FilterItem::Slug => ListItem::new(format!("Path: {}", panel.criteria.slug_query)),
                FilterItem::Tag(tag) => ListItem::new(format!("Tag: {}", tag)),
                FilterItem::Meta(k, v) => ListItem::new(format!("{}: {}", k, v)),
            })
            .collect()
    };

    let mut state = ListState::default();

    if !panel.items.is_empty() && is_focused {
        state.select(Some(panel.selection.get()));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Filters")
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
