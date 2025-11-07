use crate::simulation::state::World;

pub fn run(world: &mut World) {
    World::update_sectors(&world.grid, &mut world.sector_vec);
}
