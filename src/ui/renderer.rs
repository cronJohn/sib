use crate::ui::{glyphs::Glyphs, icons::IconMap};

pub struct Renderer {
    pub glyphs: Glyphs,
    pub icons: IconMap,
}

impl Renderer {
    pub fn render_path(&self, value: &str) -> String {
        format!("{} {}", self.glyphs.path, value)
    }

    pub fn render_tag(&self, value: &str) -> String {
        format!("{} {}", self.glyphs.tag, value)
    }

    pub fn render_kv(&self, key: &str, value: &str) -> String {
        if let Some(icon) = self.icons.get(key) {
            format!("{} {}", icon, value)
        } else {
            format!("{}: {}", key, value)
        }
    }
}
