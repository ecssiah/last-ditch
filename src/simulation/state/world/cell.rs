pub mod face;
pub mod id;

pub use face::Face;
pub use id::ID;

use crate::simulation::state::{
    physics::aabb::AABB,
    world::{block, grid::Grid, sector},
};
use ultraviolet::{IVec3, Vec3};

#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_id: ID,
    pub sector_id: sector::ID,
    pub position: IVec3,
    pub block_kind: block::Kind,
    pub solid: bool,
}

impl Cell {
    pub fn aabb(x: i32, y: i32, z: i32, grid: &Grid) -> AABB {
        AABB::new(
            Vec3::new(x as f32, y as f32, z as f32),
            Vec3::broadcast(grid.cell_size_in_meters),
        )
    }
}
