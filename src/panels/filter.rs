use ratatui::{
    crossterm::event::KeyEvent,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::{
    app::render_context::RenderContext, domain::tokenizer::Token, message::Message, panels::Focus,
};

pub struct FilterPanel;

impl FilterPanel {
    pub fn handle_key(&self, _key: KeyEvent) -> Message {
        Message::Noop
    }

    pub fn render(&self, f: &mut Frame, area: Rect, ctx: &RenderContext) {
        let border_style = if matches!(ctx.model.panel_focus, Focus::Filter) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        let list_items: Vec<ListItem> = if ctx.model.token_filters.is_empty() {
            vec![ListItem::new("No filters")]
        } else {
            ctx.model
                .token_filters
                .iter()
                .map(|item| match item {
                    Token::Text(s) => ListItem::new(ctx.renderer.render_path(s)),
                    Token::Tag(t) => ListItem::new(ctx.renderer.render_tag(t)),
                    Token::Meta { key, value } => ListItem::new(ctx.renderer.render_kv(key, value)),
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
}
