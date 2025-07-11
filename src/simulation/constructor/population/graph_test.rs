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

    Agent::set_world_position(
        Vec3::new(-9.0, -3.0, 0.0),
        &mut pathfinding_agent1.spatial,
        &mut pathfinding_agent1.detection,
    );

    Agent::set_size(Vec3::new(0.3, 1.8, 0.3), &mut pathfinding_agent1.detection);

    agent_map.insert(pathfinding_agent1.info.entity_id, pathfinding_agent1);

    let mut pathfinding_agent2 = Agent::new();
    pathfinding_agent2.info.kind = entity::Kind::Wolf;

    Agent::set_world_position(
        Vec3::new(9.0, -3.0, 0.0),
        &mut pathfinding_agent2.spatial,
        &mut pathfinding_agent2.detection,
    );

    Agent::set_size(Vec3::new(0.3, 2.4, 0.3), &mut pathfinding_agent2.detection);

    agent_map.insert(pathfinding_agent2.info.entity_id, pathfinding_agent2);

    let mut pathfinding_agent3 = Agent::new();
    pathfinding_agent3.info.kind = entity::Kind::Lion;

    Agent::set_world_position(
        Vec3::new(0.0, -3.0, -9.0),
        &mut pathfinding_agent3.spatial,
        &mut pathfinding_agent3.detection,
    );

    Agent::set_size(Vec3::new(0.3, 2.8, 0.3), &mut pathfinding_agent3.detection);

    agent_map.insert(pathfinding_agent3.info.entity_id, pathfinding_agent3);
}

fn setup_judge(judge: &mut Judge) {
    Judge::set_world_position(
        Vec3::new(-3.0, -2.0, -3.0),
        &mut judge.spatial,
        &mut judge.detection,
    );

    Judge::set_size(
        Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z),
        &mut judge.detection,
    );
}
