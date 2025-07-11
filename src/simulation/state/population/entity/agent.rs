use crate::simulation::state::{
    compute::{self},
    physics::aabb::AABB,
    population::entity::{
        self,
        decision::{plan, Plan},
        Decision, Detection, Info, Kinematic, Nation, Spatial,
    },
    time::Tick,
    world::{chunk, World},
    Compute,
};
use glam::{IVec3, Vec3};

pub struct Agent {
    pub info: Info,
    pub decision: Decision,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
}

impl Agent {
    pub fn new() -> Self {
        let info = Info {
            entity_id: entity::ID::allocate(),
            chunk_id: chunk::ID(0),
            chunk_updated: false,
            kind: entity::Kind::Eagle,
            nation: Nation {
                kind: entity::Kind::Eagle,
            },
        };

        let mut decision = Decision::new();

        let idle_plan = Plan::new(plan::Priority::High, plan::Kind::Idle);
        let idle_plan_data = plan::data::Idle::new(Tick::new(40));

        decision
            .plan_store
            .idle_data_map
            .insert(idle_plan.id, idle_plan_data);

        decision.plan_heap.push(idle_plan);

        Self {
            info,
            decision,
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            detection: Detection::new(),
        }
    }

    pub fn tick(agent: &mut Agent, compute: &mut Compute, world: &World) {
        Self::track_current_chunk(&mut agent.info, &mut agent.spatial, world);

        Self::act(
            &agent.info,
            &mut agent.spatial,
            &mut agent.decision,
            compute,
            world,
        );
    }

    fn track_current_chunk(info: &mut Info, spatial: &Spatial, world: &World) {
        let chunk_id = world.grid.world_to_chunk_id(spatial.world_position);

        if chunk_id != info.chunk_id {
            info.chunk_updated = true;
            info.chunk_id = chunk_id;
        }
    }

    fn act(
        info: &Info,
        spatial: &mut Spatial,
        decision: &mut Decision,
        compute: &mut Compute,
        world: &World,
    ) {
        let mut current_plans: Vec<_> = decision.plan_heap.drain().collect();

        while let Some(plan) = current_plans.pop() {
            match plan.kind {
                plan::Kind::Idle => {
                    let plan_data = decision.plan_store.idle_data_map.get_mut(&plan.id).unwrap();

                    match plan_data.state {
                        plan::State::Init => {
                            plan_data.state = plan::State::Active;

                            decision.plan_heap.push(plan);
                        }
                        plan::State::Active => {
                            plan_data.tick_count += 1;

                            if plan_data.tick_count >= plan_data.duration {
                                plan_data.state = plan::State::Success;
                            }

                            decision.plan_heap.push(plan);
                        }
                        plan::State::Success => {
                            let travel_plan = Plan::new(plan::Priority::High, plan::Kind::Travel);
                            let travel_data = plan::data::Travel::new();

                            decision
                                .plan_store
                                .travel_data_map
                                .insert(travel_plan.id, travel_data);

                            decision.plan_heap.push(travel_plan);
                        }
                        plan::State::Fail => todo!(),
                        plan::State::Cancel => todo!(),
                    }
                }
                plan::Kind::Travel => {
                    let travel_data = decision
                        .plan_store
                        .travel_data_map
                        .get_mut(&plan.id)
                        .unwrap();

                    match travel_data.state {
                        plan::State::Init => {
                            let graph_buffer = world.graph_buffer_lock.read().unwrap();
                            let graph = graph_buffer.get();

                            let task = compute::Task::new(
                                compute::task::Priority::High,
                                compute::task::Kind::PathRegion,
                            );

                            let task_data = compute::task::data::path::Region {
                                plan_id: plan.id,
                                entity_id: info.entity_id,
                                start_position: world
                                    .grid
                                    .world_to_position(spatial.world_position),
                                end_position: IVec3::new(0, 6, 9),
                                level_0: graph.level_0.clone(),
                                search_level: graph.level_vec[0].clone(),
                            };

                            let mut task_store = compute.task_store_arc_lock.write().unwrap();
                            task_store.path_region_data_map.insert(task.id, task_data);

                            compute.task_heap.push(task);

                            travel_data.state = plan::State::Active;

                            decision.plan_heap.push(plan);
                        }
                        plan::State::Active => {
                            if travel_data.region_path_found {
                                travel_data.region_path_found = false;
                                travel_data.region_path_tracking = true;

                                let task = compute::Task::new(
                                    compute::task::Priority::High,
                                    compute::task::Kind::PathLocal,
                                );

                                let task_data = {
                                    let graph_buffer = world.graph_buffer_lock.read().unwrap();
                                    let graph = graph_buffer.get();

                                    let start_position = travel_data.region_path_vec.pop().unwrap();
                                    let end_position = travel_data.region_path_vec.pop().unwrap();

                                    compute::task::data::path::Local {
                                        plan_id: plan.id,
                                        entity_id: info.entity_id,
                                        chunk_id: chunk::ID::MAX,
                                        start_position,
                                        end_position,
                                        level_0: graph.level_0.clone(),
                                    }
                                };

                                let mut task_store = compute.task_store_arc_lock.write().unwrap();

                                task_store.path_local_data_map.insert(task.id, task_data);

                                compute.task_heap.push(task);
                            }

                            if travel_data.local_path_found {
                                travel_data.local_path_found = false;
                                travel_data.local_path_tracking = true;
                            }

                            if travel_data.region_path_tracking {
                                if travel_data.local_path_tracking {
                                    let target_position =
                                        travel_data.local_path_vec.last().unwrap().as_vec3();

                                    let distance_vector = target_position - spatial.world_position;

                                    if distance_vector.length_squared() >= 0.01 {
                                        let direction_vector = distance_vector.normalize();

                                        spatial.world_position += 0.06 * direction_vector;
                                    } else {
                                        travel_data.local_path_vec.pop();

                                        if travel_data.local_path_vec.is_empty() {
                                            if travel_data.region_path_vec.is_empty() {
                                                travel_data.state = plan::State::Success;
                                                travel_data.region_path_tracking = false;
                                                travel_data.region_path_complete = true;
                                            } else {
                                                travel_data.local_path_tracking = false;

                                                let task = compute::Task::new(
                                                    compute::task::Priority::High,
                                                    compute::task::Kind::PathLocal,
                                                );

                                                let task_data = {
                                                    let graph_buffer =
                                                        world.graph_buffer_lock.read().unwrap();
                                                    let graph = graph_buffer.get();

                                                    let start_position =
                                                        travel_data.region_path_vec.pop().unwrap();
                                                    let end_position =
                                                        travel_data.region_path_vec.pop().unwrap();

                                                    compute::task::data::path::Local {
                                                        plan_id: plan.id,
                                                        entity_id: info.entity_id,
                                                        chunk_id: chunk::ID::MAX,
                                                        start_position,
                                                        end_position,
                                                        level_0: graph.level_0.clone(),
                                                    }
                                                };

                                                let mut task_store =
                                                    compute.task_store_arc_lock.write().unwrap();

                                                task_store
                                                    .path_local_data_map
                                                    .insert(task.id, task_data);

                                                compute.task_heap.push(task);
                                            }
                                        }
                                    }
                                }
                            }

                            if !travel_data.region_path_complete {
                                decision.plan_heap.push(plan);
                            }
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
