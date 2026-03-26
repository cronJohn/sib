#[derive(Debug, Default)]
pub enum InputMode {
    #[default]
    Path,
    Tag,
    Meta,
}

#[derive(Default)]
pub enum Focus {
    #[default]
    Input,
    Notes,
    Filters,
}
