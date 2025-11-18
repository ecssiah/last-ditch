pub mod face;
pub mod id;

pub use face::Face;
pub use id::ID;

use crate::simulation::state::{
    physics::aabb::AABB,
    world::{
        block,
        grid::{self, Grid},
        sector,
    },
};
use ultraviolet::{IVec3, Vec3};

#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_id: ID,
    pub sector_id: sector::ID,
    pub position: IVec3,
    pub block_kind: block::Kind,
    pub solid: bool,
    pub face_array: [Face; 6],
}

impl Cell {
    pub fn face_array() -> [Face; 6] {
        [
            Face::new(grid::Direction::XPYOZO),
            Face::new(grid::Direction::XNYOZO),
            Face::new(grid::Direction::XOYPZO),
            Face::new(grid::Direction::XOYNZO),
            Face::new(grid::Direction::XOYOZP),
            Face::new(grid::Direction::XOYOZN),
        ]
    }

    pub fn get_face(direction: grid::Direction, face_array: &[Face; 6]) -> &Face {
        match direction {
            grid::Direction::XPYOZO => &face_array[0],
            grid::Direction::XNYOZO => &face_array[1],
            grid::Direction::XOYPZO => &face_array[2],
            grid::Direction::XOYNZO => &face_array[3],
            grid::Direction::XOYOZP => &face_array[4],
            grid::Direction::XOYOZN => &face_array[5],
            _ => panic!("Requested a non-existent Face"),
        }
    }

    pub fn aabb(x: i32, y: i32, z: i32, grid: &Grid) -> AABB {
        AABB::new(
            Vec3::new(x as f32, y as f32, z as f32),
            Vec3::broadcast(grid.cell_size_in_meters),
        )
    }
}
