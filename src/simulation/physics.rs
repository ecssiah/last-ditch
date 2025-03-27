use crate::simulation::{
    self,
    agent::{Agent, AgentID},
    chunk::ChunkID,
    CHUNK_RADIUS, CHUNK_SIZE,
};
use glam::{Quat, Vec3};
use rapier3d::{
    na::{vector, Point3, Vector3},
    pipeline::{PhysicsPipeline, QueryPipeline},
    prelude::{
        nalgebra, CCDSolver, ColliderBuilder, ColliderHandle, ColliderSet, DefaultBroadPhase,
        ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase,
        RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
    },
};
use specs::world;
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

    pub fn add_agent(&mut self, agent: Agent) {
        let position = vector![agent.position.x, agent.position.y, agent.position.z];

        let rigid_body = RigidBodyBuilder::dynamic()
            .lock_rotations()
            .translation(position)
            .build();

        let rigid_body_handle = self.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::capsule_y(0.9, 0.4).build();

        self.collider_set
            .insert_with_parent(collider, rigid_body_handle, &mut self.rigid_body_set);

        self.agent_body_handles.insert(agent.id, rigid_body_handle);
    }

    pub fn add_chunk_collider(
        &mut self,
        chunk_id: ChunkID,
        world_position: Vec3,
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

                log::info!("Added: {:?}", chunk_id);
            }
            Err(err) => {
                log::warn!("Failed to build collider for chunk {}: {:?}", chunk_id, err);
            }
        }
    }

    pub fn apply_agent_input(&mut self, agent: &Agent) {
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

        if input_dir.length_squared() < f32::EPSILON {
            let current = rb.linvel();

            rb.set_linvel(vector![0.0, current.y, 0.0], true);

            return;
        }

        let speed = 0.3;
        let velocity = vector![input_dir.x * speed, rb.linvel().y, input_dir.z * speed];

        rb.set_linvel(velocity, true);
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
