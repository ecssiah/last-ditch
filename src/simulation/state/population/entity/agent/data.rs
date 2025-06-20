use crate::simulation::state::{population::entity::Core, World};

pub struct Data {
    pub core: Core,
}

impl Data {
    pub fn new() -> Self {
        Self { core: Core::new() }
    }

    pub fn tick(&mut self, world: &World) {
        if let Some(chunk_id) = world
            .grid
            .world_to_chunk_id(self.core.kinematics.world_position)
        {
            self.core.chunk_id.set(chunk_id);
        }
    }
}
