use crate::simulation::{
    agent::{Agent, AgentID},
    CHUNK_RADIUS, CHUNK_SIZE,
};
use glam::{Quat, Vec3};
use rapier3d::{
    na::{vector, Vector3},
    pipeline::{PhysicsPipeline, QueryPipeline},
    prelude::{
        nalgebra, CCDSolver, ColliderBuilder, ColliderSet, DefaultBroadPhase, ImpulseJointSet,
        IntegrationParameters, IslandManager, MultibodyJointSet, NarrowPhase, RigidBodyBuilder,
        RigidBodyHandle, RigidBodySet,
    },
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
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub agent_handles: HashMap<AgentID, RigidBodyHandle>,
}

impl Physics {
    pub fn new() -> Physics {
        let mut physics = Self {
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
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            agent_handles: HashMap::new(),
        };

        let ground_collider = ColliderBuilder::cuboid(
            (CHUNK_SIZE) as f32,
            1.0,
            (CHUNK_SIZE) as f32,
        )
        .build();

        physics.colliders.insert(ground_collider);

        physics
    }

    pub fn add_agent(&mut self, agent: Agent) {
        let position = vector![agent.position.x, agent.position.y, agent.position.z];

        let rigid_body = RigidBodyBuilder::dynamic()
            .lock_rotations()
            .translation(position)
            .build();

        let rigid_body_handle = self.bodies.insert(rigid_body);

        let collider = ColliderBuilder::capsule_y(0.9, 0.4).build();

        self.colliders
            .insert_with_parent(collider, rigid_body_handle, &mut self.bodies);

        self.agent_handles.insert(agent.id, rigid_body_handle);
    }

    pub fn apply_agent_input(&mut self, agent: &Agent) {
        let Some(rb_handle) = self.agent_handles.get(&agent.id) else {
            return;
        };

        let Some(rb) = self.bodies.get_mut(*rb_handle) else {
            return;
        };

        let input_dir = Vec3::new(agent.x_speed, 0.0, agent.z_speed);

        if input_dir.length_squared() < f32::EPSILON {
            let current = rb.linvel();
            rb.set_linvel(vector![0.0, current.y, 0.0], true);
            return;
        }

        let forward = agent.look_rotation * input_dir.normalize();
        let speed = 6.0;

        let velocity = vector![forward.x * speed, rb.linvel().y, forward.z * speed];
        rb.set_linvel(velocity, true);
    }

    pub fn sync_agent_transforms(&self, agent: &mut Agent) {
        if let Some(rb_handle) = self.agent_handles.get(&agent.id) {
            if let Some(rb) = self.bodies.get(*rb_handle) {
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
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &physics_hooks,
            &event_handler,
        );
    }
}
