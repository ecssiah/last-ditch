use crate::simulation::state::{
    compute::{self},
    physics::aabb::AABB,
    population::entity::{
        self,
        decision::{plan, Plan},
        Decision, Detection, Kinematic, Nation, Spatial,
    },
    time::Tick,
    world::{chunk, World},
    Compute,
};
use glam::{IVec3, Vec3};

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

    pub fn tick(agent: &mut Agent, compute: &mut Compute, world: &World) {
        Self::track_current_chunk(agent, world);
        Self::act(agent, compute, world);
    }

    fn track_current_chunk(agent: &mut Agent, world: &World) {
        let chunk_id = world.grid.world_to_chunk_id(agent.spatial.world_position);

        if chunk_id != agent.chunk_id {
            agent.chunk_updated = true;
            agent.chunk_id = chunk_id;
        }
    }

    fn act(agent: &mut Agent, compute: &mut Compute, world: &World) {
        if agent.decision.plan_heap.is_empty() {
            let idle_plan = Plan::new(plan::Priority::High, plan::Kind::Idle);
            let idle_plan_data = plan::data::Idle::new(Tick::new(60));

            agent
                .decision
                .plan_store
                .idle_data_map
                .insert(idle_plan.id, idle_plan_data);

            agent.decision.plan_heap.push(idle_plan);
        }

        let mut current_plans: Vec<_> = agent.decision.plan_heap.drain().collect();

        while let Some(plan) = current_plans.pop() {
            match plan.kind {
                plan::Kind::Idle => {
                    let plan_data = agent
                        .decision
                        .plan_store
                        .idle_data_map
                        .get_mut(&plan.id)
                        .unwrap();

                    match plan_data.state {
                        plan::State::Init => {
                            plan_data.state = plan::State::Active;

                            agent.decision.plan_heap.push(plan);
                        }
                        plan::State::Active => {
                            plan_data.tick_count += 1;

                            if plan_data.tick_count >= plan_data.duration {
                                plan_data.state = plan::State::Success;
                            }

                            agent.decision.plan_heap.push(plan);
                        }
                        plan::State::Success => {
                            let travel_plan = Plan::new(plan::Priority::High, plan::Kind::Travel);
                            let travel_data = plan::data::Travel::new();

                            agent
                                .decision
                                .plan_store
                                .travel_data_map
                                .insert(travel_plan.id, travel_data);

                            agent.decision.plan_heap.push(travel_plan);
                        }
                        plan::State::Fail => todo!(),
                        plan::State::Cancel => todo!(),
                    }
                }
                plan::Kind::Travel => {
                    let plan_data = agent
                        .decision
                        .plan_store
                        .travel_data_map
                        .get_mut(&plan.id)
                        .unwrap();

                    match plan_data.state {
                        plan::State::Init => {
                            let graph_buffer = world.graph_buffer_lock.read().unwrap();
                            let graph = graph_buffer.get();

                            let task = compute::Task::new(
                                compute::task::Priority::High,
                                compute::task::Kind::PathRegion,
                            );

                            let task_data = compute::task::data::path::Region {
                                plan_id: plan.id,
                                agent_id: agent.id,
                                start_position: world
                                    .grid
                                    .world_to_position(agent.spatial.world_position),
                                end_position: IVec3::new(0, 6, 9),
                                level_0: graph.level_0.clone(),
                                search_level: graph.level_vec[0].clone(),
                            };

                            let mut task_store = compute.task_store_arc_lock.write().unwrap();
                            task_store.path_region_data_map.insert(task.id, task_data);

                            compute.task_heap.push(task);

                            plan_data.state = plan::State::Active;

                            agent.decision.plan_heap.push(plan);
                        }
                        plan::State::Active => {
                            if !plan_data.region_path_vec.is_empty() {
                                let target_position = plan_data.region_path_vec.last().unwrap();

                                let distance_vector =
                                    target_position.as_vec3() - agent.spatial.world_position;

                                if distance_vector.length_squared() >= 0.01 {
                                    let direction_vector = distance_vector.normalize();

                                    agent.set_world_position(
                                        agent.spatial.world_position + 0.4 * direction_vector,
                                    );
                                } else {
                                    plan_data.region_path_vec.pop();

                                    if plan_data.region_path_vec.is_empty() {
                                        plan_data.state = plan::State::Success;
                                    }
                                }
                            }

                            agent.decision.plan_heap.push(plan);
                        }
                        plan::State::Success => {
                            println!("Travel Success!");
                        }
                        plan::State::Fail => todo!(),
                        plan::State::Cancel => todo!(),
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
