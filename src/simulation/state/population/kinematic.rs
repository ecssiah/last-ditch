use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Kinematic {
    pub speed: f32,
    pub velocity: Vec3,
    pub jump_speed: f32,
    pub flying: bool,
}

impl Kinematic {
    pub fn new() -> Self {
        Self {
            speed: 6.0,
            jump_speed: 16.0,
            velocity: Vec3::broadcast(0.0),
            flying: false,
        }
    }

    pub fn set_flying(flying: bool, kinematic: &mut Self) {
        kinematic.flying = flying;
    }

    pub fn toggle_flying(kinematic: &mut Self) {
        Self::set_flying(!kinematic.flying, kinematic);
    }
}
