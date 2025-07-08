use glam::IVec3;

#[derive(Clone, Copy, Debug)]
pub struct Transition {
    pub region1_position: IVec3,
    pub region2_position: IVec3,
}

impl Transition {
    pub fn new(region1_position: IVec3, region2_position: IVec3) -> Self {
        Self {
            region1_position,
            region2_position,
        }
    }
}
