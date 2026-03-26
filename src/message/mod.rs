use crate::app::mode::InputMode;

#[derive(Debug)]
pub enum Message {
    // global
    Quit,
    CycleFocusForward,

    // input
    InputChar(char),
    DeleteChar,
    SubmitInput,
    SwitchMode(InputMode),

    // filters
    FilterUp,
    FilterDown,
    DeleteSelectedFilter,

    // notes
    NoteSelectionUp,
    NoteSelectionDown,
}
