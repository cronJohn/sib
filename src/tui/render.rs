use crate::app::App;
use crate::tui::layout::main_layout;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &App) {
    let size = f.area();
    let (left, right, preview) = main_layout(size);

    // Left pane: list of notes
    let notes_text = app
        .filtered_notes
        .iter()
        .map(|n| n.path.to_string_lossy())
        .collect::<Vec<_>>()
        .join("\n");

    let notes_widget =
        Paragraph::new(notes_text).block(Block::default().borders(Borders::ALL).title("Notes"));
    f.render_widget(notes_widget, left);

    // Right pane: filters/metadata (example)
    let metadata_text = app
        .selected_note()
        .map(|n| format!("Tags: {:?}", n.clone().metadata.unwrap().tags))
        .unwrap_or_default();
    let metadata_widget = Paragraph::new(metadata_text)
        .block(Block::default().borders(Borders::ALL).title("Metadata"));
    f.render_widget(metadata_widget, right);

    // Bottom pane: preview
    let preview_text = app
        .selected_note()
        .map(|n| n.content.clone())
        .unwrap_or_default();
    let preview_widget =
        Paragraph::new(preview_text).block(Block::default().borders(Borders::ALL).title("Preview"));
    f.render_widget(preview_widget, preview);
}
