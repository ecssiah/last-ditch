use crate::simulation::state::{
    population::{entity::{self, Agent, Spatial}, nation},
    world::{chunk, grid::Grid},
    World,
};

#[derive(Debug)]
pub struct Info {
    pub entity_id: entity::ID,
    pub chunk_id: chunk::ID,
    pub chunk_updated: bool,
    pub entity_kind: entity::Kind,
    pub nation_kind: nation::Kind,
}

impl Info {
    pub fn tick(world: &World, agent: &mut Agent) {
        Self::update_chunk_id(&agent.spatial, &world.grid, &mut agent.info);
    }

    pub fn update_chunk_id(spatial: &Spatial, grid: &Grid, info: &mut Info) {
        let chunk_id = Grid::world_to_chunk_id(grid, spatial.world_position);

        if chunk_id != info.chunk_id {
            info.chunk_updated = true;
            info.chunk_id = chunk_id;
        }
    }
}
