use crate::interface::texture::texture_data::TextureData;

#[derive(Clone)]
pub struct TextureLoadWork {
    pub world_texture_data_vec: Vec<TextureData>,
    pub population_texture_data_vec: Vec<TextureData>,
}
