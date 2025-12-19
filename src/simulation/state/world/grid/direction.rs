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

    pub fn to_ivec3(direction: Self) -> IVec3 {
        match direction {
            Self::North => IVec3::new(0, 1, 0),
            Self::West => IVec3::new(-1, 0, 0),
            Self::South => IVec3::new(0, -1, 0),
            Self::East => IVec3::new(1, 0, 0),
            Self::Up => IVec3::new(0, 0, 1),
            Self::Down => IVec3::new(0, 0, -1),
        }
    }

    pub fn to_vec3(direction: Self) -> Vec3 {
        match direction {
            Self::North => Vec3::new(0.0, 1.0, 0.0),
            Self::West => Vec3::new(-1.0, 0.0, 0.0),
            Self::South => Vec3::new(0.0, -1.0, 0.0),
            Self::East => Vec3::new(1.0, 0.0, 0.0),
            Self::Up => Vec3::new(0.0, 0.0, 1.0),
            Self::Down => Vec3::new(0.0, 0.0, -1.0),
        }
    }

    pub fn to_rotation(direction: Direction) -> f32 {
        match direction {
            Self::North => 0.0f32.to_radians(),
            Self::East => 270.0f32.to_radians(),
            Self::South => 180.0f32.to_radians(),
            Self::West => 90.0f32.to_radians(),
            _ => 0.0,
        }
    }

    pub fn from_rotation(rotation: f32) -> Self {
        let rotation_normalized = rotation.rem_euclid(360.0);

        if rotation_normalized < 45.0 || rotation_normalized >= 315.0 {
            Self::North
        } else if rotation_normalized < 135.0 && rotation_normalized >= 45.0 {
            Self::West
        } else if rotation_normalized < 225.0 && rotation_normalized >= 135.0 {
            Self::South
        } else if rotation_normalized < 315.0 && rotation_normalized >= 225.0 {
            Self::East
        } else {
            panic!("Improper rotation value")
        }
    }

    pub fn to_opposing(direction: Self) -> Self {
        match direction {
            Self::North => Self::South,
            Self::West => Self::East,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    pub fn to_string(direction: Self) -> String {
        match direction {
            Self::North => String::from("North"),
            Self::West => String::from("West"),
            Self::South => String::from("South"),
            Self::East => String::from("East"),
            Self::Up => String::from("Up"),
            Self::Down => String::from("Down"),
        }
    }
}
