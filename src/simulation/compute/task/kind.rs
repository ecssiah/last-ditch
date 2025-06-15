use crate::simulation::{
    compute::{
        result,
        task::{chunk_path_task::ChunkPathTask, world_path_task::WorldPathTask},
        Task,
    },
    population::Population,
    world::World,
};
use crossbeam::channel::Sender;

pub enum Kind {
    WorldPath(WorldPathTask),
    ChunkPath(ChunkPathTask),
}

impl Kind {
    pub fn spawn(self, world: &World, population: &Population, tx: &Sender<result::Kind>) {
        match self {
            Kind::ChunkPath(task) => {
                let snapshot = task.snapshot(world, population);
                let result = task.execute(snapshot);

                let _ = tx.send(result::Kind::ChunkPath(result));
            }
            Kind::WorldPath(task) => {
                let snapshot = task.snapshot(world, population);
                let result = task.execute(snapshot);

                let _ = tx.send(result::Kind::WorldPath(result));
            }
        }
    }
}
