use ultraviolet::{Rotor3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Spatial {
    pub world_position: Vec3,
    pub size: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub rotor: Rotor3,
}

impl Spatial {
    pub fn new() -> Self {
        Self {
            world_position: Vec3::default(),
            size: Vec3::default(),
            yaw: 0.0,
            pitch: 0.0,
            rotor: Rotor3::default(),
        }
    }

    pub fn forward(spatial: &Spatial) -> Vec3 {
        spatial.rotor * Vec3::unit_z()
    }

    pub fn up(spatial: &Spatial) -> Vec3 {
        spatial.rotor * Vec3::unit_y()
    }

    pub fn right(spatial: &Spatial) -> Vec3 {
        spatial.rotor * Vec3::unit_x()
    }

    pub fn eye(spatial: &Spatial) -> Vec3 {
        spatial.world_position + Self::up(spatial) * 0.9 * spatial.size.y
    }
}
