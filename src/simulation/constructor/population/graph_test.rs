use crate::simulation::{
    consts::*,
    state::{
        population::entity::{self, Agent, Judge},
        Population, World,
    },
};
use glam::Vec3;
use std::collections::HashMap;

pub fn construct(population: &mut Population, _world: &World) {
    setup_agent_map(&mut population.agent_map);

    setup_judge(&mut population.judge);
}

fn setup_agent_map(agent_map: &mut HashMap<entity::ID, Agent>) {
    let mut pathfinding_agent = Agent::new();

    pathfinding_agent.set_world_position(Vec3::new(-9.0, -3.0, 0.0));
    pathfinding_agent.set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));

    agent_map.insert(pathfinding_agent.id, pathfinding_agent);
}

fn setup_judge(judge: &mut Judge) {
    judge.set_world_position(Vec3::new(0.0, -2.0, 0.0));
    judge.set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));
}
