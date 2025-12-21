pub mod mode;

pub use mode::Mode;

use crate::simulation::constants::*;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct Motion {
    pub mode: self::Mode,
    pub speed: f32,
    pub jump_speed: f32,
    pub velocity: Vec3,
}

impl Motion {
    pub fn new() -> Self {
        Self {
            mode: self::Mode::Ground,
            speed: PERSON_DEFAULT_SPEED,
            jump_speed: PERSON_DEFAULT_JUMP_SPEED,
            velocity: Vec3::zero(),
        }
    }
}

impl Default for Motion {
    fn default() -> Self {
        Self::new()
    }
}
