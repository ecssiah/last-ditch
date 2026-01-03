use std::collections::HashMap;
use crate::interface::{gpu::gpu_texture_data::GpuTextureData, texture::{texture_id::TextureID, texture_location::TextureLocation}};

pub struct AtlasSet {
    pub texture_size: f32,
    pub texture_data_vec: Vec<GpuTextureData>,
    pub texture_map: HashMap<TextureID, TextureLocation>,
}

impl AtlasSet {
    pub fn new(texture_size: f32) -> Self {
        let texture_data_vec = Vec::new();
        let texture_map = HashMap::new();

        Self {
            texture_size,
            texture_data_vec,
            texture_map,
        }
    }
}