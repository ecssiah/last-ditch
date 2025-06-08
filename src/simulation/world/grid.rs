pub mod axis;
pub mod direction;

pub use axis::Axis;
pub use direction::Direction;

use crate::simulation::{
    consts::*,
    physics::aabb::AABB,
    world::{block, chunk},
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

pub struct Grid {
    pub(crate) chunk_radius: u32,
    pub(crate) chunk_size: u32,
    pub(crate) chunk_area: u32,
    pub(crate) chunk_volume: u32,
    pub(crate) world_radius: u32,
    pub(crate) world_size: u32,
    pub(crate) world_area: u32,
    pub(crate) world_volume: u32,
    pub(crate) world_boundary: u32,
    pub(crate) block_index_max: u32,
    pub(crate) chunk_index_max: u32,
}

impl Grid {
    pub fn new(chunk_radius: u32, world_radius: u32) -> Self {
        let chunk_size = 2 * chunk_radius + 1;
        let chunk_area = chunk_size * chunk_size;
        let chunk_volume = chunk_size * chunk_size * chunk_size;

        let world_size = 2 * world_radius + 1;
        let world_area = world_size * world_size;
        let world_volume = world_size * world_size * world_size;
        let world_boundary = chunk_radius + world_radius * chunk_size;

        let block_index_max = chunk_volume - 1;
        let chunk_index_max = world_volume - 1;

        Grid {
            chunk_radius,
            chunk_size,
            chunk_area,
            chunk_volume,
            world_radius,
            world_size,
            world_area,
            world_volume,
            world_boundary,
            block_index_max,
            chunk_index_max,
        }
    }

    pub fn block_ids(&self) -> Vec<block::ID> {
        (0u32..self.chunk_volume)
            .map(|block_index| block::ID(block_index))
            .collect()
    }

    pub fn chunk_ids(&self) -> Vec<chunk::ID> {
        (0u32..self.world_volume)
            .map(|chunk_index| chunk::ID(chunk_index))
            .collect()
    }

    pub fn chunk_id_valid(&self, chunk_id: chunk::ID) -> bool {
        (0u32..self.world_volume).contains(&u32::from(chunk_id))
    }

    pub fn block_id_valid(&self, block_id: block::ID) -> bool {
        (0u32..self.chunk_volume).contains(&u32::from(block_id))
    }

    pub fn position_valid(&self, position: IVec3) -> bool {
        let in_x_range = position.x.abs() as u32 <= self.world_boundary;
        let in_y_range = position.y.abs() as u32 <= self.world_boundary;
        let in_z_range = position.z.abs() as u32 <= self.world_boundary;

        in_x_range && in_y_range && in_z_range
    }

    pub fn chunk_id_to_chunk_coordinates(&self, chunk_id: chunk::ID) -> Option<IVec3> {
        if self.chunk_id_valid(chunk_id) {
            let chunk_index = u32::from(chunk_id);

            println!("chunk: {:?}", chunk_index);

            let chunk_coordinates = Self::delinearize(chunk_index, self.world_radius);

            Some(chunk_coordinates)
        } else {
            None
        }
    }

    pub fn chunk_coordinates_to_chunk_id(&self, chunk_coordinates: IVec3) -> Option<chunk::ID> {
        let indexable_coordinates = chunk_coordinates + IVec3::splat(self.world_radius as i32);

        let chunk_index = Self::linearize(indexable_coordinates, self.world_radius);

        if chunk_index >= 0 && chunk_index < self.world_volume as i32 {
            Some(chunk::ID(chunk_index as u32))
        } else {
            None
        }
    }

    pub fn chunk_coordinates_to_position(&self, chunk_coordinates: IVec3) -> Option<IVec3> {
        let position = chunk_coordinates + self.chunk_size as i32;

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

    pub fn block_id_to_block_coordinates(&self, block_id: block::ID) -> Option<IVec3> {
        if self.block_id_valid(block_id) {
            let block_index = u32::from(block_id);
            let block_coordinates = Self::delinearize(block_index, self.chunk_radius);

            Some(block_coordinates)
        } else {
            None
        }
    }

    pub fn block_coordinates_to_block_id(&self, block_coordinates: IVec3) -> Option<block::ID> {
        let indexable_coordinates = block_coordinates + IVec3::splat(self.chunk_radius as i32);

        let block_index = Self::linearize(indexable_coordinates, self.chunk_radius);

        if block_index >= 0 && block_index < self.chunk_volume as i32 {
            Some(block::ID(block_index as u32))
        } else {
            None
        }
    }

    pub fn ids_to_position(&self, chunk_id: chunk::ID, block_id: block::ID) -> Option<IVec3> {
        let chunk_coordinates = self.chunk_id_to_chunk_coordinates(chunk_id)?;
        let block_coordinates = self.block_id_to_block_coordinates(block_id)?;

        Some(self.chunk_size as i32 * chunk_coordinates + block_coordinates)
    }

    pub fn position_to_ids(&self, position: IVec3) -> Option<(chunk::ID, block::ID)> {
        let chunk_id = self.position_to_chunk_id(position)?;
        let block_id = self.position_to_block_id(position)?;

        Some((chunk_id, block_id))
    }

    pub fn position_to_chunk_id(&self, position: IVec3) -> Option<chunk::ID> {
        let chunk_coordinates = self.position_to_chunk_coordinates(position)?;

        self.chunk_coordinates_to_chunk_id(chunk_coordinates)
    }

    pub fn position_to_block_id(&self, position: IVec3) -> Option<block::ID> {
        let block_coordinates = self.position_to_block_coordinates(position)?;

        self.block_coordinates_to_block_id(block_coordinates)
    }

    pub fn position_to_chunk_coordinates(&self, position: IVec3) -> Option<IVec3> {
        if self.position_valid(position) {
            let indexable_position = position + IVec3::splat(self.world_boundary as i32);

            Some(indexable_position / self.chunk_size as i32)
        } else {
            None
        }
    }

    pub fn position_to_block_coordinates(&self, position: IVec3) -> Option<IVec3> {
        let chunk_coordinates = self.position_to_chunk_coordinates(position)?;
        let chunk_position = self.chunk_coordinates_to_position(chunk_coordinates)?;

        Some(position - chunk_position)
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

    pub fn boundary_contact_directions(&self, position: IVec3) -> Vec<Direction> {
        let mut directions = Vec::new();

        if let Some(block_coordinates) = self.position_to_block_coordinates(position) {
            let chunk_radius = self.chunk_radius as i32;

            if block_coordinates.x == -chunk_radius {
                directions.push(Direction::XnYoZo);
            } else if block_coordinates.x == chunk_radius {
                directions.push(Direction::XpYoZo);
            }

            if block_coordinates.y == -chunk_radius {
                directions.push(Direction::XoYnZo);
            } else if block_coordinates.y == chunk_radius {
                directions.push(Direction::XoYpZo);
            }

            if block_coordinates.z == -chunk_radius {
                directions.push(Direction::XoYoZn);
            } else if block_coordinates.z == chunk_radius {
                directions.push(Direction::XoYoZp);
            }
        }

        directions
    }

    pub fn blocks_overlapping(&self, aabb: AABB) -> Vec<AABB> {
        let mut aabb_list = Vec::new();

        let min = aabb.min.round().as_ivec3();
        let max = aabb.max.round().as_ivec3();

        let size = Vec3::splat(BLOCK_SIZE);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let block_position = IVec3::new(x, y, z);
                    let block_aabb = AABB::new(block_position.as_vec3(), size);

                    if block_aabb.overlaps(aabb) {
                        aabb_list.push(block_aabb);
                    }
                }
            }
        }

        aabb_list
    }

    pub fn linearize(vector: IVec3, radius: u32) -> i32 {
        let size = 2 * radius + 1;
        let area = size * size;

        vector.x + vector.y * size as i32 + vector.z * area as i32
    }

    pub fn delinearize(index: u32, radius: u32) -> IVec3 {
        let index = index as i32;
        let radius = radius as i32;

        let size = 2 * radius + 1;
        let area = size * size;

        let x = (index % size) - radius;
        let y = (index / size % size) - radius;
        let z = (index / area) - radius;

        let vector = IVec3::new(x, y, z);

        println!("{:?}", vector);

        vector
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
