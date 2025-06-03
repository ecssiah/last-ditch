use glam::IVec3;

#[derive(Clone, Debug)]
pub struct Edge {
    pub target: IVec3,
    pub clearance: i32,
    pub cost: f32,
}
