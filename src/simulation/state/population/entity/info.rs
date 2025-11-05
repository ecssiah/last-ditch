use crate::simulation::state::{
    population::{
        entity::{self, Spatial},
        nation,
    },
    world::{grid::Grid, sector},
    World,
};

#[derive(Debug)]
pub struct Info {
    pub entity_id: entity::ID,
    pub sector_id: sector::ID,
    pub sector_updated: bool,
    pub entity_kind: entity::Kind,
    pub nation_kind: nation::Kind,
}

impl Info {
    pub fn tick(world: &World, spatial: &Spatial, info: &mut Info) {
        Self::update_sector_id(&world.grid, spatial, info);
    }

    pub fn update_sector_id(grid: &Grid, spatial: &Spatial, info: &mut Info) {
        let sector_id = Grid::world_to_sector_id(grid, spatial.world_position);

        if sector_id != info.sector_id {
            info.sector_updated = true;
            info.sector_id = sector_id;
        }
    }
}
