//! Entities acting in the simulated environment

pub mod builder;
pub mod entity;

use crate::simulation::{
    consts::*,
    state::{
        compute::{result, task},
        population::entity::{Agent, Judge},
        world::World,
        Compute,
    },
};
use crossbeam::channel::{Receiver, Sender};
use glam::IVec3;
use std::collections::HashMap;

pub struct Population {
    pub task_tx: Sender<task::Kind>,
    pub result_rx: Receiver<result::Kind>,
    pub judge: Judge,
    pub agent_map: HashMap<entity::ID, Agent>,
}

impl Population {
    pub fn new(compute: &Compute) -> Self {
        let task_tx = compute.task_tx.clone();
        let result_rx = compute.result_rx.clone();
        let judge = Judge::new();
        let agent_map = HashMap::new();

        Self {
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

    pub fn tick(&mut self, world: &World) {
        while let Ok(result) = self.result_rx.try_recv() {
            match result {
                result::Kind::ChunkPath(result) => {
                    println!("{:?}", result);
                }
                result::Kind::WorldPath(_result) => {}
            }
        }

        for agent in self.agent_map.values_mut() {
            agent.tick(world);
        }

        self.judge.tick(world);
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

    pub fn get_agent(&self, agent_id: &entity::ID) -> Option<&Agent> {
        self.agent_map.get(agent_id)
    }

    pub fn get_agent_mut(&mut self, agent_id: &entity::ID) -> Option<&mut Agent> {
        self.agent_map.get_mut(agent_id)
    }

    pub fn test_chunk_path(&mut self, world: &World) {
        let task = task::ChunkPathTask {
            agent_id: entity::ID::default(),
            chunk_id: world
                .grid
                .chunk_coordinates_to_chunk_id(IVec3::new(0, 0, 0))
                .unwrap(),
            block_id_start: world
                .grid
                .block_coordinates_to_block_id(IVec3::new(-2, -2, -2))
                .unwrap(),
            block_id_end: world
                .grid
                .block_coordinates_to_block_id(IVec3::new(2, -2, 2))
                .unwrap(),
        };

        self.task_tx.send(task::Kind::ChunkPath(task)).unwrap();
    }
}
