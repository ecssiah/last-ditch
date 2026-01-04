use crate::interface::{
    gpu::gpu_texture_data::GpuTextureData, texture::texture_location::TextureLocation,
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TextureAtlasSet {
    pub name: &'static str,
    pub texture_size: f32,
    pub gpu_texture_data_vec: Vec<GpuTextureData>,
    pub name_id_map: HashMap<String, u64>,
    pub id_location_map: HashMap<u64, TextureLocation>,
}

impl TextureAtlasSet {
    pub fn new(name: &'static str, texture_size: f32) -> Self {
        let gpu_texture_data_vec = Vec::new();
        let name_id_map = HashMap::new();
        let id_location_map = HashMap::new();

        Self {
            name,
            texture_size,
            gpu_texture_data_vec,
            name_id_map,
            id_location_map,
        }
    }

    pub fn add_atlas_texture(
        texture: &wgpu::Texture,
        texture_view: &wgpu::TextureView,
        sampler: &wgpu::Sampler,
        texture_atlas_set: &mut Self,
    ) {
        let gpu_texture_data = GpuTextureData {
            texture: texture.clone(),
            texture_view: texture_view.clone(),
            sampler: sampler.clone(),
        };

        texture_atlas_set
            .gpu_texture_data_vec
            .push(gpu_texture_data);
    }

    pub fn insert_texture_mapping(
        texture_id: u64,
        texture_name: &str,
        atlas_index: u8,
        layer_index: u8,
        texture_atlas_set: &mut Self,
    ) {
        let texture_location = TextureLocation {
            atlas_index,
            layer_index,
        };

        texture_atlas_set
            .name_id_map
            .insert(texture_name.to_string(), texture_id);

        texture_atlas_set
            .id_location_map
            .insert(texture_id, texture_location);
    }

    pub fn get_texture_indices(
        name: &String,
        texture_name_map: &HashMap<String, (u32, u32)>,
    ) -> Option<(u32, u32)> {
        texture_name_map.get(name).copied()
    }
}
