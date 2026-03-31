use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::panels::filter::FilterItem;

pub struct FilterWidgetOptions<'a> {
    pub items: &'a [FilterItem],
    pub selected_index: Option<usize>,
    pub is_focused: bool,
}

/// Widget to render and filter by Note metadata
pub fn render_filter_widget(f: &mut Frame, area: Rect, options: FilterWidgetOptions) {
    let border_style = if options.is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let list_items: Vec<ListItem> = if options.items.is_empty() {
        vec![ListItem::new("No filters")]
    } else {
        options
            .items
            .iter()
            .map(|item| match item {
                FilterItem::Slug(s) => ListItem::new(format!("Path: {}", s)),
                FilterItem::Tag(t) => ListItem::new(format!("Tag: {}", t)),
                FilterItem::Meta(k, v) => ListItem::new(format!("{}: {}", k, v)),
            })
            .collect()
    };

    let mut state = ListState::default();
    if let Some(sel) = options.selected_index {
        state.select(Some(sel));
    }

    let list = List::new(list_items)
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
