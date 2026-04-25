use ratatui::{
    Frame,
    crossterm::event::KeyEvent,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

use crate::{app::render_context::RenderContext, message::Message, panels::Focus};

#[derive(Default)]
pub struct NotesPanel {
    pub selection_index: usize,
}

impl NotesPanel {
    pub fn handle_key(&self, _key: KeyEvent) -> Message {
        Message::Noop
    }

    pub fn render(&self, f: &mut Frame, area: Rect, ctx: &RenderContext) {
        let items: Vec<String> = ctx
            .model
            .ranked_notes
            .iter()
            .map(|r| {
                let note = &ctx.model.notes[r.note_index];
                format!("{}", note.slug.to_string_lossy())
            })
            .collect();

        let border_style = if matches!(ctx.model.panel_focus, Focus::Notes) {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };

        //  Empty list early return
        if items.is_empty() {
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
        let scroll_offset = if max_visible > 0 && items.len() > max_visible {
            // Keep selected item at bottom when scrolling up
            let target_offset = self.selection_index.saturating_sub(max_visible - 1);
            let max_offset = items.len().saturating_sub(max_visible);
            target_offset.min(max_offset)
        } else {
            0
        };

        let start = scroll_offset;
        let end = (scroll_offset + max_visible).min(items.len());

        let items: Vec<ListItem> = items[start..end]
            .iter()
            .enumerate()
            .map(|(i, line)| {
                // Calculate actual index in original array
                let actual_index = start + i;
                let is_selected = actual_index == self.selection_index;

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
}
