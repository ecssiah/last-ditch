pub mod axis;
pub mod block_sample;
pub mod direction;
pub mod world_ray_iter;

pub use axis::Axis;
pub use block_sample::BlockSample;
pub use direction::Direction;
pub use world_ray_iter::WorldRayIter;

use crate::simulation::{
    self,
    consts::*,
    state::{
        physics::aabb::AABB,
        world::{block, chunk},
    },
    utils::indexing,
};
use glam::{IVec3, Vec3};
use once_cell::sync::Lazy;
use std::collections::HashMap;

static INTERMEDIATE_POSITION_MAP: Lazy<HashMap<IVec3, [IVec3; 2]>> = Lazy::new(|| {
    HashMap::from([
        (
            IVec3::new(1, 1, 1),
            [IVec3::new(1, 1, 0), IVec3::new(0, 1, 1)],
        ),
        (
            IVec3::new(-1, 1, 1),
            [IVec3::new(-1, 1, 0), IVec3::new(0, 1, 1)],
        ),
        (
            IVec3::new(1, 1, -1),
            [IVec3::new(1, 1, 0), IVec3::new(0, 1, -1)],
        ),
        (
            IVec3::new(-1, 1, -1),
            [IVec3::new(-1, 1, 0), IVec3::new(0, 1, -1)],
        ),
        (
            IVec3::new(1, 0, 1),
            [IVec3::new(1, 0, 0), IVec3::new(0, 0, 1)],
        ),
        (
            IVec3::new(-1, 0, 1),
            [IVec3::new(-1, 0, 0), IVec3::new(0, 0, 1)],
        ),
        (
            IVec3::new(1, 0, -1),
            [IVec3::new(1, 0, 0), IVec3::new(0, 0, -1)],
        ),
        (
            IVec3::new(-1, 0, -1),
            [IVec3::new(-1, 0, 0), IVec3::new(0, 0, -1)],
        ),
        (
            IVec3::new(1, -1, 1),
            [IVec3::new(1, 0, 0), IVec3::new(0, 0, 1)],
        ),
        (
            IVec3::new(-1, -1, 1),
            [IVec3::new(-1, 0, 0), IVec3::new(0, 0, 1)],
        ),
        (
            IVec3::new(1, -1, -1),
            [IVec3::new(1, 0, 0), IVec3::new(0, 0, -1)],
        ),
        (
            IVec3::new(-1, -1, -1),
            [IVec3::new(-1, 0, 0), IVec3::new(0, 0, -1)],
        ),
    ])
});

#[derive(Clone, Copy)]
pub struct Grid {
    pub block_extent: f32,
    pub block_size: f32,
    pub block_area: f32,
    pub block_volume: f32,
    pub chunk_extent_blocks: u32,
    pub chunk_size_blocks: u32,
    pub chunk_area_blocks: u32,
    pub chunk_volume_blocks: u32,
    pub chunk_extent_units: f32,
    pub chunk_size_units: f32,
    pub chunk_area_units: f32,
    pub chunk_volume_units: f32,
    pub world_extent_chunks: u32,
    pub world_size_chunks: u32,
    pub world_area_chunks: u32,
    pub world_volume_chunks: u32,
    pub world_extent_blocks: u32,
    pub world_size_blocks: u32,
    pub world_area_blocks: u32,
    pub world_volume_blocks: u32,
    pub world_extent_units: f32,
    pub world_size_units: f32,
    pub world_area_units: f32,
    pub world_volume_units: f32,
}

