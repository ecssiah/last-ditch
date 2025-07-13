use crate::simulation::state::{
    compute::{self, task},
    physics::aabb::AABB,
    population::entity::{
        self,
        decision::{plan, Plan},
        Decision, Detection, Info, Kinematic, Nation, Spatial,
    },
    world::{chunk, grid::Grid, World},
    Compute,
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
                    &mut agent.decision.active_plan_id,
                    &mut agent.decision.plan_map,
                    &mut agent.decision.plan_store.idle_data_map,
                    &mut agent.decision.plan_store.travel_data_map,
                ),
                plan::Kind::Travel => Self::follow_travel_plan(
                    plan,
                    &agent.info,
                    world,
                    &mut agent.spatial,
                    &mut agent.decision.active_plan_id,
                    &mut agent.decision.plan_map,
                    &mut agent.decision.plan_store.travel_data_map,
                    &mut compute.task_heap,
                    &compute.task_store_arc_lock,
                ),
            }
        }
    }

    fn follow_idle_plan(
        plan: Plan,
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
        idle_data_map: &mut HashMap<plan::ID, plan::data::Idle>,
        travel_data_map: &mut HashMap<plan::ID, plan::data::Travel>,
    ) {
        let idle_data = idle_data_map.get_mut(&plan.id).unwrap();

        match idle_data.stage {
            plan::Stage::Init => Self::follow_idle_plan_init_stage(idle_data),
            plan::Stage::Active => Self::follow_idle_plan_active_stage(idle_data),
            plan::Stage::Success => {
                Self::follow_idle_plan_success_stage(active_plan_id, plan_map, travel_data_map)
            }
            plan::Stage::Fail => Self::follow_idle_plan_fail_stage(),
            plan::Stage::Cancel => Self::follow_idle_plan_cancel_stage(),
        }
    }

    fn follow_idle_plan_init_stage(idle_data: &mut plan::data::Idle) {
        idle_data.stage = plan::Stage::Active;
    }

    fn follow_idle_plan_active_stage(idle_data: &mut plan::data::Idle) {
        idle_data.tick_count += 1;

        if idle_data.tick_count >= idle_data.tick_duration {
            idle_data.stage = plan::Stage::Success;
        }
    }

    fn follow_idle_plan_success_stage(
        active_plan_id: &mut Option<plan::ID>,
        plan_map: &mut HashMap<plan::ID, Plan>,
        travel_data_map: &mut HashMap<plan::ID, plan::data::Travel>,
    ) {
        let (travel_plan, travel_data) = Plan::create_travel_plan(IVec3::new(0, 6, 9));

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

        match travel_data.stage {
            plan::Stage::Init => Self::follow_travel_plan_init_stage(
                plan,
                info,
                world,
                spatial,
                travel_data,
                task_heap,
                task_store_arc_lock,
            ),
            plan::Stage::Active => Self::follow_travel_plan_active_stage(
                plan,
                info,
                world,
                spatial,
                travel_data,
                task_heap,
                task_store_arc_lock,
            ),
            plan::Stage::Success => {
                Self::follow_travel_plan_success_stage(active_plan_id, plan_map)
            }
            plan::Stage::Fail => {
                Self::follow_travel_plan_fail_stage();
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

        let task_data = compute::task::data::path::Region {
            plan_id: plan.id,
            entity_id: info.entity_id,
            start_position: Grid::world_to_position(&world.grid, spatial.world_position),
            end_position: travel_data.target_position,
            level_0: level_0_clone,
            search_level: search_level_clone,
        };

        {
            let mut task_store = task_store_arc_lock.write().unwrap();
            task_store.path_region_data_map.insert(task.id, task_data);
        }

        task_heap.push(task);
        travel_data.stage = plan::Stage::Active;
    }

    fn follow_travel_plan_active_stage(
        plan: Plan,
        info: &Info,
        world: &World,
        spatial: &mut Spatial,
        travel_data: &mut plan::data::Travel,
        task_heap: &mut BinaryHeap<compute::Task>,
        task_store_arc_lock: &Arc<RwLock<task::Store>>,
    ) {
        if travel_data.region_path_found {
            if travel_data.local_path_found {
                if let Some(target_position) = travel_data
                    .local_path
                    .position_vec
                    .get(travel_data.local_path_index)
                    .map(|target_position| target_position.as_vec3())
                {
                    let distance_vector = target_position - spatial.world_position;

                    if distance_vector.length_squared() >= 0.01 {
                        let direction_vector = distance_vector.normalize();

                        spatial.world_position += 0.06 * direction_vector;
                    } else {
                        travel_data.local_path_index += 1;

                        spatial.world_position = target_position;
                    }
                } else {
                    travel_data.region_path_index += 2;
                    travel_data.local_path_found = false;

                    if travel_data.region_path_index >= travel_data.region_path.position_vec.len() {
                        travel_data.region_path_complete = true;
                    }
                }
            } else {
                if travel_data.region_path_index >= travel_data.region_path.position_vec.len() {
                    travel_data.region_path_complete = true;
                } else {
                    let task = compute::Task::new(
                        compute::task::Priority::High,
                        compute::task::Kind::PathLocal,
                    );

                    let level_0_clone = {
                        let graph_buffer = world.graph_buffer_lock.read().unwrap();
                        let graph = graph_buffer.get();

                        graph.level_0.clone()
                    };

                    let start_position =
                        travel_data.region_path.position_vec[travel_data.region_path_index - 1];

                    let end_position =
                        travel_data.region_path.position_vec[travel_data.region_path_index];

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
            }
        }

        if travel_data.region_path_complete {
            travel_data.stage = plan::Stage::Success;
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

    fn follow_travel_plan_fail_stage() {
        println!("Travel Plan Fail");
    }

    fn follow_travel_plan_cancel_stage() {
        println!("Travel Plan Cancel");
    }
}
