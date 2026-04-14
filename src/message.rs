#[derive(Debug)]
pub enum Message {
    Quit,
    CycleFocusForward,

    // Input Panel
    InputChar(char),
    InputBackspace,

    // Tree Panel
    NoteSelectionUp,
    NoteSelectionDown,
    OpenSelected,

    Noop,
}