impl Grid {
    pub fn new(kind: simulation::Kind) -> Self {
        let config = kind.config();

        let block_extent = BLOCK_EXTENT;
        let block_size = 2.0 * block_extent;
        let block_area = block_size * block_size;
        let block_volume = block_size * block_size * block_size;

        let chunk_extent_blocks = config.chunk_extent_blocks;
        let chunk_size_blocks = 2 * config.chunk_extent_blocks + 1;
        let chunk_area_blocks = chunk_size_blocks * chunk_size_blocks;
        let chunk_volume_blocks = chunk_size_blocks * chunk_size_blocks * chunk_size_blocks;

        let chunk_extent_units = chunk_extent_blocks as f32 * block_size + block_extent;
        let chunk_size_units = chunk_size_blocks as f32 * block_size;
        let chunk_area_units = chunk_size_units * chunk_size_units;
        let chunk_volume_units = chunk_size_units * chunk_size_units * chunk_size_units;

        let world_extent_chunks = config.world_extent_chunks;
        let world_size_chunks = 2 * config.world_extent_chunks + 1;
        let world_area_chunks = world_size_chunks * world_size_chunks;
        let world_volume_chunks = world_size_chunks * world_size_chunks * world_size_chunks;

        let world_extent_blocks = world_extent_chunks * chunk_size_blocks + chunk_extent_blocks;
        let world_size_blocks = world_size_chunks * chunk_size_blocks;
        let world_area_blocks = world_size_blocks * world_size_blocks;
        let world_volume_blocks = world_size_blocks * world_size_blocks * world_size_blocks;

        let world_extent_units = world_extent_blocks as f32 * block_size + block_extent;
        let world_size_units = world_size_blocks as f32 * block_size;
        let world_area_units = world_size_units * world_size_units;
        let world_volume_units = world_size_units * world_size_units * world_size_units;

        Self {
            block_extent,
            block_size,
            block_area,
            block_volume,
            chunk_extent_blocks,
            chunk_size_blocks,
            chunk_area_blocks,
            chunk_volume_blocks,
            chunk_extent_units,
            chunk_size_units,
            chunk_area_units,
            chunk_volume_units,
            world_extent_chunks,
            world_size_chunks,
            world_area_chunks,
            world_volume_chunks,
            world_extent_blocks,
            world_size_blocks,
            world_area_blocks,
            world_volume_blocks,
            world_extent_units,
            world_size_units,
            world_area_units,
            world_volume_units,
        }
    }

    pub fn block_ids(grid: &Grid) -> Vec<block::ID> {
        (0u32..grid.chunk_volume_blocks).map(block::ID).collect()
    }

    pub fn chunk_ids(grid: &Grid) -> Vec<chunk::ID> {
        (0u32..grid.world_volume_chunks).map(chunk::ID).collect()
    }

    pub fn chunk_id_valid(grid: &Grid, chunk_id: chunk::ID) -> bool {
        (0u32..grid.world_volume_chunks).contains(&u32::from(chunk_id))
    }

    pub fn block_id_valid(grid: &Grid, block_id: block::ID) -> bool {
        (0u32..grid.chunk_volume_blocks).contains(&u32::from(block_id))
    }

    pub fn position_valid(grid: &Grid, position: IVec3) -> bool {
        let in_x_range = position.x.unsigned_abs() <= grid.world_extent_blocks;
        let in_y_range = position.y.unsigned_abs() <= grid.world_extent_blocks;
        let in_z_range = position.z.unsigned_abs() <= grid.world_extent_blocks;

        in_x_range && in_y_range && in_z_range
    }

    pub fn block_id_to_block_coordinates(grid: &Grid, block_id: block::ID) -> IVec3 {
        if Grid::block_id_valid(grid, block_id) {
            let block_index = u32::from(block_id);
            let block_coordinates =
                indexing::index_to_vector(block_index, grid.chunk_extent_blocks);

            block_coordinates
        } else {
            IVec3::MAX
        }
    }

    pub fn block_coordinates_to_block_id(grid: &Grid, block_coordinates: IVec3) -> block::ID {
        let block_coordinates_indexable =
            indexing::indexable_vector(block_coordinates, grid.chunk_extent_blocks);

        if block_coordinates_indexable != IVec3::MAX {
            let block_index =
                indexing::vector_to_index(block_coordinates_indexable, grid.chunk_extent_blocks);

            block::ID(block_index)
        } else {
            block::ID::MAX
        }
    }

