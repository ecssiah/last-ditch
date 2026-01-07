#[derive(Clone)]
pub struct GpuTextureData {
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl GpuTextureData {
    pub fn new(
        texture: wgpu::Texture,
        texture_view: wgpu::TextureView,
        sampler: wgpu::Sampler,
    ) -> Self {
        Self {
            texture,
            texture_view,
            sampler,
        }
    }
}
