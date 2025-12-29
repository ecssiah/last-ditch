use crate::simulation::state::world::grid;

pub type FaceMask = u8;

pub const EMPTY: FaceMask = 0;

pub const NORTH: FaceMask = 1 << 2;
pub const EAST: FaceMask = 1 << 0;
pub const WEST: FaceMask = 1 << 1;
pub const SOUTH: FaceMask = 1 << 3;
pub const UP: FaceMask = 1 << 4;
pub const DOWN: FaceMask = 1 << 5;

pub fn set(direction_mask: FaceMask, face_mask: &mut FaceMask) {
    *face_mask |= direction_mask;
}

pub fn has(direction_mask: FaceMask, face_mask: &FaceMask) -> bool {
    (*face_mask & direction_mask) != 0
}

pub fn from_direction(direction: grid::Direction) -> FaceMask {
    match direction {
        grid::Direction::East => EAST,
        grid::Direction::West => WEST,
        grid::Direction::North => NORTH,
        grid::Direction::South => SOUTH,
        grid::Direction::Up => UP,
        grid::Direction::Down => DOWN,
    }
}
