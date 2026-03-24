use std::process::Command;

pub struct EditorService {
    editor: String,
}

impl EditorService {
    pub fn new(editor: String) -> Self {
        Self { editor }
    }

    pub fn open(&self, path: &str) -> std::io::Result<()> {
        Command::new(&self.editor).arg(path).status()?;
        Ok(())
    }
}
