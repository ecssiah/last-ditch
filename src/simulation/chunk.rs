use crate::simulation::{
    block,
    id::{block_id::BlockID, chunk_id::ChunkID, palette_id::PaletteID},
    time::Tick,
    world::World,
    Simulation, BLOCKS, CHUNK_SIZE, CHUNK_VOLUME, WORLD_AREA, WORLD_BOUNDARY, WORLD_RADIUS,
    WORLD_SIZE,
};
use glam::{IVec3, Vec3};

pub mod mesh;
pub mod vertex;

pub struct Chunk {
    pub last_update: Tick,
    pub id: ChunkID,
    pub position: IVec3,
    pub palette: Vec<block::Kind>,
    pub palette_ids: Vec<PaletteID>,
    pub meta: Box<[block::Meta; CHUNK_VOLUME]>,
    pub light: Box<[block::LightLevel; CHUNK_VOLUME]>,
    pub mesh: mesh::Mesh,
}

impl Chunk {
    pub fn get_block(&self, block_id: BlockID) -> Option<&block::Block> {
        let palette_id = self.palette_ids.get(usize::from(block_id))?;
        let kind = self.palette.get(usize::from(*palette_id))?;

        let block = BLOCKS.get(&kind)?;

        Some(block)
    }

    pub fn get_meta(&self, block_id: BlockID) -> Option<&block::Meta> {
        let meta = self.meta.get(usize::from(block_id))?;

        Some(meta)
    }

    pub fn get_meta_mut(&mut self, block_id: BlockID) -> Option<&mut block::Meta> {
        let meta = self.meta.get_mut(usize::from(block_id))?;

        Some(meta)
    }

    pub fn local_position(chunk_id: ChunkID) -> IVec3 {
        let chunk_id: usize = usize::from(chunk_id);

        let chunk_position_shifted = IVec3::new(
            (chunk_id % WORLD_SIZE) as i32,
            (chunk_id / WORLD_SIZE % WORLD_SIZE) as i32,
            (chunk_id / WORLD_AREA) as i32,
        );

        let chunk_position = chunk_position_shifted - IVec3::splat(WORLD_RADIUS as i32);

        chunk_position
    }

    pub fn world_position(chunk_id: ChunkID) -> Vec3 {
        let grid_position = Self::local_position(chunk_id);
        let world_position = grid_position.as_vec3() * CHUNK_SIZE as f32;

        world_position
    }

    pub fn id_at(grid_position: IVec3) -> Option<ChunkID> {
        if World::on_map(grid_position) {
            let chunk_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.div_euclid(WORLD_SIZE as i32)
            });

            let chunk_id = chunk_position_shifted.x
                + chunk_position_shifted.y * WORLD_SIZE as i32
                + chunk_position_shifted.z * WORLD_AREA as i32;

            let chunk_id = ChunkID(chunk_id as usize);

            Some(chunk_id)
        } else {
            None
        }
    }
}
