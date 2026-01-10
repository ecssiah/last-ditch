use crate::interface::{
    asset_manager::{
        block_texture_key::BlockTextureKey, layer_index::LayerIndex,
        person_texture_key::PersonTextureKey,
    },
    gpu::gpu_texture_data::GpuTextureData,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TextureAtlas {
    pub gpu_texture_data: GpuTextureData,
    pub block_layer_map: HashMap<BlockTextureKey, LayerIndex>,
    pub person_layer_map: HashMap<PersonTextureKey, LayerIndex>,
}

impl TextureAtlas {
    pub fn new(
        gpu_texture_data: GpuTextureData,
        block_layer_map: HashMap<BlockTextureKey, LayerIndex>,
        person_layer_map: HashMap<PersonTextureKey, LayerIndex>,
    ) -> Self {
        Self {
            gpu_texture_data,
            block_layer_map,
            person_layer_map,
        }
    }
}
