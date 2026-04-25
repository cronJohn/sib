use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum GlyphMode {
    #[default]
    Unicode,
    Nerd,
}

pub struct Glyphs {
    pub tag: &'static str,
    pub path: &'static str,
}

pub const UNICODE: Glyphs = Glyphs {
    tag: "🏷 ",
    path: "📁",
};

pub const NERD: Glyphs = Glyphs {
    tag: "\u{f412} ",
    path: "\u{ea83} ",
};

impl GlyphMode {
    pub fn glyphs(&self) -> Glyphs {
        match self {
            GlyphMode::Unicode => UNICODE,
            GlyphMode::Nerd => NERD,
        }
    }
}
