use crate::simulation::state::{
    compute::{
        result::world_path_result::WorldPathResult,
        snapshot::world_path_snapshot::WorldPathSnapshot, Task,
    },
    population::{entity, Population},
    world::World,
};

#[derive(Debug)]
pub struct WorldPathTask {
    pub agent_id: entity::ID,
}

impl Task for WorldPathTask {
    type Snapshot = WorldPathSnapshot;
    type Result = WorldPathResult;

    fn snapshot(&self, _world: &World, _population: &Population) -> Self::Snapshot {
        todo!()
    }

    fn execute(self, _snapshot: Self::Snapshot) -> Self::Result {
        todo!()
    }
}
