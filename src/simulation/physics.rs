pub mod entity_controller;

use crate::simulation::{
    self, chunk,
    consts::*,
    physics::entity_controller::EntityController,
    population::{
        entity::{self, JumpStage},
        Entity,
    },
    state::State,
    world::World,
    Chunk,
};
use glam::Vec3;
use nalgebra::{ArrayStorage, Const, Matrix, Unit};
use rapier3d::{
    control::{CharacterLength, EffectiveCharacterMovement, KinematicCharacterController},
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
    pub entity_controllers: HashMap<entity::ID, EntityController>,
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
        let entity_controllers = HashMap::new();

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
            entity_controllers,
        };

        physics
    }

    pub fn generate(&mut self, state: &State) {
        self.generate_boundaries();
        self.generate_entities(state);
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

    fn update_chunk_colliders(&mut self, state: &State) {
        let entity = state.population.get(&entity::ID::USER_ENTITY1).unwrap();

        let current_grid_position = World::grid_position_at(entity.position).unwrap();
        let current_chunk_id = Chunk::id_at_grid(current_grid_position).unwrap();

        let visible_chunk_ids = World::visible_chunk_ids(current_chunk_id, USER_VIEW_RADIUS as i32);

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

    pub fn add_entity(&mut self, entity: &Entity) {
        let position = vector![entity.position.x, entity.position.y, entity.position.z];

        let rigid_body = RigidBodyBuilder::kinematic_position_based()
            .lock_rotations()
            .translation(position)
            .build();

        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::capsule_y(0.5, 0.4).build();

        let collider_handle = self.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut self.rigid_body_set,
        );

        let character_controller = KinematicCharacterController {
            up: Vector::y_axis(),
            offset: CharacterLength::Relative(0.02),
            snap_to_ground: Some(CharacterLength::Relative(0.3)),
            autostep: None,
            ..Default::default()
        };

        let entity_controller = EntityController {
            entity_id: entity.id,
            rigid_body_handle,
            collider_handle,
            character_controller,
        };

        self.entity_controllers.insert(entity.id, entity_controller);
    }

    pub fn add_chunk_collider(&mut self, chunk: &simulation::chunk::Chunk) {
        let (points, triangle_indices) = chunk.mesh.optimized_vertices_and_indices();

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

    pub fn tick(&mut self, state: &mut State) {
        if let Some(entity) = state.population.get_mut(&entity::ID::USER_ENTITY1) {
            self.tick_character_controllers(entity);

            self.step();

            self.sync_entities(entity);
        }

        self.update_chunk_colliders(state);
    }

    fn tick_character_controllers(&mut self, entity: &mut Entity) {
        let entity_controller = self.entity_controllers.get(&entity.id).unwrap();

        let desired_translation = self.get_desired_translation(entity);
        let effective_movement = self.get_effective_movement(entity, desired_translation);

        let rigid_body = self
            .rigid_body_set
            .get(entity_controller.rigid_body_handle)
            .unwrap();

        let new_translation =
            rigid_body.position().translation.vector + effective_movement.translation;

        let rigid_body = self
            .rigid_body_set
            .get_mut(entity_controller.rigid_body_handle)
            .unwrap();

        rigid_body.set_next_kinematic_translation(new_translation);
    }

    fn get_desired_translation(
        &self,
        entity: &mut Entity,
    ) -> Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>> {
        let forward = entity.orientation * Vec3::Z;
        let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let right_xz = Vec3::Y.cross(forward_xz).normalize();

        let input_direction = entity.x_speed * right_xz + entity.z_speed * forward_xz;

        entity.velocity.x = input_direction.x * DEFAULT_X_SPEED;
        entity.velocity.z = input_direction.z * DEFAULT_Z_SPEED;

        match entity.jump_state.stage {
            JumpStage::Launch => {
                entity.jump_state.stage = JumpStage::Rise;
                entity.velocity.y = JUMP_LAUNCH_VELOCITY;
            }
            JumpStage::Rise => {
                entity.jump_state.timer += 1;

                if entity.jump_state.timer < MAX_JUMP_TICKS {
                    entity.velocity.y = JUMP_HOLD_VELOCITY;
                } else {
                    entity.jump_state.stage = JumpStage::Fall;
                }
            }
            JumpStage::Fall => {
                entity.jump_state.stage = JumpStage::Ground;
                entity.velocity.y = 0.5 * entity.velocity.y;
            }
            _ => (),
        }

        entity.velocity.y = entity.velocity.y - GRAVITY_ACCELERATION;

        let desired_translation = vector![entity.velocity.x, entity.velocity.y, entity.velocity.z];

        desired_translation
    }

    fn get_effective_movement(
        &self,
        entity: &mut Entity,
        desired_translation: Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>,
    ) -> EffectiveCharacterMovement {
        let entity_controller = self.entity_controllers.get(&entity.id).unwrap();

        let rigid_body = self
            .rigid_body_set
            .get(entity_controller.rigid_body_handle)
            .unwrap();
        let collider = self
            .collider_set
            .get(entity_controller.collider_handle)
            .unwrap();

        let effective_movement = entity_controller.character_controller.move_shape(
            self.integration_parameters.dt,
            &self.rigid_body_set,
            &self.collider_set,
            &self.query_pipeline,
            collider.shape(),
            &rigid_body.position(),
            desired_translation.cast::<Real>(),
            QueryFilter::new().exclude_rigid_body(entity_controller.rigid_body_handle),
            |_| {},
        );

        effective_movement
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

    pub fn sync_entities(&self, entity: &mut Entity) {
        let Some(entity_controller) = self.entity_controllers.get(&entity.id) else {
            return;
        };

        let Some(rigid_body) = self.rigid_body_set.get(entity_controller.rigid_body_handle) else {
            return;
        };

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
