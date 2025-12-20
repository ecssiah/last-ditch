use crate::utils::ldmath::rotor3_ext;
use ultraviolet::{Rotor3, Vec3};

#[derive(Clone, Debug)]
pub struct Transform {
    pub world_position: Vec3,
    pub rotation_xy: f32,
    pub rotor: Rotor3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            world_position: Vec3::default(),
            rotation_xy: f32::default(),
            rotor: Rotor3::default(),
        }
    }

    pub fn set_world_position(world_position: Vec3, transform: &mut Self) {
        transform.world_position = world_position;
    }

    pub fn set_rotation(rotation_xy: f32, transform: &mut Self) {
        transform.rotation_xy = rotation_xy;
        transform.rotor = rotor3_ext::from_rotation_xy_deg(rotation_xy);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}
