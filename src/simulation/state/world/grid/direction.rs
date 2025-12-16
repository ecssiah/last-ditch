use ultraviolet::{IVec3, Vec3};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Direction {
    North,
    West,
    South,
    East,
    Up,
    Down,
}

impl Direction {
    pub const ALL: [Self; 6] = [
        Self::North,
        Self::West,
        Self::South,
        Self::East,
        Self::Up,
        Self::Down,
    ];

    pub fn to_ivec3(&self) -> IVec3 {
        match self {
            Self::North => IVec3::new(0, 1, 0),
            Self::West => IVec3::new(-1, 0, 0),
            Self::South => IVec3::new(0, -1, 0),
            Self::East => IVec3::new(1, 0, 0),
            Self::Up => IVec3::new(0, 0, 1),
            Self::Down => IVec3::new(0, 0, -1),
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        match self {
            Self::North => Vec3::new(0.0, 1.0, 0.0),
            Self::West => Vec3::new(-1.0, 0.0, 0.0),
            Self::South => Vec3::new(0.0, -1.0, 0.0),
            Self::East => Vec3::new(1.0, 0.0, 0.0),
            Self::Up => Vec3::new(0.0, 0.0, 1.0),
            Self::Down => Vec3::new(0.0, 0.0, -1.0),
        }
    }

    pub fn from_rotation(rotation: f32) -> Self {
        let rotation_normalized = rotation.rem_euclid(360.0);

        match rotation_normalized {
            rotation_normalized if rotation_normalized < 45.0 || rotation_normalized >= 315.0 => {
                Direction::North
            }
            rotation_normalized if rotation_normalized < 135.0 => Direction::West,
            rotation_normalized if rotation_normalized < 225.0 => Direction::South,
            rotation_normalized if rotation_normalized < 315.0 => Direction::East,
            _ => Direction::North,
        }
    }

    pub fn to_string(direction: Direction) -> String {
        match direction {
            Direction::North => String::from("North"),
            Direction::West => String::from("West"),
            Direction::South => String::from("South"),
            Direction::East => String::from("East"),
            Direction::Up => String::from("Up"),
            Direction::Down => String::from("Down"),
        }
    }
}
