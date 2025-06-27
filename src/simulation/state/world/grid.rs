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
    pub world_boundary: u32,
    pub block_index_max: u32,
    pub chunk_index_max: u32,
}

impl Grid {
    pub fn new(mode: simulation::Mode) -> Self {
        let config = mode.config();

        let world_radius = config.world_radius;
        let world_size = 2 * config.world_radius + 1;
        let world_area = world_size * world_size;
        let world_volume = world_size * world_size * world_size;

        let chunk_radius = config.chunk_radius;
        let chunk_size = 2 * config.chunk_radius + 1;
        let chunk_area = chunk_size * chunk_size;
        let chunk_volume = chunk_size * chunk_size * chunk_size;

        let world_boundary = chunk_radius + world_radius * chunk_size;

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
            world_boundary,
            block_index_max,
            chunk_index_max,
        }
    }

    pub fn block_ids(&self) -> Vec<block::ID> {
        (0u32..self.chunk_volume).map(block::ID).collect()
    }

    pub fn chunk_ids(&self) -> Vec<chunk::ID> {
        (0u32..self.world_volume).map(chunk::ID).collect()
    }

    pub fn chunk_id_valid(&self, chunk_id: chunk::ID) -> bool {
        (0u32..self.world_volume).contains(&u32::from(chunk_id))
    }

    pub fn block_id_valid(&self, block_id: block::ID) -> bool {
        (0u32..self.chunk_volume).contains(&u32::from(block_id))
    }

    pub fn position_valid(&self, position: IVec3) -> bool {
        let in_x_range = position.x.unsigned_abs() <= self.world_boundary;
        let in_y_range = position.y.unsigned_abs() <= self.world_boundary;
        let in_z_range = position.z.unsigned_abs() <= self.world_boundary;

        in_x_range && in_y_range && in_z_range
    }

    pub fn block_id_to_block_coordinates(&self, block_id: block::ID) -> Option<IVec3> {
        if self.block_id_valid(block_id) {
            let block_index = u32::from(block_id);
            let block_coordinates = indexing::index_to_vector(block_index, self.chunk_radius);

            Some(block_coordinates)
        } else {
            None
        }
    }

    pub fn block_coordinates_to_block_id(&self, block_coordinates: IVec3) -> Option<block::ID> {
        let block_coordinates_indexable =
            indexing::indexable_vector(block_coordinates, self.chunk_radius)?;
        let block_index = indexing::vector_to_index(block_coordinates_indexable, self.chunk_radius);

        Some(block::ID(block_index))
    }

    pub fn chunk_id_to_chunk_coordinates(&self, chunk_id: chunk::ID) -> Option<IVec3> {
        if self.chunk_id_valid(chunk_id) {
            let chunk_index = u32::from(chunk_id);
            let chunk_coordinates = indexing::index_to_vector(chunk_index, self.world_radius);

            Some(chunk_coordinates)
        } else {
            None
        }
    }

    pub fn chunk_coordinates_to_chunk_id(&self, chunk_coordinates: IVec3) -> Option<chunk::ID> {
        let chunk_coordinates_indexable =
            indexing::indexable_vector(chunk_coordinates, self.world_radius)?;
        let chunk_index = indexing::vector_to_index(chunk_coordinates_indexable, self.world_radius);

        Some(chunk::ID(chunk_index))
    }

    pub fn chunk_coordinates_to_position(&self, chunk_coordinates: IVec3) -> Option<IVec3> {
        let position = chunk_coordinates * self.chunk_size as i32;

        if self.position_valid(position) {
            Some(position)
        } else {
            None
        }
    }

    pub fn chunk_id_to_position(&self, chunk_id: chunk::ID) -> Option<IVec3> {
        let chunk_coordinates = self.chunk_id_to_chunk_coordinates(chunk_id)?;

        self.chunk_coordinates_to_position(chunk_coordinates)
    }

    pub fn position_to_chunk_coordinates(&self, position: IVec3) -> Option<IVec3> {
        if self.position_valid(position) {
            let position_indexable = indexing::indexable_vector(position, self.world_boundary)?;

            let chunk_coordinates_indexable = position_indexable / self.chunk_size as i32;

            let chunk_coordinates =
                chunk_coordinates_indexable - IVec3::splat(self.world_radius as i32);

            Some(chunk_coordinates)
        } else {
            None
        }
    }

    pub fn position_to_block_coordinates(&self, position: IVec3) -> Option<IVec3> {
        if self.position_valid(position) {
            let position_indexable = indexing::indexable_vector(position, self.world_boundary)?;

            let block_coordinates_indexable = position_indexable % self.chunk_size as i32;

            let block_coordinates =
                block_coordinates_indexable - IVec3::splat(self.chunk_radius as i32);

            Some(block_coordinates)
        } else {
            None
        }
    }

    pub fn position_to_chunk_id(&self, position: IVec3) -> Option<chunk::ID> {
        let chunk_coordinates = self.position_to_chunk_coordinates(position)?;

        self.chunk_coordinates_to_chunk_id(chunk_coordinates)
    }

    pub fn position_to_block_id(&self, position: IVec3) -> Option<block::ID> {
        let block_coordinates = self.position_to_block_coordinates(position)?;

        self.block_coordinates_to_block_id(block_coordinates)
    }

    pub fn position_to_ids(&self, position: IVec3) -> Option<(chunk::ID, block::ID)> {
        let chunk_id = self.position_to_chunk_id(position)?;
        let block_id = self.position_to_block_id(position)?;

        Some((chunk_id, block_id))
    }

    pub fn ids_to_position(&self, chunk_id: chunk::ID, block_id: block::ID) -> Option<IVec3> {
        let chunk_coordinates = self.chunk_id_to_chunk_coordinates(chunk_id)?;
        let block_coordinates = self.block_id_to_block_coordinates(block_id)?;

        Some(self.chunk_size as i32 * chunk_coordinates + block_coordinates)
    }

    pub fn world_to_position(&self, world_position: Vec3) -> Option<IVec3> {
        let position = (world_position + Vec3::splat(0.5)).floor().as_ivec3();

        if self.position_valid(position) {
            Some(position)
        } else {
            None
        }
    }

    pub fn world_to_chunk_id(&self, world_position: Vec3) -> Option<chunk::ID> {
        let position = self.world_to_position(world_position)?;

        self.position_to_chunk_id(position)
    }

    pub fn world_to_chunk_coordinates(&self, world_position: Vec3) -> Option<IVec3> {
        let position = self.world_to_position(world_position)?;

        self.position_to_chunk_coordinates(position)
    }

    pub fn world_to_block_id(&self, world_position: Vec3) -> Option<block::ID> {
        let position = self.world_to_position(world_position)?;

        self.position_to_block_id(position)
    }

    pub fn world_to_block_coordinates(&self, world_position: Vec3) -> Option<IVec3> {
        let position = self.world_to_position(world_position)?;

        self.position_to_block_coordinates(position)
    }

    pub fn blocks_overlapping(&self, aabb: AABB) -> Vec<AABB> {
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

    pub fn on_chunk_boundary(&self, position: IVec3) -> bool {
        self.position_to_block_coordinates(position)
            .is_some_and(|block_coordinates| {
                let chunk_radius = self.chunk_radius as i32;

                block_coordinates.x.abs() == chunk_radius
                    || block_coordinates.y.abs() == chunk_radius
                    || block_coordinates.z.abs() == chunk_radius
            })
    }

    pub fn on_boundary(&self, position: IVec3) -> bool {
        let world_boundary = self.world_boundary as i32;

        position.x.abs() == world_boundary
            || position.y.abs() == world_boundary
            || position.z.abs() == world_boundary
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
