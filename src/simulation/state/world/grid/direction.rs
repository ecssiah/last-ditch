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
    pub const ALL: [Direction; 6] = [
        Direction::East,
        Direction::West,
        Direction::North,
        Direction::South,
        Direction::Up,
        Direction::Down,
    ];

    pub fn to_ivec3(&self) -> IVec3 {
        match self {
            Direction::East => IVec3::new(1, 0, 0),
            Direction::West => IVec3::new(-1, 0, 0),
            Direction::North => IVec3::new(0, 1, 0),
            Direction::South => IVec3::new(0, -1, 0),
            Direction::Up => IVec3::new(0, 0, 1),
            Direction::Down => IVec3::new(0, 0, -1),
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        match self {
            Direction::East => Vec3::new(1.0, 0.0, 0.0),
            Direction::West => Vec3::new(-1.0, 0.0, 0.0),
            Direction::North => Vec3::new(0.0, 1.0, 0.0),
            Direction::South => Vec3::new(0.0, -1.0, 0.0),
            Direction::Up => Vec3::new(0.0, 0.0, 1.0),
            Direction::Down => Vec3::new(0.0, 0.0, -1.0),
        }
    }
}
