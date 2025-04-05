use crate::simulation::{
    self, chunk,
    consts::*,
    population::{entity::{self, JumpStage}, Entity},
    state::State,
};
use glam::Vec3;
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
        for entity in state.population.all() {
            self.add_entity(entity);
        }

        for chunk in state.world.chunks.iter() {
            self.add_chunk_collider(chunk);
        }
    }

    pub fn tick(&mut self, state: &mut State) {
        if let Some(entity) = state.population.get_mut(&entity::ID::USER_ENTITY) {
            self.tick_entity_movement(entity);
            self.tick_entity_jump(entity);

            self.step();

            self.sync_entity_transforms(entity);
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

        let collider = ColliderBuilder::capsule_y(0.9, 0.4)
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
            .map(|v| Point3::from(v.position))
            .collect();

        let triangle_indices: Vec<[u32; 3]> = chunk
            .mesh
            .indices
            .chunks(3)
            .map(|tri| [tri[0], tri[1], tri[2]])
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

        let Some(rb) = self.rigid_body_set.get_mut(*rb_handle) else {
            return;
        };

        let forward = entity.orientation * Vec3::Z;
        let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let right_xz = Vec3::Y.cross(forward_xz).normalize();

        let input_dir = entity.x_speed * right_xz + entity.z_speed * forward_xz;

        let mut velocity = *rb.linvel();

        if input_dir.length_squared() < f32::EPSILON {
            velocity.x = 0.0;
            velocity.z = 0.0;
        } else {
            let speed = 0.3;
            velocity.x = input_dir.x * speed;
            velocity.z = input_dir.z * speed;
        }

        rb.set_linvel(velocity, true);
    }

    pub fn tick_entity_jump(&mut self, entity: &mut Entity) {
        match entity.jump_state.stage {
            JumpStage::Launch => {
                if let Some(rb_handle) = self.entity_body_handles.get(&entity.id) {
                    if let Some(rb) = self.rigid_body_set.get_mut(*rb_handle) {
                        let lv = rb.linvel();
                        let jump_velocity = vector![lv.x, JUMP_LAUNCH_VELOCITY, lv.z];

                        rb.set_linvel(jump_velocity, true);
                    }
                }
            },
            JumpStage::Rise => {
                if let Some(rb_handle) = self.entity_body_handles.get(&entity.id) {
                    if let Some(rb) = self.rigid_body_set.get_mut(*rb_handle) {
                        entity.jump_state.timer += 1;
    
                        if entity.jump_state.timer < MAX_JUMP_TICKS {
                            let force = vector![0.0, JUMP_HOLD_FORCE, 0.0];

                            rb.add_force(force, true);
                        } else {
                            entity.jump_state.stage = JumpStage::Fall;
                        }
                    }
                }
            },
            JumpStage::Fall => {
                if let Some(rb_handle) = self.entity_body_handles.get(&entity.id) {
                    if let Some(rb) = self.rigid_body_set.get_mut(*rb_handle) {
                        entity.jump_state.stage = JumpStage::Ground;
    
                        let lv = rb.linvel();
                        if lv.y > 0.0 {
                            rb.set_linvel(vector![lv.x, lv.y * 0.5, lv.z], true);
                        }
                    }
                }
            },
            _ => (),
        }
    }

    pub fn sync_entity_transforms(&self, entity: &mut Entity) {
        if let Some(rb_handle) = self.entity_body_handles.get(&entity.id) {
            if let Some(rb) = self.rigid_body_set.get(*rb_handle) {
                let pos = rb.position();

                let translation = pos.translation.vector;
                entity.position = Vec3::new(translation.x, translation.y, translation.z);
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
