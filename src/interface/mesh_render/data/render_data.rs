use crate::interface::mesh_render::data::MeshData;
use glam::Mat4;

pub struct RenderData {
    pub mesh_data: MeshData,
    pub transform: Mat4,
    pub texture_bind_group: wgpu::BindGroup,
}
