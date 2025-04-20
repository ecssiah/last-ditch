pub mod judge_controller;

use crate::simulation::{
    self, chunk,
    consts::*,
    physics::judge_controller::JudgeController,
    population::{
        judge::{self, JumpStage},
        Judge,
    },
    state::State,
    world::World,
    Chunk,
};
use glam::Vec3;
use nalgebra::Unit;
use rapier3d::{
    na::{vector, Vector3},
    pipeline::{PhysicsPipeline, QueryPipeline},
    prelude::*,
};
use std::collections::HashMap;

pub struct Physics {
    pub gravity: Vector3<f32>,
    pub integration_parameters: IntegrationParameters,
    pub pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
    pub query_pipeline: QueryPipeline,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub chunk_collider_handles: HashMap<chunk::ID, ColliderHandle>,
    pub judge_controllers: HashMap<judge::ID, JudgeController>,
}

impl Physics {
    pub fn new() -> Physics {
        let gravity = vector![0.0, GRAVITY_ACCELERATION, 0.0];

        let integration_parameters = IntegrationParameters {
            dt: FIXED_DT.as_secs_f32(),
            ..Default::default()
        };

        let pipeline = PhysicsPipeline::new();
        let island_manager = IslandManager::new();
        let broad_phase = DefaultBroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let impulse_joint_set = ImpulseJointSet::new();
        let multibody_joint_set = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::new();
        let query_pipeline = QueryPipeline::new();
        let rigid_body_set = RigidBodySet::new();
        let collider_set = ColliderSet::new();
        let chunk_collider_handles = HashMap::new();
        let judge_controllers = HashMap::new();

        let physics = Self {
            gravity,
            integration_parameters,
            pipeline,
            island_manager,
            broad_phase,
            narrow_phase,
            impulse_joint_set,
            multibody_joint_set,
            ccd_solver,
            query_pipeline,
            rigid_body_set,
            collider_set,
            chunk_collider_handles,
            judge_controllers,
        };

        physics
    }

    pub fn generate(&mut self, state: &State) {
        self.generate_boundaries();
        self.generate_judge(state);
        self.generate_agents(state);
    }

    pub fn generate_boundaries(&mut self) {
        let boundary = (WORLD_BOUNDARY as f32) + 0.5;

        let boundary_positions = [
            (vector![-boundary, 0.0, 0.0], vector![1.0, 0.0, 0.0]),
            (vector![boundary, 0.0, 0.0], vector![-1.0, 0.0, 0.0]),
            (vector![0.0, -boundary, 0.0], vector![0.0, 1.0, 0.0]),
            (vector![0.0, boundary, 0.0], vector![0.0, -1.0, 0.0]),
            (vector![0.0, 0.0, -boundary], vector![0.0, 0.0, 1.0]),
            (vector![0.0, 0.0, boundary], vector![0.0, 0.0, -1.0]),
        ];

        for (position, normal) in boundary_positions.iter() {
            let unit_normal = Unit::new_normalize(*normal);

            let collider = ColliderBuilder::halfspace(unit_normal)
                .translation(*position)
                .build();

            self.collider_set.insert(collider);
        }
    }

    fn generate_judge(&mut self, state: &State) {
        let judge = state.population.get_judge();
        let position = vector![judge.position.x, judge.position.y, judge.position.z];

        let rigid_body = RigidBodyBuilder::dynamic()
            .ccd_enabled(true)
            .linear_damping(0.1)
            .lock_rotations()
            .translation(position)
            .build();

        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::cuboid(ENTITY_SIZE_X, ENTITY_SIZE_Y, ENTITY_SIZE_Z)
            .mass(50.0)
            .contact_skin(0.02)
            .friction(0.0)
            .friction_combine_rule(CoefficientCombineRule::Average)
            .restitution(0.0)
            .restitution_combine_rule(CoefficientCombineRule::Average)
            .build();

        let collider_handle = self.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigid_body_set,
        );

        let judge_controller = JudgeController {
            judge_id: judge.id,
            rigid_body_handle,
            collider_handle,
        };

