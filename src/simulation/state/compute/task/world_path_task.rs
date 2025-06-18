use crate::simulation::state::{
    compute::{
        result::world_path_result::WorldPathResult,
        snapshot::world_path_snapshot::WorldPathSnapshot, Task,
    },
    population::{agent, Population},
    world::World,
};

#[derive(Debug)]
pub struct WorldPathTask {
    pub agent_id: agent::ID,
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
