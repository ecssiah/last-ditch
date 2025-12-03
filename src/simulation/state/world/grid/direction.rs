use ultraviolet::{IVec3, Vec3};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    East,
    West,
    North,
    South,
    Up,
    Down,
}

impl Direction {
    pub const ALL: [Self; 6] = [
        Self::East,
        Self::West,
        Self::North,
        Self::South,
        Self::Up,
        Self::Down,
    ];

    pub fn to_ivec3(&self) -> IVec3 {
        match self {
            Self::East => IVec3::new(1, 0, 0),
            Self::West => IVec3::new(-1, 0, 0),
            Self::North => IVec3::new(0, 1, 0),
            Self::South => IVec3::new(0, -1, 0),
            Self::Up => IVec3::new(0, 0, 1),
            Self::Down => IVec3::new(0, 0, -1),
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        match self {
            Self::East => Vec3::new(1.0, 0.0, 0.0),
            Self::West => Vec3::new(-1.0, 0.0, 0.0),
            Self::North => Vec3::new(0.0, 1.0, 0.0),
            Self::South => Vec3::new(0.0, -1.0, 0.0),
            Self::Up => Vec3::new(0.0, 0.0, 1.0),
            Self::Down => Vec3::new(0.0, 0.0, -1.0),
        }
    }
}
