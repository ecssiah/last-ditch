use crate::interface::texture::texture_load_work::TextureLoadWork;

#[derive(Clone)]
pub enum TextureLoadResult {
    Ok(TextureLoadWork),
}
