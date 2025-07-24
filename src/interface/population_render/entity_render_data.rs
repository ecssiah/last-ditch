use crate::{interface::mesh_data::MeshData, simulation::state::population::entity};
use glam::Mat4;
use std::sync::Arc;

pub struct EntityRenderData {
    pub entity_id: entity::ID,
    pub transform: Mat4,
    pub mesh_data_arc: Arc<MeshData>,
    pub texture_bind_group_arc: Arc<wgpu::BindGroup>,
}
