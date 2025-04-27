pub mod direction;

pub use direction::Direction;

use crate::simulation::{
    consts::*,
    world::{block, chunk},
};
use glam::{IVec3, Vec3};

pub struct Grid {}

impl Grid {
    pub fn is_valid(grid_position: IVec3) -> bool {
        if let Some(chunk_position) = Self::grid_to_chunk(grid_position) {
            let in_x_range = chunk_position.x >= 0 && chunk_position.x < WORLD_SIZE as i32;
            let in_y_range = chunk_position.y >= 0 && chunk_position.y < WORLD_SIZE as i32;
            let in_z_range = chunk_position.z >= 0 && chunk_position.z < WORLD_SIZE as i32;

            in_x_range && in_y_range && in_z_range
        } else {
            false
        }
    }

    pub fn world_to_grid(world_position: Vec3) -> Option<IVec3> {
        let grid_position = (world_position + Vec3::splat(0.5)).floor().as_ivec3();

        if Self::is_valid(grid_position) {
            Some(grid_position)
        } else {
            None
        }
    }

    pub fn grid_to_chunk(grid_position: IVec3) -> Option<IVec3> {
        if Self::is_valid(grid_position) {
            let chunk_position = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.div_euclid(CHUNK_SIZE as i32)
            });

            Some(chunk_position)
        } else {
            None
        }
    }

    pub fn grid_to_block(grid_position: IVec3) -> Option<IVec3> {
        if Self::is_valid(grid_position) {
            let chunk_position = Self::grid_to_chunk(grid_position)?;
            let chunk_center_grid_position = chunk_position * CHUNK_SIZE as i32;
            let block_position = grid_position - chunk_center_grid_position;

            Some(block_position)
        } else {
            None
        }
    }

    pub fn chunk_to_grid(chunk_position: IVec3) -> Option<IVec3> {
        let grid_position = CHUNK_RADIUS as i32 * chunk_position;

        if Self::is_valid(grid_position) {
            Some(grid_position)
        } else {
            None
        }
    }

    pub fn get_chunk_id(grid_position: IVec3) -> Option<chunk::ID> {
        if Self::is_valid(grid_position) {
            let chunk_position = Self::grid_to_chunk(grid_position)?;

            let x = chunk_position.x as usize;
            let y = chunk_position.y as usize;
            let z = chunk_position.z as usize;

            let chunk_id = z * WORLD_AREA + y * WORLD_SIZE + x;

            Some(chunk::ID(chunk_id))
        } else {
            None
        }
    }

    pub fn get_chunk_position(chunk_id: chunk::ID) -> Option<IVec3> {
        if chunk::ID::valid(chunk_id) {
            let chunk_id: usize = chunk_id.into();

            let x = (chunk_id % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;
            let y = (chunk_id / WORLD_SIZE % WORLD_SIZE) as i32 - WORLD_RADIUS as i32;
            let z = (chunk_id / WORLD_AREA) as i32 - WORLD_RADIUS as i32;

            let local_position = IVec3::new(x, y, z);

            Some(local_position)
        } else {
            None
        }
    }

    pub fn get_block_id(grid_position: IVec3) -> Option<block::ID> {
        if Self::is_valid(grid_position) {
            let grid_position_shift = grid_position + IVec3::splat(CHUNK_RADIUS as i32);

            let block_id = grid_position_shift.x
                + grid_position_shift.y * CHUNK_SIZE as i32
                + grid_position_shift.z * CHUNK_AREA as i32;

            let block_id = block::ID(block_id as usize);

            Some(block_id)
        } else {
            None
        }
    }

    pub fn get_block_position(block_id: block::ID) -> Option<IVec3> {
        if block::ID::valid(block_id) {
            let block_id: usize = block_id.into();

            let x = (block_id % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
            let y = (block_id / CHUNK_SIZE % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
            let z = (block_id / CHUNK_AREA) as i32 - CHUNK_RADIUS as i32;

            Some(IVec3::new(x, y, z))
        } else {
            None
        }
    }

    pub fn get_ids(grid_position: IVec3) -> Option<(chunk::ID, block::ID)> {
        let chunk_id = Self::get_chunk_id(grid_position)?;
        let block_id = Self::get_block_id(grid_position)?;

        Some((chunk_id, block_id))
    }

    pub fn get_grid_position(chunk_id: chunk::ID, block_id: block::ID) -> Option<IVec3> {
        let chunk_position = Self::get_chunk_position(chunk_id)?;
        let block_position = Self::get_block_position(block_id)?;

        let grid_position = CHUNK_SIZE as i32 * chunk_position + block_position;

        Some(grid_position)
    }
}
