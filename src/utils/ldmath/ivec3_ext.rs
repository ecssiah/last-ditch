use crate::simulation::state::world::grid::Direction;
use ultraviolet::IVec3;

pub fn rotate_by_direction(ivec3: IVec3, direction: Direction) -> IVec3 {
    match direction {
        Direction::North => ivec3,
        Direction::West => IVec3::new(-ivec3.y, ivec3.x, ivec3.z),
        Direction::South => IVec3::new(-ivec3.x, -ivec3.y, ivec3.z),
        Direction::East => IVec3::new(ivec3.y, -ivec3.x, ivec3.z),
        _ => ivec3,
    }
}
