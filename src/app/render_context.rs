use crate::{model::Model, ui::renderer::Renderer};

/// A read-only snapshot of application state needed for rendering a single frame
pub struct RenderContext<'a> {
    pub model: &'a Model,
    pub renderer: &'a Renderer,
}
