use glam::IVec3;

#[derive(Clone, Copy, Debug)]
pub struct Transition {
    pub region1_position: IVec3,
    pub region2_position: IVec3,
}
