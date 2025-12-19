use crate::simulation::state::world::grid::Direction;
use ultraviolet::IVec3;

#[inline]
pub fn rotate_by_direction(ivec3: IVec3, direction: Direction) -> IVec3 {
    match direction {
        Direction::North => ivec3,
        Direction::West => IVec3::new(-ivec3.y, ivec3.x, ivec3.z),
        Direction::South => IVec3::new(-ivec3.x, -ivec3.y, ivec3.z),
        Direction::East => IVec3::new(ivec3.y, -ivec3.x, ivec3.z),
        _ => ivec3,
    }
}

#[inline]
pub fn index_to_ivec3(index: usize, radius: usize) -> IVec3 {
    let index = index as i32;

    let radius = radius as i32;
    let size = 2 * radius + 1;
    let area = size * size;

    let x = index % size;
    let y = index / size % size;
    let z = index / area;

    IVec3::new(x, y, z) - IVec3::broadcast(radius as i32)
}

#[inline]
pub fn ivec3_to_index(ivec3: IVec3, radius: usize) -> usize {
    let ivec3_indexable = ivec3 + IVec3::broadcast(radius as i32);

    let radius = radius as i32;
    let size = 2 * radius + 1;
    let area = size * size;

    (ivec3_indexable.x + ivec3_indexable.y * size + ivec3_indexable.z * area) as usize
}