        self.judge_controllers.insert(judge.id, judge_controller);
    }

    fn generate_agents(&mut self, _state: &State) {}

    fn update_chunk_colliders(&mut self, state: &State) {
        let judge = state.population.get_judge();

        let grid_position = World::grid_position_at(judge.position).unwrap();
        let current_chunk_id = Chunk::id_at_grid(grid_position).unwrap();

        let visible_chunk_ids =
            World::visible_chunk_ids(current_chunk_id, USER_VIEW_RADIUS as i32);

        let current_loaded_chunks: Vec<chunk::ID> =
            self.chunk_collider_handles.keys().cloned().collect();

        for chunk_id in current_loaded_chunks {
            if !visible_chunk_ids.contains(&chunk_id) {
                if let Some(old_handle) = self.chunk_collider_handles.remove(&chunk_id) {
                    self.collider_set.remove(
                        old_handle,
                        &mut self.island_manager,
                        &mut self.rigid_body_set,
                        true,
                    );
                }
            }
        }

        for &chunk_id in visible_chunk_ids.iter() {
            if !self.chunk_collider_handles.contains_key(&chunk_id) {
                if let Some(chunk) = state.world.get_chunk(chunk_id) {
                    if chunk.mesh.faces.len() > 0 {
                        self.add_chunk_collider(chunk);
                    }
                }
            }
        }
    }

    pub fn add_judge(&mut self, judge: &Judge) {
        let position = vector![judge.position.x, judge.position.y, judge.position.z];

        let rigid_body = RigidBodyBuilder::dynamic()
            .ccd_enabled(true)
            .linear_damping(0.1)
            .lock_rotations()
            .translation(position)
            .build();

        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::cuboid(ENTITY_SIZE_X, ENTITY_SIZE_Y, ENTITY_SIZE_Z)
            .mass(50.0)
            .contact_skin(0.02)
            .friction(0.0)
            .friction_combine_rule(CoefficientCombineRule::Average)
            .restitution(0.0)
            .restitution_combine_rule(CoefficientCombineRule::Average)
            .build();

        let collider_handle = self.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigid_body_set,
        );

        let judge_controller = JudgeController {
            judge_id: judge.id,
            rigid_body_handle,
            collider_handle,
        };

        self.judge_controllers.insert(judge.id, judge_controller);
    }

    pub fn add_chunk_collider(&mut self, chunk: &simulation::chunk::Chunk) {
        let (points, triangle_indices) = chunk.mesh.optimized_vertices_and_indices();

        match ColliderBuilder::trimesh(points, triangle_indices) {
            Ok(builder) => {
                let collider = builder
                    .friction(0.0)
                    .friction_combine_rule(CoefficientCombineRule::Average)
                    .restitution(0.0)
                    .restitution_combine_rule(CoefficientCombineRule::Average)
                    .build();

                if let Some(old_handle) = self.chunk_collider_handles.remove(&chunk.id) {
                    self.collider_set.remove(
                        old_handle,
                        &mut self.island_manager,
                        &mut self.rigid_body_set,
                        true,
                    );
                }

                let handle = self.collider_set.insert(collider);
                self.chunk_collider_handles.insert(chunk.id, handle);
            }
            Err(err) => {
                log::warn!(
                    "Failed to build collider for chunk {:?}: {:?}",
                    chunk.id,
                    err
                );
            }
        }
    }

    pub fn tick(&mut self, state: &mut State) {
        self.tick_judge(state);
        self.tick_agents(state);

        self.step();

        self.sync_judge(state);

        self.update_chunk_colliders(state);
    }

    fn tick_judge(&mut self, state: &mut State) {
        let judge = state.population.get_judge_mut();

        let judge_controller = self.judge_controllers.get(&judge.id).unwrap();

        let rigid_body = self
            .rigid_body_set
            .get_mut(judge_controller.rigid_body_handle)
            .unwrap();

        let forward = judge.orientation * Vec3::Z;
        let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let right_xz = Vec3::Y.cross(forward_xz).normalize();

        let input_direction = judge.x_speed * right_xz + judge.z_speed * forward_xz;

        let mut velocity = *rigid_body.linvel();

        velocity.x = input_direction.x * DEFAULT_X_SPEED;
        velocity.z = input_direction.z * DEFAULT_Z_SPEED;

        match judge.jump_state.stage {
            JumpStage::Launch => {
                judge.jump_state.stage = JumpStage::Rise;
                velocity.y = JUMP_LAUNCH_VELOCITY;
            }
            JumpStage::Rise => {
                judge.jump_state.timer += 1;

                if judge.jump_state.timer < MAX_JUMP_TICKS {
                    velocity.y = JUMP_LAUNCH_VELOCITY;
                } else {
                    judge.jump_state.stage = JumpStage::Ground;
                }
            }
            JumpStage::Fall => {
                judge.jump_state.stage = JumpStage::Ground;
            }
            JumpStage::Ground => {}
        }

        rigid_body.set_linvel(velocity, true);
    }

    fn tick_agents(&mut self, _state: &mut State) {}

    pub fn step(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
    }

    fn sync_judge(&self, state: &mut State) {
        let judge = state.population.get_judge_mut();

        let Some(entity_controller) = self.judge_controllers.get(&judge.id) else {
            return;
        };

        let Some(rigid_body) = self.rigid_body_set.get(entity_controller.rigid_body_handle)
        else {
            return;
        };

        let rigid_body_position = rigid_body.position();

        let translation = rigid_body_position.translation.vector;
        let next_position = Vec3::new(translation.x, translation.y, translation.z);

        let current_grid_position = World::grid_position_at(judge.position).unwrap();
        let next_grid_position = World::grid_position_at(next_position).unwrap();

        judge.position = next_position;

        if current_grid_position == next_grid_position {
            judge.chunk_update = false;
        } else {
            let current_chunk_id = Chunk::id_at_grid(current_grid_position).unwrap();
            let next_chunk_id = Chunk::id_at_grid(next_grid_position).unwrap();

            judge.chunk_update = next_chunk_id != current_chunk_id;
        }
    }
}
