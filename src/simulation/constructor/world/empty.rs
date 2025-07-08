use crate::simulation::state::World;

pub fn construct(world: &mut World) {
    World::update_chunks(&world.grid, &mut world.chunk_vec);
}
