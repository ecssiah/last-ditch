use std::{
    collections::{BinaryHeap, HashMap},
    sync::{Arc, RwLock},
};

use crate::simulation::state::{
    compute::{self, task},
    physics::aabb::AABB,
    population::entity::{
        self,
        decision::{plan, Plan},
        Decision, Detection, Info, Kinematic, Nation, Spatial,
    },
    time::Tick,
    world::{chunk, grid::Grid, World},
    Compute,
};
use glam::{IVec3, Vec3};
use rand::Rng;

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

        let idle_wait_tick_count = Tick::new(rand::thread_rng().gen_range(60..240));
        let idle_plan_data = plan::data::Idle::new(idle_wait_tick_count);

        let idle_plan = Plan::create_idle_plan();

        decision
            .plan_store
            .idle_data_map
            .insert(idle_plan.id, idle_plan_data);

        decision.active_plan_id = Some(idle_plan.id);
        decision.plan_map.insert(idle_plan.id, idle_plan);

        Self {
            info,
            decision,
            spatial: Spatial::new(),
            kinematic: Kinematic::new(),
            detection: Detection::new(),
        }
    }

    pub fn tick(world: &World, agent: &mut Agent, compute: &mut Compute) {
        Info::update_chunk_id(&agent.spatial, &world.grid, &mut agent.info);

        if let Some(plan) = agent
            .decision
            .active_plan_id
            .and_then(|plan_id| agent.decision.plan_map.get(&plan_id))
            .cloned()
        {
            Self::act(
                plan,
                world,
                &agent.info,
                &mut agent.spatial,
                &mut agent.decision,
                compute,
            );
        }
    }

    fn act(
        plan: Plan,
        world: &World,
        info: &Info,
        spatial: &mut Spatial,
        decision: &mut Decision,
        compute: &mut Compute,
    ) {
        match plan.kind {
            plan::Kind::Idle => Self::act_idle(
                plan,
                &mut decision.active_plan_id,
                &mut decision.plan_map,
                &mut decision.plan_store.idle_data_map,
                &mut decision.plan_store.travel_data_map,
            ),
            plan::Kind::Travel => Self::act_travel(
                plan,
                info,
                world,
                spatial,
                &mut decision.active_plan_id,
                &mut decision.plan_map,
                &mut decision.plan_store.travel_data_map,
                &mut compute.task_heap,
                &compute.task_store_arc_lock,
            ),
        }
    }

    fn act_idle(
        plan: Plan,
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
        idle_data_map: &mut HashMap<plan::ID, plan::data::Idle>,
        travel_data_map: &mut HashMap<plan::ID, plan::data::Travel>,
    ) {
        let idle_data = idle_data_map.get_mut(&plan.id).unwrap();

        match idle_data.state {
            plan::State::Init => Self::act_idle_init(idle_data),
            plan::State::Active => Self::act_idle_active(idle_data),
            plan::State::Success => {
                Self::act_idle_success(active_plan_id, plan_map, travel_data_map)
            }
            plan::State::Fail => todo!(),
            plan::State::Cancel => todo!(),
        }
    }

    fn act_idle_init(idle_data: &mut plan::data::Idle) {
        idle_data.state = plan::State::Active;
    }

    fn act_idle_active(idle_data: &mut plan::data::Idle) {
        idle_data.tick_count += 1;

        if idle_data.tick_count >= idle_data.duration {
            idle_data.state = plan::State::Success;
        }
    }

    fn act_idle_success(
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
        travel_data_map: &mut HashMap<plan::ID, plan::data::Travel>,
    ) {
        let travel_plan = Plan::create_travel_plan();
        let travel_data = plan::data::Travel::new();

        *active_plan_id = Some(travel_plan.id);

        travel_data_map.insert(travel_plan.id, travel_data);
        plan_map.insert(travel_plan.id, travel_plan);
    }

    fn act_travel(
        plan: Plan,
        info: &Info,
        world: &World,
        spatial: &mut Spatial,
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
        travel_data_map: &mut HashMap<plan::ID, plan::data::Travel>,
        task_heap: &mut BinaryHeap<compute::Task>,
        task_store_arc_lock: &Arc<RwLock<task::Store>>,
    ) {
        let travel_data = travel_data_map.get_mut(&plan.id).unwrap();

        match travel_data.state {
            plan::State::Init => Self::act_travel_init(
                plan,
                info,
                world,
                spatial,
                travel_data,
                task_heap,
                task_store_arc_lock,
            ),
            plan::State::Active => Self::act_travel_active(
                plan,
                info,
                world,
                spatial,
                travel_data,
                task_heap,
                task_store_arc_lock,
            ),
            plan::State::Success => Self::act_travel_success(active_plan_id, plan_map),
            plan::State::Fail => {
                println!("Travel Fail!");
            }
            plan::State::Cancel => {
                println!("Travel Cancel!");
            }
        }
    }

    fn act_travel_init(
        plan: Plan,
        info: &Info,
        world: &World,
        spatial: &mut Spatial,
        travel_data: &mut plan::data::Travel,
        task_heap: &mut BinaryHeap<compute::Task>,
        task_store_arc_lock: &Arc<RwLock<task::Store>>,
    ) {
        let (level_0_clone, search_level_clone) = {
            let graph_buffer = world.graph_buffer_lock.read().unwrap();
            let graph = graph_buffer.get();

            (graph.level_0.clone(), graph.level_vec[0].clone())
        };

        let task = compute::Task::new(
            compute::task::Priority::High,
            compute::task::Kind::PathRegion,
        );

        let end_position = match info.kind {
            entity::Kind::Lion => IVec3::new(-9, -3, 0),
            entity::Kind::Eagle => IVec3::new(9, -3, 0),
            entity::Kind::Wolf => IVec3::new(0, -3, -9),
            entity::Kind::Horse => IVec3::new(0, -3, 9),
        };

        let task_data = compute::task::data::path::Region {
            plan_id: plan.id,
            entity_id: info.entity_id,
            start_position: Grid::world_to_position(&world.grid, spatial.world_position),
            end_position,
            level_0: level_0_clone,
            search_level: search_level_clone,
        };

        {
            let mut task_store = task_store_arc_lock.write().unwrap();
            task_store.path_region_data_map.insert(task.id, task_data);
        }

        task_heap.push(task);

        travel_data.state = plan::State::Active;
    }

    fn act_travel_active(
        plan: Plan,
        info: &Info,
        world: &World,
        spatial: &mut Spatial,
        travel_data: &mut plan::data::Travel,
        task_heap: &mut BinaryHeap<compute::Task>,
        task_store_arc_lock: &Arc<RwLock<task::Store>>,
    ) {
        if travel_data.region_path_found {
            travel_data.region_path_found = false;
            travel_data.region_path_tracking = true;

            let task = compute::Task::new(
                compute::task::Priority::High,
                compute::task::Kind::PathLocal,
            );

            let level_0_clone = {
                let graph_buffer = world.graph_buffer_lock.read().unwrap();
                let graph = graph_buffer.get();

                graph.level_0.clone()
            };

            let start_position = travel_data.region_path_vec.pop().unwrap();
            let end_position = travel_data.region_path_vec.pop().unwrap();

            let task_data = compute::task::data::path::Local {
                plan_id: plan.id,
                entity_id: info.entity_id,
                chunk_id: chunk::ID::MAX,
                start_position,
                end_position,
                level_0: level_0_clone,
            };

            {
                let mut task_store = task_store_arc_lock.write().unwrap();
                task_store.path_local_data_map.insert(task.id, task_data);
            }

            task_heap.push(task);
        }

        if travel_data.local_path_found {
            travel_data.local_path_found = false;
            travel_data.local_path_tracking = true;
        }

        if travel_data.region_path_tracking {
            if travel_data.local_path_tracking {
                let target_position = travel_data.local_path_vec.last().unwrap().as_vec3();

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

                            let level_0_clone = {
                                let graph_buffer = world.graph_buffer_lock.read().unwrap();
                                let graph = graph_buffer.get();

                                graph.level_0.clone()
                            };

                            let start_position = travel_data.region_path_vec.pop();
                            let end_position = travel_data.region_path_vec.pop();

                            if start_position.is_some() && end_position.is_some() {
                                let task_data = compute::task::data::path::Local {
                                    plan_id: plan.id,
                                    entity_id: info.entity_id,
                                    chunk_id: chunk::ID::MAX,
                                    start_position: start_position.unwrap(),
                                    end_position: end_position.unwrap(),
                                    level_0: level_0_clone,
                                };

                                {
                                    let mut task_store = task_store_arc_lock.write().unwrap();
                                    task_store.path_local_data_map.insert(task.id, task_data);
                                }

                                task_heap.push(task);
                            } else {
                                travel_data.region_path_complete = true;

                                log::warn!("Missing region positions");
                            }
                        }
                    }
                }
            }
        }

        if travel_data.region_path_complete {
            travel_data.state = plan::State::Success;
        }
    }

    fn act_travel_success(
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
    ) {
        if let Some(active_plan_id) = active_plan_id {
            plan_map.remove(active_plan_id);
        }

        *active_plan_id = None;
    }

    pub fn set_world_position(
        world_position: Vec3,
        spatial: &mut Spatial,
        detection: &mut Detection,
    ) {
        spatial.world_position = world_position;

        Detection::set_world_position(world_position, &mut detection.body);
    }

    pub fn set_size(size: Vec3, detection: &mut Detection) {
        detection.body = AABB::new(detection.body.center(), size);
    }
}
