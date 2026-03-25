#[derive(Debug)]
pub enum Message {
    Quit,
    InputChar(char),
    DeleteChar,
    NoteSelectionUp,
    NoteSelectionDown,
}
