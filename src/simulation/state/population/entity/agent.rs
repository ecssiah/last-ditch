use crate::simulation::{
    observation::state_pair::StatePair,
    state::{
        physics::aabb::AABB,
        population::entity::{self, Detection, Kinematic, Nation, Spatial},
        world::{chunk, World},
    },
};
use glam::Vec3;

pub struct Agent {
    pub id: entity::ID,
    pub chunk_id: StatePair<chunk::ID>,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
    pub kind: entity::Kind,
    pub nation: Nation,
}

impl Agent {
    pub fn new() -> Self {
        Self {
            id: entity::ID::allocate(),
            chunk_id: StatePair::default(),
            spatial: Spatial::default(),
            kinematic: Kinematic::default(),
            detection: Detection::default(),
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

    pub fn set_world_position(&mut self, world_position: Vec3) {
        self.spatial.world_position = world_position;
        self.detection.set_world_position(world_position);
    }

    pub fn set_size(&mut self, size: Vec3) {
        self.detection.body = AABB::new(self.detection.body.center(), size);
    }
}

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
}
