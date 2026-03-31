use crate::panels::input::InputMode;

#[derive(Debug)]
pub enum Message {
    Quit,
    CycleFocusForward,

    // Input Panel
    SwitchInputMode(InputMode),
    InputChar(char),
    InputBackspace,
    AddFilter,

    // Tree Panel
    TreeSelectionUp,
    TreeSelectionDown,
    OpenSelected,

    // Filter Panel
    DeleteSelectedFilter,
    FilterSelectionUp,
    FilterSelectionDown,

    Noop,
}
