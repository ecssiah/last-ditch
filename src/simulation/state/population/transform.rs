use ultraviolet::{Rotor3, Vec3};

use crate::utils::ldmath::rotor3_ext;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub world_position: Vec3,
    pub rotation_xy: f32,
    pub rotor: Rotor3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            world_position: Vec3::default(),
            rotation_xy: 0.0,
            rotor: rotor3_ext::from_rotation_xy_deg(0.0),
        }
    }

    pub fn default() -> Self {
        Self::new()
    }

    pub fn set_world_position(world_position: Vec3, transform: &mut Self) {
        transform.world_position = world_position;
    }

    pub fn set_rotation(rotation_xy: f32, transform: &mut Self) {
        transform.rotation_xy = rotation_xy;
        transform.rotor = rotor3_ext::from_rotation_xy_deg(rotation_xy);
    }
}
