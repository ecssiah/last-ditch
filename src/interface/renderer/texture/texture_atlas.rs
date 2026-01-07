use crate::interface::{
    gpu::gpu_texture_data::GpuTextureData,
    renderer::texture::texture_layer_index::TextureLayerIndex,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TextureAtlas {
    pub gpu_texture_data: GpuTextureData,
    pub name_layer_index_map: HashMap<String, TextureLayerIndex>,
}

impl TextureAtlas {
    pub fn new(
        gpu_texture_data: GpuTextureData,
        name_layer_index_map: HashMap<String, TextureLayerIndex>,
    ) -> Self {
        Self {
            gpu_texture_data,
            name_layer_index_map,
        }
    }
}