    pub fn chunk_id_to_chunk_coordinates(grid: &Grid, chunk_id: chunk::ID) -> IVec3 {
        if Grid::chunk_id_valid(grid, chunk_id) {
            let chunk_index = u32::from(chunk_id);
            let chunk_coordinates =
                indexing::index_to_vector(chunk_index, grid.world_extent_chunks);

            chunk_coordinates
        } else {
            IVec3::MAX
        }
    }

    pub fn chunk_coordinates_to_chunk_id(grid: &Grid, chunk_coordinates: IVec3) -> chunk::ID {
        let chunk_coordinates_indexable =
            indexing::indexable_vector(chunk_coordinates, grid.world_extent_chunks);

        if chunk_coordinates_indexable != IVec3::MAX {
            let chunk_index =
                indexing::vector_to_index(chunk_coordinates_indexable, grid.world_extent_chunks);

            chunk::ID(chunk_index)
        } else {
            chunk::ID::MAX
        }
    }

    pub fn chunk_coordinates_to_position(grid: &Grid, chunk_coordinates: IVec3) -> IVec3 {
        let position = chunk_coordinates * grid.chunk_size_blocks as i32;

        if Grid::position_valid(grid, position) {
            position
        } else {
            IVec3::MAX
        }
    }

    pub fn chunk_id_to_position(grid: &Grid, chunk_id: chunk::ID) -> IVec3 {
        let chunk_coordinates = Grid::chunk_id_to_chunk_coordinates(grid, chunk_id);

        if chunk_coordinates != IVec3::MAX {
            Grid::chunk_coordinates_to_position(grid, chunk_coordinates)
        } else {
            IVec3::MAX
        }
    }

    pub fn position_to_chunk_coordinates(grid: &Grid, position: IVec3) -> IVec3 {
        if Grid::position_valid(grid, position) {
            let position_indexable = indexing::indexable_vector(position, grid.world_extent_blocks);

            if position_indexable != IVec3::MAX {
                let chunk_coordinates_indexable =
                    position_indexable / grid.chunk_size_blocks as i32;

                let chunk_coordinates =
                    chunk_coordinates_indexable - IVec3::splat(grid.world_extent_chunks as i32);

                chunk_coordinates
            } else {
                IVec3::MAX
            }
        } else {
            IVec3::MAX
        }
    }

    pub fn position_to_block_coordinates(grid: &Grid, position: IVec3) -> IVec3 {
        if Grid::position_valid(grid, position) {
            let position_indexable = indexing::indexable_vector(position, grid.world_extent_blocks);

            if position_indexable != IVec3::MAX {
                let block_coordinates_indexable =
                    position_indexable % grid.chunk_size_blocks as i32;

                let block_coordinates =
                    block_coordinates_indexable - IVec3::splat(grid.chunk_extent_blocks as i32);

                block_coordinates
            } else {
                IVec3::MAX
            }
        } else {
            IVec3::MAX
        }
    }

    pub fn position_to_chunk_id(grid: &Grid, position: IVec3) -> chunk::ID {
        let chunk_coordinates = Grid::position_to_chunk_coordinates(grid, position);

        if chunk_coordinates != IVec3::MAX {
            Grid::chunk_coordinates_to_chunk_id(grid, chunk_coordinates)
        } else {
            chunk::ID::MAX
        }
    }

    pub fn position_to_block_id(grid: &Grid, position: IVec3) -> block::ID {
        let block_coordinates = Grid::position_to_block_coordinates(grid, position);

        if block_coordinates != IVec3::MAX {
            Grid::block_coordinates_to_block_id(grid, block_coordinates)
        } else {
            block::ID::MAX
        }
    }

    pub fn position_to_ids(grid: &Grid, position: IVec3) -> (chunk::ID, block::ID) {
        let chunk_id = Grid::position_to_chunk_id(grid, position);
        let block_id = Grid::position_to_block_id(grid, position);

        (chunk_id, block_id)
    }

