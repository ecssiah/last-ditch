use glam::IVec3;

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub level: u32,
    pub region_coordinates: IVec3,
    pub position: IVec3,
}
