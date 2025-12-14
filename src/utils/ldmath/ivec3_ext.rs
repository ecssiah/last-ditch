use crate::simulation::state::world::grid::Direction;
use ultraviolet::IVec3;

pub fn rotate_by_direction(vector: IVec3, direction: Direction) -> IVec3 {
    match direction {
        Direction::North => vector,
        Direction::West => IVec3::new(-vector.y, vector.x, vector.z),
        Direction::South => IVec3::new(-vector.x, -vector.y, vector.z),
        Direction::East => IVec3::new(vector.y, -vector.x, vector.z),
        _ => vector,
    }
}
