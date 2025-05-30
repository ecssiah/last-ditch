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

pub struct Grid {
    pub radius: u32,
    pub size: u32,
    pub area: u32,
    pub volume: u32,
    pub chunk_radius: u32,
    pub chunk_size: u32,
    pub chunk_area: u32,
    pub chunk_volume: u32,
    pub boundary: u32,
    pub block_id_max: u32,
    pub chunk_id_max: u32,
}

impl Grid {
    pub fn new(radius: u32, chunk_radius: u32) -> Self {
        let size = 2 * radius + 1;
        let area = size * size;
        let volume = size * size * size;

        let chunk_size = 2 * chunk_radius + 1;
        let chunk_area = chunk_size * chunk_size;
        let chunk_volume = chunk_size * chunk_size * chunk_size;

        let boundary = chunk_radius + radius * chunk_size;

        let block_id_max = chunk_volume - 1;
        let chunk_id_max = volume - 1;

        let grid = Grid {
            radius,
            size,
            area,
            volume,
            chunk_radius,
            chunk_size,
            chunk_area,
            chunk_volume,
            boundary,
            block_id_max,
            chunk_id_max,
        };

        grid
    }

    pub fn is_valid_grid_position(&self, grid_position: IVec3) -> bool {
        let in_x_range = grid_position.x.abs() as u32 <= self.boundary;
        let in_y_range = grid_position.y.abs() as u32 <= self.boundary;
        let in_z_range = grid_position.z.abs() as u32 <= self.boundary;

        in_x_range && in_y_range && in_z_range
    }

    pub fn is_valid_chunk_id(&self, chunk_id: chunk::ID) -> bool {
        (0..=self.chunk_id_max).contains(&u32::from(chunk_id))
    }

    pub fn is_valid_block_id(&self, block_id: block::ID) -> bool {
        (0..=self.block_id_max).contains(&u32::from(block_id))
    }

    pub fn ids_to_grid(&self, chunk_id: chunk::ID, block_id: block::ID) -> Option<IVec3> {
        let chunk_position = self.chunk_id_to_position(chunk_id)?;
        let block_position = self.block_id_to_position(block_id)?;

        let grid_position = self.chunk_size as i32 * chunk_position + block_position;

        Some(grid_position)
    }

    pub fn grid_to_ids(&self, grid_position: IVec3) -> Option<(chunk::ID, block::ID)> {
        let chunk_id = self.grid_to_chunk_id(grid_position)?;
        let block_id = self.grid_to_block_id(grid_position)?;

        Some((chunk_id, block_id))
    }

    pub fn grid_to_chunk_id(&self, grid_position: IVec3) -> Option<chunk::ID> {
        if self.is_valid_grid_position(grid_position) {
            let chunk_position = self.grid_to_chunk(grid_position)?;

            let x = chunk_position.x + self.radius as i32;
            let y = chunk_position.y + self.radius as i32;
            let z = chunk_position.z + self.radius as i32;

            let chunk_id = z * self.area as i32 + y * self.size as i32 + x;

            Some(chunk::ID(chunk_id as u32))
        } else {
            None
        }
    }

    pub fn grid_to_chunk(&self, grid_position: IVec3) -> Option<IVec3> {
        if self.is_valid_grid_position(grid_position) {
            let grid_position_shifted = grid_position + IVec3::splat(self.boundary as i32);

            let chunk_position = grid_position_shifted
                .map(|coordinate| coordinate.div_euclid(self.chunk_size as i32));

            Some(chunk_position - IVec3::splat(self.radius as i32))
        } else {
            None
        }
    }

    pub fn grid_to_block_id(&self, grid_position: IVec3) -> Option<block::ID> {
        if self.is_valid_grid_position(grid_position) {
            let block_position = self.grid_to_block(grid_position)?;

            let chunk_radius = self.chunk_radius as i32;
            let chunk_size = self.chunk_size as i32;
            let chunk_area = self.chunk_area as i32;

            let block_id = (block_position.x + chunk_radius)
                + (block_position.y + chunk_radius) * chunk_size
                + (block_position.z + chunk_radius) * chunk_area;

            Some(block::ID(block_id as u32))
        } else {
            None
        }
    }

    pub fn grid_to_block(&self, grid_position: IVec3) -> Option<IVec3> {
        if self.is_valid_grid_position(grid_position) {
            let grid_position_shifted = grid_position + IVec3::splat(self.boundary as i32);

            let chunk_position = grid_position_shifted
                .map(|coordinate| coordinate.div_euclid(self.chunk_size as i32));
            let chunk_center = chunk_position * self.chunk_size as i32;

            let block_position = grid_position_shifted - chunk_center;
            let block_position = block_position - IVec3::splat(self.chunk_radius as i32);

            Some(block_position)
        } else {
            None
        }
    }

    pub fn world_to_chunk_id(&self, world_position: Vec3) -> Option<chunk::ID> {
        let grid_position = self.world_to_grid(world_position)?;
        let chunk_id = self.grid_to_chunk_id(grid_position)?;

        Some(chunk_id)
    }

    pub fn world_to_grid(&self, world_position: Vec3) -> Option<IVec3> {
        let grid_position = (world_position + Vec3::splat(0.5)).floor().as_ivec3();

        if self.is_valid_grid_position(grid_position) {
            Some(grid_position)
        } else {
            None
        }
    }

    pub fn chunk_to_grid(&self, chunk_position: IVec3) -> Option<IVec3> {
        let grid_position = chunk_position * self.chunk_size as i32;

        if self.is_valid_grid_position(grid_position) {
            Some(grid_position)
        } else {
            None
        }
    }

    pub fn chunk_id_to_position(&self, chunk_id: chunk::ID) -> Option<IVec3> {
        if self.is_valid_chunk_id(chunk_id) {
            let chunk_index = i32::from(chunk_id);

            let radius = self.radius as i32;
            let size = self.size as i32;
            let area = self.area as i32;

            let x = (chunk_index % size) - radius;
            let y = (chunk_index / size % size) - radius;
            let z = (chunk_index / area) - radius;

            let local_position = IVec3::new(x, y, z);

            Some(local_position)
        } else {
            None
        }
    }

    pub fn block_id_to_position(&self, block_id: block::ID) -> Option<IVec3> {
        if self.is_valid_block_id(block_id) {
            let block_id = i32::from(block_id);

            let chunk_radius = self.chunk_radius as i32;
            let chunk_size = self.chunk_size as i32;
            let chunk_area = self.chunk_area as i32;

            let x = (block_id % chunk_size) - chunk_radius;
            let y = (block_id / chunk_size % chunk_size) - chunk_radius;
            let z = (block_id / chunk_area) - chunk_radius;

            Some(IVec3::new(x, y, z))
        } else {
            None
        }
    }

    pub fn overlapping_aabb_list(&self, aabb: AABB) -> Vec<AABB> {
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
}
