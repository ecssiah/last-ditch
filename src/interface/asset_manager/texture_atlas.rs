use crate::interface::{
    asset_manager::layer_index::LayerIndex, gpu::gpu_texture_data::GpuTextureData,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TextureAtlas {
    pub gpu_texture_data: GpuTextureData,
    pub name_layer_map: HashMap<String, LayerIndex>,
}

impl TextureAtlas {
    pub fn new(
        gpu_texture_data: GpuTextureData,
        name_layer_map: HashMap<String, LayerIndex>,
    ) -> Self {
        Self {
            gpu_texture_data,
            name_layer_map,
        }
    }
}
