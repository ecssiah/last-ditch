use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Kinematic {
    pub speed: f32,
    pub velocity: Vec3,
    pub jump_speed: f32,
}

impl Kinematic {
    pub fn new() -> Self {
        Self {
            speed: 6.0,
            jump_speed: 16.0,
            velocity: Vec3::broadcast(0.0),
        }
    }
}
