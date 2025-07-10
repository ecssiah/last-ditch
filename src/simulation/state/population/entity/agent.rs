use crate::simulation::state::{
    physics::aabb::AABB,
    population::entity::{self, decision::plan, Decision, Detection, Kinematic, Nation, Spatial},
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
        Self::track_current_chunk(agent, world);

        Decision::tick(&mut agent.decision, world);

        Self::act(&mut agent.decision);
    }

    fn track_current_chunk(agent: &mut Agent, world: &World) {
        let chunk_id = world.grid.world_to_chunk_id(agent.spatial.world_position);

        if chunk_id != agent.chunk_id {
            agent.chunk_updated = true;
            agent.chunk_id = chunk_id;
        }
    }

    fn act(decision: &mut Decision) {
        let mut action_count = 0;

        for priority in plan::Priority::ALL_ARRAY {
            if let Some(plan_vec) = decision.plan_map.get(&priority) {
                if !plan_vec.is_empty() {
                    for plan in plan_vec {


                        action_count += 1;

                        if action_count > decision.max_actions {
                            return;
                        }
                    }
                }
            }
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
