pub mod face;
pub mod id;
pub mod info;
pub mod kind;

pub use face::Face;
pub use id::ID;
pub use info::Info;
pub use kind::Kind;

use crate::simulation::state::{
    physics::aabb::AABB,
    world::{
        grid::{self, Grid},
        sector,
    },
};
use glam::{IVec3, Vec3};

#[derive(Clone, Debug)]
pub struct Cell {
    pub id: ID,
    pub sector_id: sector::ID,
    pub position: IVec3,
    pub kind: Kind,
    pub solid: bool,
    pub face_array: [Face; 6],
}

impl Cell {
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

    pub fn get_face(direction: grid::Direction, face_array: &[Face; 6]) -> &Face {
        match direction {
            grid::Direction::XpYoZo => &face_array[0],
            grid::Direction::XnYoZo => &face_array[1],
            grid::Direction::XoYpZo => &face_array[2],
            grid::Direction::XoYnZo => &face_array[3],
            grid::Direction::XoYoZp => &face_array[4],
            grid::Direction::XoYoZn => &face_array[5],
            _ => panic!("Requested a non-existent Face"),
        }
    }

    pub fn aabb(x: i32, y: i32, z: i32, grid: &Grid) -> AABB {
        AABB::new(
            Vec3::new(x as f32, y as f32, z as f32),
            Vec3::splat(grid.cell_size_in_meters),
        )
    }
}
