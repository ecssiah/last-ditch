use crate::simulation::state::physics::box_collider::BoxCollider;
use ultraviolet::{Rotor3, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Spatial {
    pub world_position: Vec3,
    pub rotation_xy: f32,
    pub rotor: Rotor3,
    pub size: Vec3,
    pub body: BoxCollider,
}

impl Spatial {
    pub fn new() -> Self {
        Self {
            world_position: Vec3::default(),
            rotation_xy: 0.0,
            size: Vec3::default(),
            body: BoxCollider::default(),
            rotor: Rotor3::default(),
        }
    }

    pub fn set_world_position(world_position: Vec3, spatial: &mut Self) {
        spatial.world_position = world_position;
        spatial.body.set_bottom_center(world_position);
    }

    pub fn set_size(size: Vec3, spatial: &mut Self) {
        spatial.size = size;
        spatial.body.set_size(size);
    }

    pub fn set_rotation(rotation_xy: f32, spatial: &mut Self) {
        let rotation_xy_radians = rotation_xy.to_radians();

        spatial.rotation_xy = rotation_xy;
        spatial.rotor = Rotor3::from_rotation_xy(rotation_xy_radians);
    }
}
