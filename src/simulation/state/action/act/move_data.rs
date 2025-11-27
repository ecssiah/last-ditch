use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct MoveData {
    pub direction: Vec3,
    pub rotation_xy: f32,
    pub rotation_yz: f32,
}
