use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::{model::Model, panels::Focus};

pub fn render_app(f: &mut Frame, model: &Model) {
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
    model
        .input_panel
        .render(f, top_row, matches!(model.panel_focus, Focus::Input));

    model.tree_panel.render(
        f,
        left_col,
        &model.tree_items(),
        matches!(model.panel_focus, Focus::Tree),
    );

    model.filter_panel.render(
        f,
        right_col,
        &model.build_filter_items(),
        matches!(model.panel_focus, Focus::Filter),
    );

    model
        .liveview_panel
        .render(f, bottom_row, matches!(model.panel_focus, Focus::Liveview));
}
