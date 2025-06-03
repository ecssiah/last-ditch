use glam::IVec3;

#[derive(Clone, Debug)]
pub struct Edge {
    pub target_chunk_position: IVec3,
    pub from_grid_position: IVec3,
    pub to_grid_position: IVec3,
    pub clearance: u32,
    pub cost: f32,
    pub group_id: u32,
}
