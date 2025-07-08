use crate::simulation::state::{
    physics::aabb::AABB,
    population::entity::{self, Decision, Detection, Kinematic, Nation, Spatial},
    world::{chunk, World},
};
use glam::Vec3;

pub struct Agent {
    pub id: entity::ID,
    pub chunk_id: chunk::ID,
    pub chunk_updated: bool,
    pub decision: Decision,
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
            chunk_id: chunk::ID(0),
            chunk_updated: false,
            decision: Decision::new(),
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            detection: Detection::new(),
            kind: entity::Kind::Eagle,
            nation: Nation {
                kind: entity::Kind::Eagle,
            },
        }
    }

    pub fn tick(agent: &mut Agent, world: &World) {
        let chunk_id = world.grid.world_to_chunk_id(agent.spatial.world_position);

        if chunk_id != agent.chunk_id {
            agent.chunk_updated = true;
            agent.chunk_id = chunk_id;
        }
    }

    pub fn set_world_position(&mut self, world_position: Vec3) {
        self.spatial.world_position = world_position;
        self.detection.set_world_position(world_position);
    }

    pub fn set_size(&mut self, size: Vec3) {
        self.detection.body = AABB::new(self.detection.body.center(), size);
    }
}
