use crate::simulation::state::{
    population::entity::{self, Spatial},
    world::{chunk, grid::Grid},
};

#[derive(Debug)]
pub struct Info {
    pub entity_id: entity::ID,
    pub chunk_id: chunk::ID,
    pub chunk_updated: bool,
    pub kind: entity::Kind,
    pub nation: entity::Nation,
}

impl Info {
    pub fn update_chunk_id(spatial: &Spatial, grid: &Grid, info: &mut Info) {
        let chunk_id = Grid::world_to_chunk_id(grid, spatial.world_position);

        if chunk_id != info.chunk_id {
            info.chunk_updated = true;
            info.chunk_id = chunk_id;
        }
    }
}
