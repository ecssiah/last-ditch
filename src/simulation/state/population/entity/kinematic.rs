use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Kinematic {
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

impl Kinematic {
    pub fn new() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
        }
    }
}
