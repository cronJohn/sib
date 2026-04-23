use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::app::{render_context::RenderContext, App};

impl App {
    pub fn render(&self, f: &mut Frame) {
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

        let ctx = RenderContext {
            model: &self.model,
            renderer: &self.renderer,
        };

        // Render individual widgets
        self.input_panel.render(f, top_row, &ctx);
        self.notes_panel.render(f, left_col, &ctx);
        self.filter_panel.render(f, right_col, &ctx);
        self.liveview_panel.render(f, bottom_row, &ctx);
    }
}
