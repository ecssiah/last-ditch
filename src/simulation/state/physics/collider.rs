pub mod info;
pub mod kind;
pub mod owner;

pub use info::Info;
pub use kind::Kind;
pub use owner::Owner;

use crate::simulation::state::physics::collider;
use ultraviolet::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Collider {
    pub relative_position: Vec3,
    pub world_position: Vec3,
    pub size: Vec3,
    pub kind: collider::Kind,
    pub owner: collider::Owner,
}

impl Collider {
    pub fn new(world_position: Vec3, size: Vec3) -> Self {
        let relative_position = Vec3::new(0.0, 0.0, 0.9);
        let kind = collider::Kind::Physics;
        let owner = collider::Owner::None;

        Self {
            relative_position,
            world_position,
            size,
            kind,
            owner,
        }
    }

    pub fn default() -> Self {
        Self::new(Vec3::zero(), Vec3::one())
    }

    pub fn set_relative_position(relative_position: Vec3, collider: &mut Self) {
        collider.relative_position = relative_position;
    }

    pub fn set_world_position(world_position: Vec3, collider: &mut Self) {
        collider.world_position = world_position;
    }

    pub fn set_size(size: Vec3, collider: &mut Self) {
        collider.size = size;
    }
}
