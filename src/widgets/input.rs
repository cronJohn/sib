use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct InputWidgetOptions<T: Into<String>> {
    /// Input buffer to show
    pub buffer: T,
    /// Is this widget focused?
    /// Currently changes the border color
    pub is_focused: bool,
}

/// Widget to let user input string to filter notes
pub fn render_input_widget<T: Into<String>>(
    f: &mut Frame,
    area: Rect,
    options: InputWidgetOptions<T>,
) {
    let border_style = if options.is_focused {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default()
    };

    let text = options.buffer.into();

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Input")
            .border_style(border_style),
    );

    f.render_widget(paragraph, area);
}
