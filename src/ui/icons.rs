use std::collections::HashMap;

use crate::ui::glyphs::GlyphMode;

pub struct IconMap {
    map: HashMap<&'static str, &'static str>,
}

impl IconMap {
    pub fn new(mode: &GlyphMode) -> Self {
        let mut map = HashMap::new();

        use GlyphMode::*;
        match mode {
            Unicode => {
                map.insert("rust", "🦀");
            }
            Nerd => {
                map.insert("rust", "\u{e7a8}");
            }
        }

        Self { map }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).copied()
    }
}
