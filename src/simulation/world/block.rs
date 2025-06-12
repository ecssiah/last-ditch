pub mod edge;
pub mod face;
pub mod id;
pub mod kind;
pub mod meta;
pub mod node;

pub use edge::Edge;
pub use edge::Key;
pub use face::Face;
pub use id::ID;
pub use kind::Kind;
pub use meta::Meta;
pub use node::Node;

use crate::simulation::{consts::*, physics::aabb::AABB};
use glam::{IVec3, Vec3};

#[derive(Debug)]
pub struct Block {
    pub id: ID,
    pub position: IVec3,
    pub kind: Kind,
    pub solid: bool,
}

pub fn aabb(x: i32, y: i32, z: i32) -> AABB {
    AABB::new(
        Vec3::new(x as f32, y as f32, z as f32),
        Vec3::splat(BLOCK_SIZE),
    )
}
