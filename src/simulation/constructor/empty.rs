use crate::simulation::state::{Population, World};

pub fn construct_world(world: &mut World) {
    world.update_chunks();
}

pub fn construct_population(_population: &mut Population, _world: &World) {}
