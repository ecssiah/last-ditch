pub mod direction;
pub mod face;
pub mod id;
pub mod kind;
pub mod light;
pub mod meta;
pub mod neighbors;

pub use direction::Direction;
pub use face::Face;
pub use id::ID;
pub use kind::Kind;
pub use light::Light;
pub use meta::Meta;
pub use neighbors::Neighbors;

use crate::simulation::{block, consts::*, world::World};
use glam::IVec3;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub emittance: u8,
    pub solid: bool,
    pub color: (f32, f32, f32, f32),
}

impl Block {
    pub fn id_at(position: IVec3) -> Option<block::ID> {
        let position_shift = position + IVec3::splat(CHUNK_RADIUS as i32);

        let block_id = position_shift.x
            + position_shift.y * CHUNK_SIZE as i32
            + position_shift.z * CHUNK_AREA as i32;

        let block_id = block::ID(block_id as usize);

        Some(block_id)
    }

    pub fn id_at_grid(grid_position: IVec3) -> Option<block::ID> {
        if World::on_map(grid_position) {
            let grid_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shift = coordinate + CHUNK_RADIUS as i32;

                coordinate_shift.rem_euclid(CHUNK_SIZE as i32)
            });

            let block_id = grid_position_shifted.x
                + grid_position_shifted.y * CHUNK_SIZE as i32
                + grid_position_shifted.z * CHUNK_AREA as i32;

            Some(block::ID(block_id as usize))
        } else {
            None
        }
    }

    pub fn position(block_id: block::ID) -> Option<IVec3> {
        if block::ID::valid(block_id) {
            let block_id: usize = block_id.into();

            let x = (block_id % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
            let y = (block_id / CHUNK_SIZE % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
            let z = (block_id / CHUNK_AREA) as i32 - CHUNK_RADIUS as i32;

            let block_position = IVec3::new(x, y, z);

            Some(block_position)
        } else {
            None
        }
    }

    pub fn position_at(grid_position: IVec3) -> Option<IVec3> {
        let block_id = Self::id_at_grid(grid_position)?;
        let position = Self::position(block_id)?;

        Some(position)
    }
}
