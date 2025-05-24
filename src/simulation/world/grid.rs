pub mod axis;
pub mod direction;

pub use axis::Axis;
pub use direction::Direction;

use crate::simulation::{
    consts::*,
    physics::aabb::AABB,
    world::{block, chunk, grid},
};
use glam::{IVec3, Vec3};

pub fn is_valid(grid_position: IVec3) -> bool {
    let grid_boundary = GRID_BOUNDARY as i32;

    let in_x_range = grid_position.x.abs() <= grid_boundary;
    let in_y_range = grid_position.y.abs() <= grid_boundary;
    let in_z_range = grid_position.z.abs() <= grid_boundary;

    in_x_range && in_y_range && in_z_range
}

pub fn world_to_grid(world_position: Vec3) -> Option<IVec3> {
    let grid_position = (world_position + Vec3::splat(0.5)).floor().as_ivec3();

    if is_valid(grid_position) {
        Some(grid_position)
    } else {
        None
    }
}

pub fn grid_to_chunk(grid_position: IVec3) -> Option<IVec3> {
    if is_valid(grid_position) {
        let grid_position_shifted = grid_position + IVec3::splat(GRID_BOUNDARY as i32);

        let chunk_position =
            grid_position_shifted.map(|coordinate| coordinate.div_euclid(CHUNK_SIZE as i32));

        Some(chunk_position - IVec3::splat(WORLD_RADIUS as i32))
    } else {
        None
    }
}

pub fn grid_to_block(grid_position: IVec3) -> Option<IVec3> {
    if is_valid(grid_position) {
        let grid_position_shifted = grid_position + IVec3::splat(GRID_BOUNDARY as i32);

        let chunk_position =
            grid_position_shifted.map(|coordinate| coordinate.div_euclid(CHUNK_SIZE as i32));
        let chunk_center = chunk_position * CHUNK_SIZE as i32;

        let block_position = grid_position_shifted - chunk_center;
        let block_position = block_position - IVec3::splat(CHUNK_RADIUS as i32);

        Some(block_position)
    } else {
        None
    }
}

pub fn chunk_to_grid(chunk_position: IVec3) -> Option<IVec3> {
    let grid_position = CHUNK_SIZE as i32 * chunk_position;

    if grid::is_valid(grid_position) {
        Some(grid_position)
    } else {
        None
    }
}

pub fn grid_to_chunk_id(grid_position: IVec3) -> Option<chunk::ID> {
    if is_valid(grid_position) {
        let chunk_position = grid_to_chunk(grid_position)?;

        let x = (chunk_position.x + WORLD_RADIUS as i32) as usize;
        let y = (chunk_position.y + WORLD_RADIUS as i32) as usize;
        let z = (chunk_position.z + WORLD_RADIUS as i32) as usize;

        let chunk_id = z * WORLD_AREA + y * WORLD_SIZE + x;

        Some(chunk::ID(chunk_id))
    } else {
        None
    }
}

pub fn world_to_chunk_id(world_position: Vec3) -> Option<chunk::ID> {
    let grid_position = world_to_grid(world_position)?;
    let chunk_id = grid_to_chunk_id(grid_position)?;

    Some(chunk_id)
}

pub fn chunk_id_to_position(chunk_id: chunk::ID) -> Option<IVec3> {
    if chunk::ID::is_valid(chunk_id) {
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

pub fn grid_to_block_id(grid_position: IVec3) -> Option<block::ID> {
    if is_valid(grid_position) {
        let block_position = grid_to_block(grid_position)?;

        let block_id = (block_position.x + CHUNK_RADIUS as i32)
            + (block_position.y + CHUNK_RADIUS as i32) * CHUNK_SIZE as i32
            + (block_position.z + CHUNK_RADIUS as i32) * CHUNK_AREA as i32;

        Some(block::ID(block_id as usize))
    } else {
        None
    }
}

pub fn block_id_to_position(block_id: block::ID) -> Option<IVec3> {
    if block::ID::is_valid(block_id) {
        let block_id: usize = block_id.into();

        let x = (block_id % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
        let y = (block_id / CHUNK_SIZE % CHUNK_SIZE) as i32 - CHUNK_RADIUS as i32;
        let z = (block_id / CHUNK_AREA) as i32 - CHUNK_RADIUS as i32;

        Some(IVec3::new(x, y, z))
    } else {
        None
    }
}

pub fn grid_to_ids(grid_position: IVec3) -> Option<(chunk::ID, block::ID)> {
    let chunk_id = grid_to_chunk_id(grid_position)?;
    let block_id = grid_to_block_id(grid_position)?;

    Some((chunk_id, block_id))
}

pub fn ids_to_grid(chunk_id: chunk::ID, block_id: block::ID) -> Option<IVec3> {
    let chunk_position = chunk_id_to_position(chunk_id)?;
    let block_position = block_id_to_position(block_id)?;

    let grid_position = CHUNK_SIZE as i32 * chunk_position + block_position;

    Some(grid_position)
}

pub fn overlapping_aabb_list(aabb: &AABB) -> Vec<AABB> {
    let mut aabb_list = Vec::new();

    let min = aabb.min.round().as_ivec3();
    let max = aabb.max.round().as_ivec3();

    let size = Vec3::splat(BLOCK_SIZE);

    for x in min.x..=max.x {
        for y in min.y..=max.y {
            for z in min.z..=max.z {
                let block_position = Vec3::new(x as f32, y as f32, z as f32);
                let block_aabb = AABB::new(block_position, size);

                if block_aabb.overlaps(aabb) {
                    aabb_list.push(block_aabb);
                }
            }
        }
    }

    aabb_list
}
