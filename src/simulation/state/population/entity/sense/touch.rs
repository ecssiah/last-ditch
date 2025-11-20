use crate::simulation::state::physics::aabb::AABB;
use ultraviolet::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Touch {
    pub body: AABB,
    pub upper: AABB,
    pub lower: AABB,
    pub ground: AABB,
}

impl Touch {
    pub fn new() -> Self {
        Self {
            body: AABB::default(),
            upper: AABB::default(),
            lower: AABB::default(),
            ground: AABB::default(),
        }
    }

    pub fn set_world_position(world_position: Vec3, touch: &mut Touch) {
        touch
            .body
            .set_bottom_center(world_position.x, world_position.y, world_position.z);
    }

    pub fn set_size(size: Vec3, touch: &mut Touch) {
        touch.body.set_size(size);
    }
}
