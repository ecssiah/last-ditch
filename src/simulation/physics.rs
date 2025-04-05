use crate::simulation::{
    self,
    agent::{self, Agent},
    chunk,
    consts::*,
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
    pub agent_body_handles: HashMap<agent::ID, RigidBodyHandle>,
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
        let agent_body_handles = HashMap::new();
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
            agent_body_handles,
            chunk_collider_handles,
        };

        physics
    }

    pub fn generate(
        &mut self,
        agents: &HashMap<agent::ID, Agent>,
        chunks: &[chunk::Chunk; CHUNK_VOLUME],
    ) {
        for agent in agents.values() {
            self.add_agent(agent);
        }

        for chunk in chunks {
            self.add_chunk_collider(chunk);
        }
    }

    pub fn update(&mut self, state: &mut State) {
        if let Some(user_agent) = state.agents.get_mut(&agent::ID::USER_AGENT) {
            self.update_agent_movement(user_agent);
            self.update_agent_jump(user_agent);
            self.step();
            self.sync_agent_transforms(user_agent);
        }
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
            .restitution(0.0)
            .restitution_combine_rule(CoefficientCombineRule::Min)
            .build();

        self.collider_set
            .insert_with_parent(collider, rigid_body_handle, &mut self.rigid_body_set);

        self.agent_body_handles.insert(agent.id, rigid_body_handle);
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

    pub fn update_agent_movement(&mut self, agent: &Agent) {
        let Some(rb_handle) = self.agent_body_handles.get(&agent.id) else {
            return;
        };

        let Some(rb) = self.rigid_body_set.get_mut(*rb_handle) else {
            return;
        };

        let forward = agent.orientation * Vec3::Z;
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

    pub fn begin_agent_jump(&mut self, agent: &Agent) {
        if let Some(rb_handle) = self.agent_body_handles.get(&agent.id) {
            if let Some(rb) = self.rigid_body_set.get_mut(*rb_handle) {
                let lv = rb.linvel();
                rb.set_linvel(vector![lv.x, JUMP_LAUNCH_VELOCITY, lv.z], true);
            }
        }
    }

    pub fn update_agent_jump(&mut self, agent: &mut Agent) {
        if agent.jump_state.active {
            if let Some(rb_handle) = self.agent_body_handles.get(&agent.id) {
                if let Some(rb) = self.rigid_body_set.get_mut(*rb_handle) {
                    agent.jump_state.timer += FIXED_DT;

                    let is_powered =
                        agent.jump_state.timer < MAX_JUMP_DURATION && !agent.jump_state.cancel;

                    if is_powered {
                        let force = vector![0.0, JUMP_HOLD_FORCE * FIXED_DT.as_secs_f32(), 0.0];
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
