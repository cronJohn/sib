/// Which input mode are we currently in?
/// Path - entering path slug to filter by
/// Tag - entering tags to filter by
/// Meta - entering `key: value` to filter by
#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Path,
    Tag,
    Meta,
}

/// Which TUI panel are we currently focusing?
#[derive(Default)]
pub enum Focus {
    #[default]
    Input,
    Tree,
    Filters,
    Liveview,
}
