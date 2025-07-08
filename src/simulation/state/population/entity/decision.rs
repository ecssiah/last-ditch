use glam::IVec3;

use crate::simulation::state::World;

#[derive(Default)]
pub struct Decision {}

impl Decision {
    pub fn new() -> Self {
        Self {}
    }

    pub fn tick(&mut self, world: &World) {
        // let graph_buffer = world.graph_buffer_lock.read().unwrap();
        // let mut graph = graph_buffer.get().clone();
    }
}
