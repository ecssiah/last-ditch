use crate::simulation::{
    observation::state_pair::StatePair,
    state::{
        population::entity::{self, Kinematics, Nation, Spatial},
        world::{chunk, World},
    },
};

pub struct Agent {
    pub id: entity::ID,
    pub chunk_id: StatePair<chunk::ID>,
    pub spatial: Spatial,
    pub kinematics: Kinematics,
    pub kind: entity::Kind,
    pub nation: Nation,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            id: entity::ID::allocate(),
            chunk_id: StatePair::new(chunk::ID::zero(), chunk::ID::zero()),
            spatial: Spatial::new(),
            kinematics: Kinematics::new(),
            kind: entity::Kind::Eagle,
            nation: Nation {
                kind: entity::Kind::Eagle,
            },
        }
    }

    pub fn tick(&mut self, world: &World) {
        if let Some(chunk_id) = world.grid.world_to_chunk_id(self.spatial.world_position) {
            self.chunk_id.set(chunk_id);
        }
    }

    pub fn chunk_updated(&self) -> bool {
        self.chunk_id.current != self.chunk_id.next
    }
}
