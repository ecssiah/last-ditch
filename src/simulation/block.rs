use crate::simulation::{consts::*, id::block_id::BlockID, world::World};
use glam::IVec3;
use serde::Deserialize;

pub mod direction;
pub mod face;
pub mod kind;
pub mod light;
pub mod meta;
pub mod neighbors;

pub use direction::Direction;
pub use face::Face;
pub use kind::Kind;
pub use light::Light;
pub use meta::Meta;
pub use neighbors::Neighbors;

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub emittance: u8,
    pub solid: bool,
    pub color: (f32, f32, f32, f32),
}

impl Block {
    pub fn local_position(block_id: BlockID) -> IVec3 {
        let block_id = usize::from(block_id);

        let block_position_shifted = IVec3::new(
            (block_id % CHUNK_SIZE) as i32,
            (block_id / CHUNK_SIZE % CHUNK_SIZE) as i32,
            (block_id / CHUNK_AREA) as i32,
        );

        let block_position = block_position_shifted - IVec3::splat(CHUNK_RADIUS as i32);

        block_position
    }

    pub fn id_at(grid_position: IVec3) -> Option<BlockID> {
        if World::on_map(grid_position) {
            let grid_position_shifted = grid_position.map(|coordinate| {
                let coordinate_shifted = coordinate + WORLD_BOUNDARY as i32;

                coordinate_shifted.rem_euclid(CHUNK_SIZE as i32)
            });

            let block_id = grid_position_shifted.x
                + grid_position_shifted.y * CHUNK_SIZE as i32
                + grid_position_shifted.z * CHUNK_AREA as i32;

            Some(BlockID(block_id as usize))
        } else {
            None
        }
    }
}
