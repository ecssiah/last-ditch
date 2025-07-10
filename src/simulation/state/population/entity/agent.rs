use crate::simulation::state::{
    compute, physics::aabb::AABB, population::entity::{self, decision::plan, Decision, Detection, Kinematic, Nation, Spatial}, world::{chunk, World}
};
use glam::Vec3;

pub struct Agent {
    pub id: entity::ID,
    pub chunk_id: chunk::ID,
    pub chunk_updated: bool,
    pub task_vec: Vec<compute::Task>,
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
            task_vec: Vec::new(),
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

        while let Some(mut plan) = decision.plan_heap.pop() {
            match plan.kind {
                plan::Kind::Idle => {
                    if let Some(idle_data) = decision.plan_data.idle_data.get_mut(&plan.id) {
                        match plan.state {
                            plan::State::Init => todo!(),
                            plan::State::Active => {
                                idle_data.tick_count += 1;

                                if idle_data.tick_count >= idle_data.duration {
                                    plan.state = plan::State::Success;
                                    
                                    decision.plan_heap.push(plan);
                                }
                            },
                            plan::State::Success => todo!(),
                            plan::State::Fail => todo!(),
                            plan::State::Cancel => todo!(),
                        }
                    } else {
                        log::warn!("Plan ID: {:?} is missing idle::Data", plan.id);
                    }
                }
                plan::Kind::Travel => {
                    if let Some(travel_data) = decision.plan_data.travel_data.get_mut(&plan.id) {
                        match plan.state {
                            plan::State::Init => {

                            },
                            plan::State::Active => {
                                
                            },
                            plan::State::Success => todo!(),
                            plan::State::Fail => todo!(),
                            plan::State::Cancel => todo!(),
                        }
                    } else {
                        log::warn!("Plan ID: {:?} is missing travel::Data", plan.id);
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
