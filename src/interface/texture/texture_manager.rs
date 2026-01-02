use crate::interface::{
    gpu::{gpu_context::GPUContext, gpu_texture_data::GpuTextureData},
    texture::{texture_id::TextureID, texture_location::TextureLocation},
};
use std::collections::HashMap;

pub struct TextureManager {
    pub texture_atlas_vec: Vec<GpuTextureData>,
    pub texture_map: HashMap<TextureID, TextureLocation>,
    pub depth_texture: wgpu::Texture,
    pub depth_texture_view: wgpu::TextureView,
}

impl TextureManager {
    pub fn new(gpu_context: &GPUContext) -> Self {
        let texture_atlas_vec = Vec::new();
        let texture_map = HashMap::new();

        let depth_texture = Self::create_depth_texture(gpu_context);
        let depth_texture_view = depth_texture.create_view(&Default::default());

        Self {
            texture_atlas_vec,
            texture_map,
            depth_texture,
            depth_texture_view,
        }
    }

    pub fn get_surface_texture(gpu_context: &GPUContext) -> wgpu::SurfaceTexture {
        let surface_texture = gpu_context
            .surface
            .get_current_texture()
            .expect("failed to acquire next swapchain texture");

        surface_texture
    }

    pub fn create_depth_texture(gpu_context: &GPUContext) -> wgpu::Texture {
        let depth_texture_descriptor = wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: gpu_context.surface_config.width,
                height: gpu_context.surface_config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        };

        gpu_context.device.create_texture(&depth_texture_descriptor)
    }
}
