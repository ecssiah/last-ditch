use crate::interface::{camera::Camera, gpu_context::GPUContext};

pub struct ItemRender {}

impl ItemRender {
    pub fn new(_gpu_context: &GPUContext, _camera: &Camera) -> Self {
        Self {}
    }

    pub fn render(
        _gpu_context: &GPUContext,
        _surface_texture_view: &wgpu::TextureView,
        _depth_texture_view: &wgpu::TextureView,
        _camera_uniform_bind_group: &wgpu::BindGroup,
        _item_render: &ItemRender,
        _encoder: &mut wgpu::CommandEncoder,
    ) {
    }
}
