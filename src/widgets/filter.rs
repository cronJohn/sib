use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::domain::tokenizer::Token;

pub struct FilterWidgetOptions<'a> {
    pub items: &'a [Token],
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
                Token::Text(s) => ListItem::new(format!("Path: {}", s)),
                Token::Tag(t) => ListItem::new(format!("Tag: {}", t)),
                Token::Meta { key, value } => ListItem::new(format!("{}: {}", key, value)),
            })
            .collect()
    };

    let list = List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Filters")
            .border_style(border_style),
    );

    f.render_widget(list, area);
}
