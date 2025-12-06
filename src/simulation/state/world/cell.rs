use crate::simulation::{
    constants::CELL_SIZE_IN_METERS,
    state::{physics::box_collider::BoxCollider, world::block},
};
use ultraviolet::{IVec3, Vec3};

#[derive(Clone, Debug)]
pub struct Cell {
    pub cell_id: usize,
    pub sector_id: usize,
    pub grid_position: IVec3,
    pub block_kind: block::Kind,
    pub solid: bool,
}

impl Cell {
    pub fn box_collider(x: i32, y: i32, z: i32) -> BoxCollider {
        BoxCollider::new(
            Vec3::new(x as f32, y as f32, z as f32),
            Vec3::broadcast(CELL_SIZE_IN_METERS),
        )
    }
}
