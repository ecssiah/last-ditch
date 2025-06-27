use crate::simulation::state::World;

pub fn construct(world: &mut World) {
    world.update_chunks();
}
