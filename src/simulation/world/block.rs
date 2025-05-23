pub mod face;
pub mod id;
pub mod kind;
pub mod light;
pub mod meta;

pub use face::Face;
pub use id::ID;
pub use kind::Kind;
pub use light::Light;
pub use meta::Meta;

use crate::simulation::{physics::aabb::AABB, BLOCK_SIZE};
use glam::Vec3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub emittance: u8,
    pub solid: bool,
}

impl Block {
    pub fn get_aabb(x: i32, y: i32, z: i32) -> AABB {
        AABB::new(
            Vec3::new(x as f32, y as f32, z as f32),
            Vec3::splat(BLOCK_SIZE),
        )
    }
}
