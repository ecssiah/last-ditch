use crate::simulation::{self, agent::Agent, chunk::ChunkID, id::AgentID, JUMP_FORCE, MAX_JUMP_DURATION};
use glam::Vec3;
use rapier3d::{
    na::{vector, Point3, Vector3},
    pipeline::{PhysicsPipeline, QueryPipeline},
    prelude::{
        nalgebra, CCDSolver, CoefficientCombineRule, ColliderBuilder, ColliderHandle, ColliderSet,
        DefaultBroadPhase, ImpulseJointSet, IntegrationParameters, IslandManager,
        MultibodyJointSet, NarrowPhase, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
    },
};
use std::{collections::HashMap, sync::{Arc, RwLock}};

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
    pub chunk_collider_handles: HashMap<ChunkID, ColliderHandle>,
    pub agent_body_handles: HashMap<AgentID, RigidBodyHandle>,
}

impl Physics {
    pub fn new() -> Physics {
        let physics = Self {
            gravity: vector![0.0, -9.81, 0.0],
            integration_parameters: IntegrationParameters::default(),
            pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            query_pipeline: QueryPipeline::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            chunk_collider_handles: HashMap::new(),
            agent_body_handles: HashMap::new(),
        };

        physics
    }

    pub fn add_agent(&mut self, agent: &Agent) {
        let position = vector![agent.position.x, agent.position.y, agent.position.z];

        let rigid_body = RigidBodyBuilder::dynamic()
            .lock_rotations()
            .translation(position)
            .additional_mass(80.0)
            .build();

        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::capsule_y(0.9, 0.4)
            .friction(0.0)
            .friction_combine_rule(CoefficientCombineRule::Min)
            .build();

        self.collider_set
            .insert_with_parent(collider, rigid_body_handle, &mut self.rigid_body_set);

        self.agent_body_handles.insert(agent.id, rigid_body_handle);
    }

    pub fn add_chunk_collider(
        &mut self,
        chunk_id: ChunkID,
        vertices: &[simulation::chunk::Vertex],
        indices: &[u32],
    ) {
        let points: Vec<Point3<f32>> = vertices.iter().map(|v| Point3::from(v.position)).collect();

        let triangle_indices: Vec<[u32; 3]> = indices
            .chunks(3)
            .map(|tri| [tri[0], tri[1], tri[2]])
            .collect();

        match ColliderBuilder::trimesh(points, triangle_indices) {
            Ok(builder) => {
                let collider = builder.build();

                if let Some(old_handle) = self.chunk_collider_handles.remove(&chunk_id) {
                    self.collider_set.remove(
                        old_handle,
                        &mut self.island_manager,
                        &mut self.rigid_body_set,
                        true,
                    );
                }

                let handle = self.collider_set.insert(collider);
                self.chunk_collider_handles.insert(chunk_id, handle);
            }
            Err(err) => {
                log::warn!("Failed to build collider for chunk {}: {:?}", chunk_id, err);
            }
        }
    }

    pub fn apply_agent_movement(&mut self, agent: Arc<RwLock<Agent>>) {
        let agent = agent.read().unwrap();

        let Some(rb_handle) = self.agent_body_handles.get(&agent.id) else {
            return;
        };

        let Some(rb) = self.rigid_body_set.get_mut(*rb_handle) else {
            return;
        };

        let forward = agent.look_rotation * Vec3::Z;
        let forward_xz = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let right_xz = Vec3::Y.cross(forward_xz).normalize();

        let input_dir = agent.x_speed * right_xz + agent.z_speed * forward_xz;

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

    pub fn apply_agent_jump(&mut self, dt: f64, agent: Arc<RwLock<Agent>>) {
        let mut agent = agent.write().unwrap();

        if agent.jump_state.active {
            if let Some(rb_handle) = self.agent_body_handles.get(&agent.id) {
                if let Some(rb) = self.rigid_body_set.get_mut(*rb_handle) {
                    agent.jump_state.timer += dt as f32;

                    let still_powered = agent.jump_state.timer < MAX_JUMP_DURATION && !agent.jump_state.cancel;

                    if still_powered {
                        let force = vector![0.0, JUMP_FORCE * dt as f32, 0.0];
                        rb.add_force(force, true);
                    } else {
                        agent.jump_state.active = false;

                        let lv = rb.linvel();
                        if agent.jump_state.cancel && lv.y > 0.0 {
                            rb.set_linvel(vector![lv.x, lv.y * 0.5, lv.z], true);
                        }
                    }
                }
            }
        }
    }

    pub fn sync_agent_transforms(&self, agent: &mut Agent) {
        if let Some(rb_handle) = self.agent_body_handles.get(&agent.id) {
            if let Some(rb) = self.rigid_body_set.get(*rb_handle) {
                let pos = rb.position();

                let translation = pos.translation.vector;
                agent.position = Vec3::new(translation.x, translation.y, translation.z);
            }
        }
    }

    pub fn step(&mut self) {
        let physics_hooks = ();
        let event_handler = ();

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
            &physics_hooks,
            &event_handler,
        );
    }
}
