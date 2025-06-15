use crate::simulation::{
    compute::{
        result::world_path_result::WorldPathResult,
        snapshot::world_path_snapshot::WorldPathSnapshot, Task,
    },
    population::{agent, Population},
    world::World,
};

pub struct WorldPathTask {
    pub agent_id: agent::ID,
}

impl Task for WorldPathTask {
    type Snapshot = WorldPathSnapshot;
    type Result = WorldPathResult;

    fn snapshot(&self, world: &World, population: &Population) -> Self::Snapshot {
        todo!()
    }

    fn execute(self, snapshot: Self::Snapshot) -> Self::Result {
        todo!()
    }
}
