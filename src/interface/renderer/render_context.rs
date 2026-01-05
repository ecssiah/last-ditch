use crate::interface::renderer::{
    render_catalog::RenderCatalog, texture::texture_manager::TextureManager,
};

pub struct RenderContext<'a> {
    pub render_catalog: &'a RenderCatalog,
    pub texture_manager: &'a TextureManager,
}
