use glam::{Quat, Vec3};

use crate::simulation::state::physics::aabb::AABB;

#[derive(Clone, Copy, Debug)]
pub struct Spatial {
    pub aabb: AABB,
    pub world_position: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub quaternion: Quat,
}

impl Spatial {
    pub fn new() -> Self {
        Self {
            aabb: AABB::default(),
            world_position: Vec3::default(),
            yaw: 0.0,
            pitch: 0.0,
            quaternion: Quat::default(),
        }
    }
}
