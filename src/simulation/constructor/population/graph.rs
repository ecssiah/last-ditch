use crate::simulation::{
    constants::*,
    state::{
        population::{
            agent::{self, Agent},
            entity::{nation, Entity},
            judge::Judge,
        },
        Population,
    },
};
use std::collections::HashMap;
use ultraviolet::Vec3;

pub fn run(population: &mut Population) {
    setup_judge(&mut population.judge);
    setup_agent_map(&mut population.agent_map);
}

fn setup_judge(judge: &mut Judge) {
    Entity::set_world_position(Vec3::new(-0.0, -2.0, -0.0), &mut judge.entity);

    Entity::set_size(
        Vec3::new(
            JUDGE_DEFAULT_SIZE_X,
            JUDGE_DEFAULT_SIZE_Y,
            JUDGE_DEFAULT_SIZE_Z,
        ),
        &mut judge.entity,
    );
}

fn setup_agent_map(agent_map: &mut HashMap<agent::ID, Agent>) {
    let mut pathfinding_agent1 = Agent::new(nation::Kind::Eagle);
    let mut pathfinding_agent2 = Agent::new(nation::Kind::Wolf);
    let mut pathfinding_agent3 = Agent::new(nation::Kind::Lion);
    let mut pathfinding_agent4 = Agent::new(nation::Kind::Horse);

    Entity::set_world_position(Vec3::new(0.0, 6.0, 9.0), &mut pathfinding_agent1.entity);
    Entity::set_size(Vec3::new(0.3, 2.5, 0.3), &mut pathfinding_agent1.entity);

    Entity::set_world_position(Vec3::new(0.0, 6.0, 9.0), &mut pathfinding_agent2.entity);
    Entity::set_size(Vec3::new(0.3, 2.0, 0.3), &mut pathfinding_agent2.entity);

    Entity::set_world_position(Vec3::new(0.0, 6.0, 9.0), &mut pathfinding_agent3.entity);
    Entity::set_size(Vec3::new(0.3, 1.5, 0.3), &mut pathfinding_agent3.entity);

    Entity::set_world_position(Vec3::new(0.0, 6.0, 9.0), &mut pathfinding_agent4.entity);
    Entity::set_size(Vec3::new(0.1, 1.0, 0.1), &mut pathfinding_agent4.entity);

    agent_map.insert(pathfinding_agent1.agent_id, pathfinding_agent1);
    agent_map.insert(pathfinding_agent2.agent_id, pathfinding_agent2);
    agent_map.insert(pathfinding_agent3.agent_id, pathfinding_agent3);
    agent_map.insert(pathfinding_agent4.agent_id, pathfinding_agent4);
}
