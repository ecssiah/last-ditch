use glam::IVec3;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub position: IVec3,
    pub region_id: u32,
    pub depth: usize,
}

impl Node {
    pub fn new(position: IVec3, region_id: u32, depth: usize) -> Self {
        Self {
            position,
            region_id,
            depth,
        }
    }
}
