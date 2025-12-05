use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct MoveData {
    pub person_id: u64,
    pub move_direction: Vec3,
}
