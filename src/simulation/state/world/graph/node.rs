use glam::IVec3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Node {
    pub position: IVec3,
    pub region_position: IVec3,
    pub depth: usize,
}

impl Node {
    pub fn new(position: IVec3, region_position: IVec3, depth: usize) -> Self {
        Self {
            position,
            region_position,
            depth,
        }
    }
}
