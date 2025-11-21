use crate::simulation::state::{
    population::entity::{self, nation, Spatial},
    world::{grid::Grid, sector},
    World,
};

#[derive(Debug)]
pub struct Info {
    pub entity_kind: entity::Kind,
    pub sector_id: sector::ID,
    pub sector_updated: bool,
    pub nation_kind: nation::Kind,
}

impl Info {
    pub fn tick(spatial: &Spatial, world: &World, info: &mut Info) {
        Self::update_sector_id(spatial, &world.grid, info);
    }

    pub fn update_sector_id(spatial: &Spatial, grid: &Grid, info: &mut Info) {
        let sector_id = Grid::world_position_to_sector_id(spatial.world_position, grid);

        if sector_id != info.sector_id {
            info.sector_updated = true;
            info.sector_id = sector_id;
        }
    }
}
