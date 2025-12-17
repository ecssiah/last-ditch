use ultraviolet::{Rotor3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub world_position: Vec3,
    pub rotation_xy: f32,
    pub rotor: Rotor3,
    pub size: Vec3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            world_position: Vec3::default(),
            rotation_xy: 0.0,
            size: Vec3::default(),
            rotor: Rotor3::default(),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn set_world_position(world_position: Vec3, transform: &mut Self) {
        transform.world_position = world_position;
    }

    pub fn set_size(size: Vec3, transform: &mut Self) {
        transform.size = size;
    }

    pub fn set_rotation(rotation_xy: f32, transform: &mut Self) {
        let rotation_xy_radians = rotation_xy.to_radians();

        transform.rotation_xy = rotation_xy;
        transform.rotor = Rotor3::from_rotation_xy(rotation_xy_radians);
    }
}