    pub fn ids_to_position(grid: &Grid, chunk_id: chunk::ID, block_id: block::ID) -> IVec3 {
        let chunk_coordinates = Grid::chunk_id_to_chunk_coordinates(grid, chunk_id);
        let block_coordinates = Grid::block_id_to_block_coordinates(grid, block_id);

        if chunk_coordinates != IVec3::MAX && block_coordinates != IVec3::MAX {
            grid.chunk_size_blocks as i32 * chunk_coordinates + block_coordinates
        } else {
            IVec3::MAX
        }
    }

    pub fn world_to_position(grid: &Grid, world_position: Vec3) -> IVec3 {
        let position = (world_position + Vec3::splat(0.5)).floor().as_ivec3();

        if Grid::position_valid(grid, position) {
            position
        } else {
            IVec3::MAX
        }
    }

    pub fn world_to_chunk_id(grid: &Grid, world_position: Vec3) -> chunk::ID {
        let position = Grid::world_to_position(grid, world_position);

        if Grid::position_valid(grid, position) {
            Grid::position_to_chunk_id(grid, position)
        } else {
            chunk::ID::MAX
        }
    }

    pub fn world_to_chunk_coordinates(grid: &Grid, world_position: Vec3) -> IVec3 {
        let position = Grid::world_to_position(grid, world_position);

        if Grid::position_valid(grid, position) {
            Grid::position_to_chunk_coordinates(grid, position)
        } else {
            IVec3::MAX
        }
    }

    pub fn world_to_block_id(grid: &Grid, world_position: Vec3) -> block::ID {
        let position = Grid::world_to_position(grid, world_position);

        if Grid::position_valid(grid, position) {
            Grid::position_to_block_id(grid, position)
        } else {
            block::ID::MAX
        }
    }

    pub fn world_to_block_coordinates(grid: &Grid, world_position: Vec3) -> IVec3 {
        let position = Grid::world_to_position(grid, world_position);

        if Grid::position_valid(grid, position) {
            Grid::position_to_block_coordinates(grid, position)
        } else {
            IVec3::MAX
        }
    }

    pub fn blocks_overlapping(grid: &Grid, aabb: AABB) -> Vec<AABB> {
        let mut aabb_vec = Vec::new();

        let min = aabb.min.round().as_ivec3();
        let max = aabb.max.round().as_ivec3();

        let size = Vec3::splat(grid.block_size);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let block_position = IVec3::new(x, y, z);
                    let block_aabb = AABB::new(block_position.as_vec3(), size);

                    if block_aabb.overlaps(aabb) {
                        aabb_vec.push(block_aabb);
                    }
                }
            }
        }

        aabb_vec
    }

    pub fn on_chunk_boundary(grid: &Grid, position: IVec3) -> bool {
        let block_coordinates = Grid::position_to_block_coordinates(grid, position);

        if block_coordinates == IVec3::MAX {
            true
        } else {
            let chunk_extent_blocks = grid.chunk_extent_blocks as i32;

            block_coordinates.x.abs() == chunk_extent_blocks
                || block_coordinates.y.abs() == chunk_extent_blocks
                || block_coordinates.z.abs() == chunk_extent_blocks
        }
    }

    pub fn on_world_extent_blocks(grid: &Grid, position: IVec3) -> bool {
        let world_extent_blocks = grid.world_extent_blocks as i32;

        position.x.abs() == world_extent_blocks
            || position.y.abs() == world_extent_blocks
            || position.z.abs() == world_extent_blocks
    }

    pub fn offsets_in(radius: i32) -> impl Iterator<Item = IVec3> {
        (-radius..=radius).flat_map(move |x| {
            (-radius..=radius)
                .flat_map(move |y| (-radius..=radius).map(move |z| IVec3::new(x, y, z)))
        })
    }

    pub fn intermediate_positions(from_position: IVec3, to_position: IVec3) -> Vec<IVec3> {
        let delta = to_position - from_position;

        INTERMEDIATE_POSITION_MAP
            .get(&delta)
            .map(|[offset1, offset2]| vec![from_position + *offset1, from_position + *offset2])
            .unwrap_or_default()
    }
}
