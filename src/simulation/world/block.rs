pub mod edge;
pub mod face;
pub mod id;
pub mod kind;
pub mod node;

pub use edge::Edge;
pub use face::Face;
pub use id::ID;
pub use kind::Kind;
pub use node::Node;

use crate::simulation::{physics::aabb::AABB, BLOCK_SIZE};
use glam::Vec3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Block {
    pub(crate) kind: Kind,
    pub(crate) opacity: f32,
    pub(crate) emittance: u8,
    pub(crate) solid: bool,
}

pub fn aabb(x: i32, y: i32, z: i32) -> AABB {
    AABB::new(
        Vec3::new(x as f32, y as f32, z as f32),
        Vec3::splat(BLOCK_SIZE),
    )
}
