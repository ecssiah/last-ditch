use ultraviolet::Vec3;

use crate::simulation::state::world::grid::Direction;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub const ALL: [Self; 3] = [Self::X, Self::Y, Self::Z];

    pub fn index(axis: Self) -> usize {
        axis as usize
    }

    pub fn unit(axis: Self) -> Vec3 {
        match axis {
            Self::X => Vec3::unit_x(),
            Self::Y => Vec3::unit_y(),
            Self::Z => Vec3::unit_z(),
        }
    }

    pub fn to_direction(axis: Self) -> Direction {
        match axis {
            Self::X => Direction::East,
            Self::Y => Direction::North,
            Self::Z => Direction::Up,
        }
    }
}
