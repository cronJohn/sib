use crate::app::App;
use crate::tui::widgets::*;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();

    // Split vertically into 3 rows: top 10%, middle 80%, bottom 10%
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(size);

    let top_row = vertical_chunks[0];
    let middle_row = vertical_chunks[1];
    let bottom_row = vertical_chunks[2];

    // Split middle row horizontally into 2 columns
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(middle_row);

    let left_col = middle_chunks[0];
    let right_col = middle_chunks[1];

    // Render individual widgets
    render_input_widget(f, top_row, app);
    render_notes_widget(f, left_col, app);
    render_metadata_widget(f, right_col, app);
    render_liveview_widget(f, bottom_row, app);
}
