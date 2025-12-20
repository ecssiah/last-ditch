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
            velocity: Vec3::zero(),
            flying: false,
        }
    }

    pub fn set_flying(flying: bool, kinematic: &mut Self) {
        if flying {
            kinematic.flying = true;
            kinematic.speed = 12.0;
        } else {
            kinematic.flying = false;
            kinematic.speed = 6.0;
        }
    }

    pub fn toggle_flying(kinematic: &mut Self) {
        Self::set_flying(!kinematic.flying, kinematic);
    }
}

impl Default for Kinematic {
    fn default() -> Self {
        Self::new()
    }
}
