use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct RotateData {
    pub person_id: u64,
    pub rotation_angles: Vec3,
}
