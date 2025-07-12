pub mod axis;
pub mod direction;

pub use axis::Axis;
pub use direction::Direction;

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
    pub chunk_radius: u32,
    pub chunk_size: u32,
    pub chunk_area: u32,
    pub chunk_volume: u32,
    pub world_radius: u32,
    pub world_size: u32,
    pub world_area: u32,
    pub world_volume: u32,
    pub world_limit: u32,
    pub block_index_max: u32,
    pub chunk_index_max: u32,
}

impl Grid {
    pub fn new(kind: simulation::Kind) -> Self {
        let config = kind.config();

        let world_radius = config.world_radius;
        let world_size = 2 * config.world_radius + 1;
        let world_area = world_size * world_size;
        let world_volume = world_size * world_size * world_size;

        let chunk_radius = config.chunk_radius;
        let chunk_size = 2 * config.chunk_radius + 1;
        let chunk_area = chunk_size * chunk_size;
        let chunk_volume = chunk_size * chunk_size * chunk_size;

        let world_limit = chunk_radius + world_radius * chunk_size;

        let block_index_max = chunk_volume - 1;
        let chunk_index_max = world_volume - 1;

        Self {
            world_radius,
            world_size,
            world_area,
            world_volume,
            chunk_radius,
            chunk_size,
            chunk_area,
            chunk_volume,
            world_limit,
            block_index_max,
            chunk_index_max,
        }
    }

    pub fn block_ids(grid: &Grid) -> Vec<block::ID> {
        (0u32..grid.chunk_volume).map(block::ID).collect()
    }

    pub fn chunk_ids(grid: &Grid) -> Vec<chunk::ID> {
        (0u32..grid.world_volume).map(chunk::ID).collect()
    }

    pub fn chunk_id_valid(grid: &Grid, chunk_id: chunk::ID) -> bool {
        (0u32..grid.world_volume).contains(&u32::from(chunk_id))
    }

    pub fn block_id_valid(grid: &Grid, block_id: block::ID) -> bool {
        (0u32..grid.chunk_volume).contains(&u32::from(block_id))
    }

    pub fn position_valid(grid: &Grid, position: IVec3) -> bool {
        let in_x_range = position.x.unsigned_abs() <= grid.world_limit;
        let in_y_range = position.y.unsigned_abs() <= grid.world_limit;
        let in_z_range = position.z.unsigned_abs() <= grid.world_limit;

        in_x_range && in_y_range && in_z_range
    }

    pub fn block_id_to_block_coordinates(grid: &Grid, block_id: block::ID) -> IVec3 {
        if Grid::block_id_valid(grid, block_id) {
            let block_index = u32::from(block_id);
            let block_coordinates = indexing::index_to_vector(block_index, grid.chunk_radius);

            block_coordinates
        } else {
            IVec3::MAX
        }
    }

    pub fn block_coordinates_to_block_id(grid: &Grid, block_coordinates: IVec3) -> block::ID {
        let block_coordinates_indexable =
            indexing::indexable_vector(block_coordinates, grid.chunk_radius);

        if block_coordinates_indexable != IVec3::MAX {
            let block_index =
                indexing::vector_to_index(block_coordinates_indexable, grid.chunk_radius);

            block::ID(block_index)
        } else {
            block::ID::MAX
        }
    }

    pub fn chunk_id_to_chunk_coordinates(grid: &Grid, chunk_id: chunk::ID) -> IVec3 {
        if Grid::chunk_id_valid(grid, chunk_id) {
            let chunk_index = u32::from(chunk_id);
            let chunk_coordinates = indexing::index_to_vector(chunk_index, grid.world_radius);

            chunk_coordinates
        } else {
            IVec3::MAX
        }
    }

    pub fn chunk_coordinates_to_chunk_id(grid: &Grid, chunk_coordinates: IVec3) -> chunk::ID {
        let chunk_coordinates_indexable =
            indexing::indexable_vector(chunk_coordinates, grid.world_radius);

        if chunk_coordinates_indexable != IVec3::MAX {
            let chunk_index =
                indexing::vector_to_index(chunk_coordinates_indexable, grid.world_radius);

            chunk::ID(chunk_index)
        } else {
            chunk::ID::MAX
        }
    }

    pub fn chunk_coordinates_to_position(grid: &Grid, chunk_coordinates: IVec3) -> IVec3 {
        let position = chunk_coordinates * grid.chunk_size as i32;

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
            let position_indexable = indexing::indexable_vector(position, grid.world_limit);

            if position_indexable != IVec3::MAX {
                let chunk_coordinates_indexable = position_indexable / grid.chunk_size as i32;

                let chunk_coordinates =
                    chunk_coordinates_indexable - IVec3::splat(grid.world_radius as i32);

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
            let position_indexable = indexing::indexable_vector(position, grid.world_limit);

            if position_indexable != IVec3::MAX {
                let block_coordinates_indexable = position_indexable % grid.chunk_size as i32;

                let block_coordinates =
                    block_coordinates_indexable - IVec3::splat(grid.chunk_radius as i32);

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
            grid.chunk_size as i32 * chunk_coordinates + block_coordinates
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

    pub fn blocks_overlapping(aabb: AABB) -> Vec<AABB> {
        let mut aabb_vec = Vec::new();

        let min = aabb.min.round().as_ivec3();
        let max = aabb.max.round().as_ivec3();

        let size = Vec3::splat(BLOCK_SIZE);

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
            let chunk_radius = grid.chunk_radius as i32;

            block_coordinates.x.abs() == chunk_radius
                || block_coordinates.y.abs() == chunk_radius
                || block_coordinates.z.abs() == chunk_radius
        }
    }

    pub fn on_world_limit(grid: &Grid, position: IVec3) -> bool {
        let world_limit = grid.world_limit as i32;

        position.x.abs() == world_limit
            || position.y.abs() == world_limit
            || position.z.abs() == world_limit
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
