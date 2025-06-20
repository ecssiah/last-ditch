use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Kinematics {
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

impl Kinematics {
    pub fn new() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
        }
    }
}
