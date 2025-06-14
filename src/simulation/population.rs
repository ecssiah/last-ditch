//! Entities acting in the simulated environment

pub mod agent;
pub mod builder;
pub mod decision;
pub mod judge;

pub use agent::Agent;
use crossbeam::channel::{Receiver, Sender};
use glam::IVec3;
pub use judge::Judge;

use crate::simulation::{
    compute::{self, task},
    consts::*,
    time::{Tick, Time},
    world::World,
};
use std::collections::HashMap;

pub struct Population {
    pub tick: Tick,
    pub task_tx: Sender<Box<dyn compute::Task>>,
    pub result_rx: Receiver<Box<dyn compute::Result>>,
    pub judge: Judge,
    pub agent_map: HashMap<agent::ID, Agent>,
}

impl Population {
    pub fn new(
        task_tx: Sender<Box<dyn compute::Task>>,
        result_rx: Receiver<Box<dyn compute::Result>>,
    ) -> Self {
        let tick = Tick::ZERO;
        let judge = Judge::new(judge::ID::allocate());
        let agent_map = HashMap::new();

        Self {
            tick,
            task_tx,
            result_rx,
            judge,
            agent_map,
        }
    }

    pub fn setup(&mut self, world: &World) {
        if TESTING {
            builder::TestPopulation::build(self, &world);
        } else {
            builder::MainPopulation::build(self, &world);
        }
    }

    pub fn tick(&mut self, time: &Time, world: &World) {
        self.tick = time.tick;

        self.tick_agent_map(world);
        self.judge.tick(world);
    }

    fn tick_agent_map(&mut self, world: &World) {
        for agent in self.agent_map.values_mut() {
            agent.tick(world);
        }
    }

    pub fn get_judge(&self) -> &Judge {
        &self.judge
    }

    pub fn get_judge_mut(&mut self) -> &mut Judge {
        &mut self.judge
    }

    pub fn get_agent_map(&self) -> impl Iterator<Item = &Agent> {
        self.agent_map.values()
    }

    pub fn get_agent_map_mut(&mut self) -> impl Iterator<Item = &mut Agent> {
        self.agent_map.values_mut()
    }

    pub fn get_agent(&self, agent_id: &agent::ID) -> Option<&Agent> {
        self.agent_map.get(agent_id)
    }

    pub fn get_agent_mut(&mut self, agent_id: &agent::ID) -> Option<&mut Agent> {
        self.agent_map.get_mut(agent_id)
    }

    pub fn test_pathfinding_action(&mut self, world: &World) {
        let task = task::ChunkPathTask {
            agent_id: agent::ID(0),
            chunk_id: world
                .grid
                .chunk_coordinates_to_chunk_id(IVec3::new(0, 0, 0))
                .unwrap(),
            block_id_from: world
                .grid
                .block_coordinates_to_block_id(IVec3::new(-2, -2, -2))
                .unwrap(),
            block_id_to: world
                .grid
                .block_coordinates_to_block_id(IVec3::new(2, -2, 2))
                .unwrap(),
        };

        self.task_tx.send(Box::new(task)).unwrap();
    }
}
