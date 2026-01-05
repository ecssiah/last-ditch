use crate::interface::{
    gpu::gpu_texture_data::GpuTextureData,
    renderer::texture::{texture_id::TextureID, texture_location::TextureLocation},
};
use std::collections::HashMap;

#[derive(Clone)]
pub struct TextureAtlasSet {
    pub gpu_texture_data_vec: Vec<GpuTextureData>,
    pub name_id_map: HashMap<String, TextureID>,
    pub id_location_map: HashMap<TextureID, TextureLocation>,
}

impl TextureAtlasSet {
    pub fn new() -> Self {
        let gpu_texture_data_vec = Vec::new();
        let name_id_map = HashMap::new();
        let id_location_map = HashMap::new();

        Self {
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
        texture_id: &TextureID,
        texture_name: &str,
        texture_location: &TextureLocation,
        texture_atlas_set: &mut Self,
    ) {
        texture_atlas_set
            .name_id_map
            .insert(texture_name.to_string(), texture_id.clone());

        texture_atlas_set
            .id_location_map
            .insert(texture_id.clone(), texture_location.clone());
    }

    pub fn get_texture_location<'a>(
        texture_name: &'static str,
        texture_atlas_set: &'a Self,
    ) -> Option<&'a TextureLocation> {
        let texture_id = texture_atlas_set.name_id_map.get(texture_name)?;

        texture_atlas_set.id_location_map.get(texture_id)
    }
}
