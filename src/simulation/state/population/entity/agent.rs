use crate::simulation::{
    consts::*,
    state::{
        compute::task,
        physics::aabb::AABB,
        population::{
            entity::{
                self,
                decision::{
                    plan::{
                        self,
                        data::{IdleData, TravelData},
                    },
                    Plan,
                },
                Decision, Detection, Info, Kinematic, Spatial,
            },
            nation,
        },
        world::{
            chunk,
            graph::{edge, Edge},
            grid::Grid,
            World,
        },
        Compute,
    },
};
use glam::{IVec3, Vec3};
use rand::Rng;
use std::{
    collections::{BinaryHeap, HashMap},
    sync::{Arc, RwLock},
};

pub struct Agent {
    pub info: Info,
    pub decision: Decision,
    pub spatial: Spatial,
    pub kinematic: Kinematic,
    pub detection: Detection,
}

impl Agent {
    pub fn new(nation_kind: nation::Kind) -> Self {
        let info = Info {
            entity_id: entity::ID::allocate(),
            chunk_id: chunk::ID(0),
            chunk_updated: false,
            entity_kind: entity::Kind::Agent,
            nation_kind,
        };

        let mut decision = Decision::new();

        let tick_duration = rand::thread_rng().gen_range(60..240);

        let (idle_plan, idle_data) = Plan::create_idle_plan(tick_duration);

        decision
            .plan_store
            .idle_data_map
            .insert(idle_plan.id, idle_data);

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

    pub fn tick(world: &World, agent: &mut Agent, compute: &mut Compute) {
        Info::tick(world, agent);
        Decision::tick(world, agent);

        if let Some(plan) = agent
            .decision
            .active_plan_id
            .and_then(|plan_id| agent.decision.plan_map.get(&plan_id))
            .cloned()
        {
            match plan.kind {
                plan::Kind::Idle => Self::follow_idle_plan(
                    plan,
                    &agent.info,
                    &mut agent.decision.active_plan_id,
                    &mut agent.decision.plan_map,
                    &mut agent.decision.plan_store.idle_data_map,
                    &mut agent.decision.plan_store.travel_data_map,
                ),
                plan::Kind::Travel => Self::follow_travel_plan(
                    plan,
                    world,
                    &agent.info,
                    &agent.kinematic,
                    &mut agent.spatial,
                    &mut agent.decision.active_plan_id,
                    &mut agent.decision.plan_map,
                    &mut agent.decision.plan_store.travel_data_map,
                    &mut compute.input_heap,
                    &compute.input_store_arc_lock,
                ),
            }
        }
    }

    fn follow_idle_plan(
        plan: Plan,
        info: &Info,
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
        idle_data_map: &mut HashMap<plan::ID, IdleData>,
        travel_data_map: &mut HashMap<plan::ID, TravelData>,
    ) {
        let idle_data = idle_data_map.get_mut(&plan.id).unwrap();

        match idle_data.stage {
            plan::Stage::Init => Self::follow_idle_plan_init_stage(idle_data),
            plan::Stage::Active => Self::follow_idle_plan_active_stage(idle_data),
            plan::Stage::Success => Self::follow_idle_plan_success_stage(
                info,
                active_plan_id,
                plan_map,
                travel_data_map,
            ),
            plan::Stage::Fail => Self::follow_idle_plan_fail_stage(),
            plan::Stage::Cancel => Self::follow_idle_plan_cancel_stage(),
        }
    }

    fn follow_idle_plan_init_stage(idle_data: &mut plan::data::IdleData) {
        idle_data.stage = plan::Stage::Active;
    }

    fn follow_idle_plan_active_stage(idle_data: &mut plan::data::IdleData) {
        idle_data.tick_count += 1;

        if idle_data.tick_count >= idle_data.tick_duration {
            idle_data.stage = plan::Stage::Success;
        }
    }

    fn follow_idle_plan_success_stage(
        info: &Info,
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
        travel_data_map: &mut HashMap<plan::ID, plan::data::TravelData>,
    ) {
        let target_position = match info.nation_kind {
            nation::Kind::Lion => IVec3::new(-9, -3, 0),
            nation::Kind::Eagle => IVec3::new(9, -3, 0),
            nation::Kind::Wolf => IVec3::new(0, -3, -9),
            nation::Kind::Horse => IVec3::new(0, -3, 9),
        };

        let (travel_plan, travel_data) = Plan::create_travel_plan(target_position);

        *active_plan_id = Some(travel_plan.id);

        travel_data_map.insert(travel_plan.id, travel_data);
        plan_map.insert(travel_plan.id, travel_plan);
    }

    fn follow_idle_plan_fail_stage() {
        println!("Idle Plan Fail");
    }

    fn follow_idle_plan_cancel_stage() {
        println!("Idle Plan Cancel");
    }

    fn follow_travel_plan(
        plan: Plan,
        world: &World,
        info: &Info,
        kinematic: &Kinematic,
        spatial: &mut Spatial,
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
        travel_data_map: &mut HashMap<plan::ID, plan::data::TravelData>,
        input_heap: &mut BinaryHeap<task::Input>,
        input_store_arc_lock: &Arc<RwLock<task::input::Store>>,
    ) {
        let travel_data = travel_data_map.get_mut(&plan.id).unwrap();

        match travel_data.stage {
            plan::Stage::Init => Self::follow_travel_plan_init_stage(
                plan,
                info,
                world,
                spatial,
                travel_data,
                input_heap,
                input_store_arc_lock,
            ),
            plan::Stage::Active => Self::follow_travel_plan_active_stage(
                plan,
                info,
                world,
                kinematic,
                spatial,
                travel_data,
                input_heap,
                input_store_arc_lock,
            ),
            plan::Stage::Success => {
                Self::follow_travel_plan_success_stage(active_plan_id, plan_map)
            }
            plan::Stage::Fail => {
                Self::follow_travel_plan_fail_stage(active_plan_id, plan_map);
            }
            plan::Stage::Cancel => {
                Self::follow_travel_plan_cancel_stage();
            }
        }
    }

    fn follow_travel_plan_init_stage(
        plan: Plan,
        info: &Info,
        world: &World,
        spatial: &mut Spatial,
        travel_data: &mut plan::data::TravelData,
        task_input_heap: &mut BinaryHeap<task::Input>,
        task_input_store_arc_lock: &Arc<RwLock<task::input::Store>>,
    ) {
        let (level_0_clone, search_level_clone) = {
            let graph_buffer = world.graph_buffer_lock.read().unwrap();
            let graph = graph_buffer.get();

            (graph.level_0.clone(), graph.level_vec[0].clone())
        };

        let task_input = task::Input::new(task::Kind::PathRegion);

        let task_input_data = task::input::data::path::RegionData {
            plan_id: plan.id,
            entity_id: info.entity_id,
            start_position: Grid::world_to_position(&world.grid, spatial.world_position),
            end_position: travel_data.target_position,
            level_0: level_0_clone,
            search_level: search_level_clone,
        };

        {
            let mut task_input_store = task_input_store_arc_lock.write().unwrap();

            task_input_store
                .path_region_data_map
                .insert(task_input.id, task_input_data);
        }

        task_input_heap.push(task_input);
        travel_data.stage = plan::Stage::Active;
    }

    fn follow_travel_plan_active_stage(
        plan: Plan,
        info: &Info,
        world: &World,
        kinematic: &Kinematic,
        spatial: &mut Spatial,
        travel_data: &mut plan::data::TravelData,
        task_input_heap: &mut BinaryHeap<task::Input>,
        task_input_store_arc_lock: &Arc<RwLock<task::input::Store>>,
    ) {
        if let Some(path) = &mut travel_data.path {
            if let Some(step) = path.step_vec.get_mut(path.step_index) {
                if let Some(edge_vec) = &step.edge_vec {
                    if let Some(edge) = edge_vec.get(step.edge_index) {
                        let target_position = edge.node2.position.as_vec3();
                        let displacement = target_position - spatial.world_position;

                        let direction = displacement.normalize_or(Vec3::ZERO);
                        let translation = kinematic.speed * SIMULATION_TICK_IN_SECONDS * direction;

                        if displacement.length_squared() < translation.length_squared() {
                            spatial.world_position = target_position;
                            step.edge_index += 1;
                        } else {
                            spatial.world_position += translation;
                        }
                    } else {
                        path.step_index += 1;
                    }
                } else if !step.pending {
                    match step.edge.kind {
                        edge::Kind::External => {
                            let edge_vec = vec![Edge::new(
                                step.edge.node1,
                                step.edge.node2,
                                edge::Kind::External,
                                10,
                                0,
                            )];

                            step.edge_vec = Some(edge_vec);
                        }
                        edge::Kind::Internal => {
                            let level_0_clone = {
                                let graph_buffer = world.graph_buffer_lock.read().unwrap();
                                let graph = graph_buffer.get();

                                graph.level_0.clone()
                            };

                            let task_input = task::Input::new(task::Kind::PathLocal);

                            let task_input_data = task::input::data::path::LocalData {
                                plan_id: plan.id,
                                entity_id: info.entity_id,
                                step_index: path.step_index,
                                start_position: step.edge.node1.position,
                                end_position: step.edge.node2.position,
                                level_0: level_0_clone,
                            };

                            {
                                let mut task_input_store =
                                    task_input_store_arc_lock.write().unwrap();

                                task_input_store
                                    .path_local_data_map
                                    .insert(task_input.id, task_input_data);
                            }

                            step.pending = true;
                            task_input_heap.push(task_input);
                        }
                    }
                }
            } else {
                travel_data.stage = plan::Stage::Success;
            }
        }
    }

    fn follow_travel_plan_success_stage(
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
    ) {
        if let Some(active_plan_id) = active_plan_id {
            plan_map.remove(active_plan_id);
        }

        *active_plan_id = None;
    }

    fn follow_travel_plan_fail_stage(
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
    ) {
        if let Some(active_plan_id) = active_plan_id {
            plan_map.remove(active_plan_id);
        }

        *active_plan_id = None;
    }

    fn follow_travel_plan_cancel_stage() {
        println!("Travel Plan Cancel");
    }
}
