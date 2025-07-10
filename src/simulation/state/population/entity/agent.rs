use crate::simulation::state::{
    compute::{self},
    physics::aabb::AABB,
    population::entity::{self, decision::plan, Decision, Detection, Kinematic, Nation, Spatial},
    world::{chunk, World},
};
use glam::{IVec3, Vec3};

pub struct Agent {
    pub id: entity::ID,
    pub chunk_id: chunk::ID,
    pub chunk_updated: bool,
    pub result_vec: Vec<compute::Result>,
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
            result_vec: Vec::new(),
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

    pub fn tick(agent: &mut Agent, task_vec: &mut Vec<compute::Task>, world: &World) {
        Self::track_current_chunk(agent, world);
        Self::act(agent, task_vec, world);
    }

    fn track_current_chunk(agent: &mut Agent, world: &World) {
        let chunk_id = world.grid.world_to_chunk_id(agent.spatial.world_position);

        if chunk_id != agent.chunk_id {
            agent.chunk_updated = true;
            agent.chunk_id = chunk_id;
        }
    }

    fn act(agent: &mut Agent, task_vec: &mut Vec<compute::Task>, world: &World) {
        let mut action_count = 0;

        while let Some(mut plan) = agent.decision.plan_heap.pop() {
            match plan.kind {
                plan::Kind::Idle => match plan.state {
                    plan::State::Init => todo!(),
                    plan::State::Active => {
                        if let Some(idle_data) =
                            agent.decision.plan_data.idle_data.get_mut(&plan.id)
                        {
                            idle_data.tick_count += 1;

                            if idle_data.tick_count >= idle_data.duration {
                                plan.state = plan::State::Success;

                                agent.decision.plan_heap.push(plan);
                            }
                        } else {
                            log::warn!("Plan ID: {:?} is missing idle::Data", plan.id);
                        }
                    }
                    plan::State::Success => todo!(),
                    plan::State::Fail => todo!(),
                    plan::State::Cancel => todo!(),
                },
                plan::Kind::Travel => match plan.state {
                    plan::State::Init => {
                        let data_regional = compute::task::path::data::Regional {
                            agent_id: agent.id,
                            start_position: world
                                .grid
                                .world_to_position(agent.spatial.world_position),
                            end_position: IVec3::new(0, -3, 0),
                        };

                        let path_data_kind = compute::task::path::Data::Regional(data_regional);

                        let task = compute::Task::Path(path_data_kind);

                        task_vec.push(task);
                    }
                    plan::State::Active => {
                        if let Some(travel_data) =
                            agent.decision.plan_data.travel_data.get_mut(&plan.id)
                        {
                        } else {
                            log::warn!("Plan ID: {:?} is missing travel::Data", plan.id);
                        }
                    }
                    plan::State::Success => todo!(),
                    plan::State::Fail => todo!(),
                    plan::State::Cancel => todo!(),
                },
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
