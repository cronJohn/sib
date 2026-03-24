use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn main_layout(area: Rect) -> (Rect, Rect, Rect) {
    // vertical split: main pane vs preview pane
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(70), // top: main UI (list/filter)
            Constraint::Percentage(30), // bottom: preview
        ])
        .split(area);

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(40), // left: topics/notes
            Constraint::Percentage(60), // right: filters/metadata
        ])
        .split(chunks[0]);

    let left_pane = top_chunks[0];
    let right_pane = top_chunks[1];
    let preview_pane = chunks[1];

    (left_pane, right_pane, preview_pane)
}
