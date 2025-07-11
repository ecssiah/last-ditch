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
    let mut pathfinding_agent1 = Agent::new();
    pathfinding_agent1.info.kind = entity::Kind::Eagle;

    pathfinding_agent1.set_world_position(Vec3::new(-9.0, -3.0, 0.0));
    pathfinding_agent1.set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));

    agent_map.insert(pathfinding_agent1.info.entity_id, pathfinding_agent1);

    let mut pathfinding_agent2 = Agent::new();
    pathfinding_agent2.info.kind = entity::Kind::Wolf;

    pathfinding_agent2.set_world_position(Vec3::new(9.0, -3.0, 0.0));
    pathfinding_agent2.set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));

    agent_map.insert(pathfinding_agent2.info.entity_id, pathfinding_agent2);

    let mut pathfinding_agent3 = Agent::new();
    pathfinding_agent3.info.kind = entity::Kind::Lion;

    pathfinding_agent3.set_world_position(Vec3::new(0.0, -3.0, -9.0));
    pathfinding_agent3.set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));

    agent_map.insert(pathfinding_agent3.info.entity_id, pathfinding_agent3);
}

fn setup_judge(judge: &mut Judge) {
    judge.set_world_position(Vec3::new(-3.0, -2.0, -3.0));
    judge.set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));
}
