use crate::simulation::{
    self, chunk,
    consts::*,
    population::{
        entity::{self, JumpStage},
        Entity,
    },
    state::State,
    world::World,
    Chunk,
};
use glam::Vec3;
use nalgebra::Unit;
use rapier3d::{
    na::{vector, Point3, Vector3},
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
    pub entity_body_handles: HashMap<entity::ID, RigidBodyHandle>,
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
        let entity_body_handles = HashMap::new();
        let chunk_collider_handles = HashMap::new();

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
            entity_body_handles,
            chunk_collider_handles,
        };

        physics
    }

    pub fn generate(&mut self, state: &State) {
        self.generate_boundaries();
        self.generate_entities(state);
        self.generate_chunks(state);
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

    pub fn generate_entities(&mut self, state: &State) {
        for entity in state.population.all() {
            self.add_entity(entity);
        }
    }

    pub fn generate_chunks(&mut self, state: &State) {
        for chunk in state.world.chunks.iter() {
            self.add_chunk_collider(chunk);
        }
    }

    pub fn tick(&mut self, state: &mut State) {
        if let Some(entity) = state.population.get_mut(&entity::ID::USER_ENTITY) {
            self.tick_entity_movement(entity);
            self.tick_entity_jump(entity);

            self.step();

            self.sync_entities(entity);
        }

        self.update_chunk_colliders(state);
    }

    fn update_chunk_colliders(&mut self, state: &State) {
        let entity = state.population.get(&entity::ID::USER_ENTITY).unwrap();

        let current_grid_position = World::grid_position_at(entity.position).unwrap();
        let current_chunk_id = Chunk::id_at_grid(current_grid_position).unwrap();

        let visible_chunk_ids = World::visible_chunk_ids(current_chunk_id, VIEW_RADIUS as i32);

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
                    self.add_chunk_collider(chunk);
                }
            }
        }
    }

    pub fn add_entity(&mut self, entity: &Entity) {
        let position = vector![entity.position.x, entity.position.y, entity.position.z];

        let rigid_body = RigidBodyBuilder::dynamic()
            .lock_rotations()
            .translation(position)
            .additional_mass(80.0)
            .build();

        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::capsule_y(0.5, 0.4)
            .friction(0.0)
            .friction_combine_rule(CoefficientCombineRule::Min)
            .restitution(0.0)
            .restitution_combine_rule(CoefficientCombineRule::Min)
            .build();

        self.collider_set
            .insert_with_parent(collider, rigid_body_handle, &mut self.rigid_body_set);

        self.entity_body_handles
            .insert(entity.id, rigid_body_handle);
    }

    pub fn add_chunk_collider(&mut self, chunk: &simulation::chunk::Chunk) {
        let points: Vec<Point3<f32>> = chunk
            .mesh
            .vertices
            .iter()
            .map(|vertex| Point3::from(vertex.position))
            .collect();

        let triangle_indices: Vec<[u32; 3]> = chunk
            .mesh
            .indices
            .chunks(3)
            .map(|triangle| [triangle[0], triangle[1], triangle[2]])
            .collect();

        match ColliderBuilder::trimesh(points, triangle_indices) {
            Ok(builder) => {
                let collider = builder.build();

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

    pub fn tick_entity_movement(&mut self, entity: &Entity) {
        let Some(rb_handle) = self.entity_body_handles.get(&entity.id) else {
            return;
        };

        let Some(rigid_body) = self.rigid_body_set.get_mut(*rb_handle) else {
            return;
        };

        let forward = entity.orientation * Vec3::Z;
        let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let right_xz = Vec3::Y.cross(forward_xz).normalize();

        let input_dir = entity.x_speed * right_xz + entity.z_speed * forward_xz;

        let mut velocity = *rigid_body.linvel();

        if input_dir.length_squared() < f32::EPSILON {
            velocity.x = 0.0;
            velocity.z = 0.0;
        } else {
            velocity.x = input_dir.x * DEFAULT_X_SPEED;
            velocity.z = input_dir.z * DEFAULT_Z_SPEED;
        }

        rigid_body.set_linvel(velocity, true);
    }

    pub fn tick_entity_jump(&mut self, entity: &mut Entity) {
        match entity.jump_state.stage {
            JumpStage::Launch => {
                if let Some(rb_handle) = self.entity_body_handles.get(&entity.id) {
                    if let Some(rigid_body) = self.rigid_body_set.get_mut(*rb_handle) {
                        entity.jump_state.stage = JumpStage::Rise;

                        let lv = rigid_body.linvel();
                        let jump_velocity = vector![lv.x, JUMP_LAUNCH_VELOCITY, lv.z];

                        rigid_body.set_linvel(jump_velocity, true);
                    }
                }
            }
            JumpStage::Rise => {
                if let Some(rb_handle) = self.entity_body_handles.get(&entity.id) {
                    if let Some(rigid_body) = self.rigid_body_set.get_mut(*rb_handle) {
                        entity.jump_state.timer += 1;

                        if entity.jump_state.timer < MAX_JUMP_TICKS {
                            let force = vector![0.0, JUMP_HOLD_FORCE, 0.0];
                            rigid_body.add_force(force, true);
                        } else {
                            entity.jump_state.stage = JumpStage::Fall;
                        }
                    }
                }
            }
            JumpStage::Fall => {
                if let Some(rb_handle) = self.entity_body_handles.get(&entity.id) {
                    if let Some(rigid_body) = self.rigid_body_set.get_mut(*rb_handle) {
                        entity.jump_state.stage = JumpStage::Ground;

                        rigid_body.reset_forces(true);

                        let lv = rigid_body.linvel();
                        let damped_velocity = vector![lv.x, 0.5 * lv.y, lv.z];

                        rigid_body.set_linvel(damped_velocity, true);
                    }
                }
            }
            _ => (),
        }
    }

    pub fn sync_entities(&self, entity: &mut Entity) {
        if let Some(rb_handle) = self.entity_body_handles.get(&entity.id) {
            if let Some(rigid_body) = self.rigid_body_set.get(*rb_handle) {
                let rigid_body_position = rigid_body.position();

                let translation = rigid_body_position.translation.vector;
                let next_position = Vec3::new(translation.x, translation.y, translation.z);

                let current_grid_position = World::grid_position_at(entity.position).unwrap();
                let next_grid_position = World::grid_position_at(next_position).unwrap();

                entity.position = next_position;

                if current_grid_position == next_grid_position {
                    entity.chunk_update = false;
                } else {
                    let current_chunk_id = Chunk::id_at_grid(current_grid_position).unwrap();
                    let next_chunk_id = Chunk::id_at_grid(next_grid_position).unwrap();

                    entity.chunk_update = next_chunk_id != current_chunk_id;
                }
            }
        }
    }

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
}
