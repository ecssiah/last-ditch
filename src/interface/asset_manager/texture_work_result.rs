use crate::interface::asset_manager::texture_data::TextureData;

#[derive(Default)]
pub struct TextureWorkResult {
    pub block_texture_data_vec: Vec<TextureData>,
    pub person_texture_data_vec: Vec<TextureData>,
}
