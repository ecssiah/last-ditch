use crate::simulation::state::physics::aabb::AABB;
use glam::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Detection {
    pub body: AABB,
    pub upper: AABB,
    pub lower: AABB,
    pub ground: AABB,
}

impl Detection {
    pub fn new() -> Self {
        Self {
            body: AABB::default(),
            upper: AABB::default(),
            lower: AABB::default(),
            ground: AABB::default(),
        }
    }

    pub fn set_world_position(world_position: Vec3, body_aabb: &mut AABB) {
        body_aabb.set_bottom_center(world_position.x, world_position.y, world_position.z);
    }
}
