use glam::IVec3;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub depth: usize,
    pub region_id: u32,
    pub position: IVec3,
}
