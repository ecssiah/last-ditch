pub mod id;
pub mod mesh;
pub mod vertex;

pub use id::ID;
pub use mesh::Mesh;
pub use vertex::Vertex;

use crate::simulation::{block, chunk, consts::*, time::Tick, world::World, BLOCKS};
use glam::{IVec3, Vec3};

pub struct Chunk {
    pub id: chunk::ID,
    pub tick: Tick,
    pub updated: bool,
    pub position: IVec3,
    pub palette: Vec<block::Kind>,
    pub blocks: Box<[usize; CHUNK_VOLUME]>,
    pub meta: Box<[block::Meta; CHUNK_VOLUME]>,
    pub light: Box<[block::Light; CHUNK_VOLUME]>,
    pub mesh: chunk::Mesh,
}

impl Chunk {
    pub fn get_block(&self, block_id: block::ID) -> Option<&block::Block> {
        if block::ID::valid(block_id) {
            let palette_id = self.blocks.get(usize::from(block_id))?;
            let kind = self.palette.get(usize::from(*palette_id))?;

            let block = BLOCKS.get(&kind)?;

            Some(block)
        } else {
            None
        }
    }

    pub fn get_meta(&self, block_id: block::ID) -> Option<&block::Meta> {
        let meta = self.meta.get(usize::from(block_id))?;

        Some(meta)
    }

    pub fn get_meta_mut(&mut self, block_id: block::ID) -> Option<&mut block::Meta> {
        let meta = self.meta.get_mut(usize::from(block_id))?;

        Some(meta)
    }

    pub fn on_map(position: IVec3) -> bool {
        let in_x_range = position.x.abs() <= WORLD_RADIUS as i32;
        let in_y_range = position.y.abs() <= WORLD_RADIUS as i32;
        let in_z_range = position.z.abs() <= WORLD_RADIUS as i32;

        in_x_range && in_y_range && in_z_range
    }

    pub fn id_at(position: IVec3) -> Option<chunk::ID> {
        if Self::on_map(position) {
            let position_shift = position + IVec3::splat(WORLD_RADIUS as i32);
            
            let chunk_id = position_shift.x
                + position_shift.y * WORLD_SIZE as i32
                + position_shift.z * WORLD_AREA as i32;
    
            let chunk_id = chunk::ID(chunk_id as usize);
    
            Some(chunk_id)
        } else {
            None
        }
    }

    pub fn id_at_grid(grid_position: IVec3) -> Option<chunk::ID> {
        if World::on_map(grid_position) {
            let chunk_position = grid_position.map(|coordinate| {
                let coordinate = coordinate + WORLD_BOUNDARY as i32;

                coordinate.div_euclid(CHUNK_SIZE as i32)
            });

            let chunk_id = chunk_position.x
                + chunk_position.y * WORLD_SIZE as i32
                + chunk_position.z * WORLD_AREA as i32;

            let chunk_id = chunk::ID(chunk_id as usize);

            Some(chunk_id)
        } else {
            None
        }
    }

    pub fn position(chunk_id: chunk::ID) -> Option<IVec3> {
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

    pub fn position_at(grid_position: IVec3) -> Option<IVec3> {
        let chunk_id = Self::id_at_grid(grid_position)?;
        let position = Self::position(chunk_id)?;

        Some(position)
    }

    pub fn world_position(chunk_id: chunk::ID) -> Option<Vec3> {
        let position = Self::position(chunk_id)?;
        let world_position = position.as_vec3() * CHUNK_SIZE as f32;

        Some(world_position)
    }
}
