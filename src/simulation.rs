//! The Simulation module contains all of the logic required to generate and evolve
//! the core civilizational garden.

pub mod action;
pub mod agent;
pub mod block;
pub mod chunk;
pub mod consts;
pub mod id;
pub mod observation;
pub mod physics;
pub mod state;
pub mod structure;
pub mod time;
pub mod world;

use crate::simulation::{
    action::{JumpAction, MovementAction},
    id::agent_id::AgentID,
    observation::Observation,
    time::{Tick, Time},
    world::World,
};
use action::{Action, AgentAction, WorldAction};
use agent::Agent;
pub use block::Block;
pub use chunk::Chunk;
pub use consts::*;
use glam::Quat;
use physics::Physics;
use state::State;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::{Duration, Instant},
};
use tokio::sync::mpsc::UnboundedReceiver;

pub struct Simulation {
    action_rx: UnboundedReceiver<Action>,
    state: State,
    observation: Arc<RwLock<Observation>>,
    physics: Physics,
}

impl Simulation {
    pub fn new(action_rx: UnboundedReceiver<Action>) -> Self {
        let state = State {
            active: true,
            seed: SEED,
            agents: Self::setup_agents(),
            time: Self::setup_time(),
            world: Self::setup_world(),
        };

        let observation = Arc::new(RwLock::new(Observation::new()));
        let physics = Physics::new();

        let simulation = Self {
            action_rx,
            state,
            observation,
            physics,
        };

        log::info!("Simulation Initialized");

        simulation
    }

    pub fn get_observation(&self) -> Arc<RwLock<Observation>> {
        Arc::clone(&self.observation)
    }

    pub fn run(&mut self) {
        let mut accumulator = Duration::ZERO;
        let mut previous = Instant::now();

        self.state.world.generate();
        self.physics.generate(&self.state.agents);

        loop {
            let now = Instant::now();
            let frame_time = now.duration_since(previous);
            previous = now;

            accumulator += frame_time;

            while accumulator >= FIXED_DT {
                self.update();
                accumulator -= FIXED_DT;
            }

            thread::sleep(SIMULATION_WAIT_DURATION);
        }
    }

    fn setup_time() -> Time {
        let time = Time {
            clock: Duration::ZERO,
            tick: Tick::ZERO,
        };

        time
    }

    fn setup_agents() -> HashMap<AgentID, Agent> {
        let mut agents = HashMap::new();

        let mut user_agent = Agent::new(AgentID::USER_AGENT_ID);

        user_agent.set_position(3.0, 3.0, 3.0);
        user_agent.set_rotation(0.0, 0.0);

        agents.insert(user_agent.id, user_agent);

        agents
    }

    fn setup_world() -> World {
        let world = World::new();

        world
    }

    fn update(&mut self) {
        self.handle_actions();
        self.evolve_time();

        self.physics.update(&mut self.state);

        if let Ok(mut observation) = self.observation.write() {
            observation.update(&self.state);
        } else {
            log::error!("Failed to acquire Observation write lock");
        }
    }

    fn handle_actions(&mut self) {
        while let Ok(action) = self.action_rx.try_recv() {
            match action {
                Action::World(WorldAction::Quit) => {
                    self.handle_quit_action();
                }
                Action::Agent(AgentAction::Movement(movement_actions)) => {
                    self.handle_movement_action(&movement_actions);
                }
                Action::Agent(AgentAction::Jump(jump_action)) => {
                    self.handle_jump_action(&jump_action);
                }
            }
        }
    }

    fn handle_quit_action(&mut self) {
        self.state.active = false;
    }

    fn handle_movement_action(&mut self, movement_actions: &MovementAction) {
        if let Some(agent) = self.state.agents.get_mut(&AgentID::USER_AGENT_ID) {
            agent.z_speed = movement_actions.direction.z;
            agent.x_speed = movement_actions.direction.x;

            if movement_actions.rotation.length_squared() > 1e-6 {
                agent.look_x_axis -= movement_actions.rotation.x;
                agent.look_y_axis += movement_actions.rotation.y;

                let limit = 89.0_f32.to_radians();

                agent.look_x_axis = agent.look_x_axis.clamp(-limit, limit);

                let y_axis_quat = Quat::from_rotation_y(agent.look_y_axis);
                let x_axis_quat = Quat::from_rotation_x(agent.look_x_axis);

                let target_rotation = y_axis_quat * x_axis_quat;

                agent.orientation = agent.orientation.slerp(target_rotation, 0.3);
            }
        }
    }

    fn handle_jump_action(&mut self, jump_action: &JumpAction) {
        match jump_action {
            JumpAction::Start => {
                if let Some(agent) = self.state.agents.get_mut(&AgentID::USER_AGENT_ID) {
                    agent.jump_state.active = true;
                    agent.jump_state.timer = Duration::ZERO;
                    agent.jump_state.cancel = false;

                    self.physics.begin_agent_jump(agent);
                }
            }
            JumpAction::End => {
                if let Some(agent) = self.state.agents.get_mut(&AgentID::USER_AGENT_ID) {
                    agent.jump_state.cancel = true;
                }
            }
        }
    }

    fn evolve_time(&mut self) {
        self.state.time.clock += FIXED_DT;
        self.state.time.tick.advance();
    }
}
