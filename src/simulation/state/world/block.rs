pub mod face;
pub mod id;
pub mod info;
pub mod kind;

pub use face::Face;
pub use id::ID;
pub use info::Info;
pub use kind::Kind;

use crate::simulation::{
    consts::BLOCK_SIZE,
    state::{
        physics::aabb::AABB,
        world::{chunk, grid},
    },
};
use glam::{IVec3, Vec3};

#[derive(Clone, Debug)]
pub struct Block {
    pub id: ID,
    pub chunk_id: chunk::ID,
    pub position: IVec3,
    pub kind: Kind,
    pub solid: bool,
    pub face_array: [Face; 6],
}

impl Block {
    pub fn face_array() -> [Face; 6] {
        [
            Face::new(grid::Direction::XpYoZo),
            Face::new(grid::Direction::XnYoZo),
            Face::new(grid::Direction::XoYpZo),
            Face::new(grid::Direction::XoYnZo),
            Face::new(grid::Direction::XoYoZp),
            Face::new(grid::Direction::XoYoZn),
        ]
    }

    pub fn aabb(x: i32, y: i32, z: i32) -> AABB {
        AABB::new(
            Vec3::new(x as f32, y as f32, z as f32),
            Vec3::splat(BLOCK_SIZE),
        )
    }
}
