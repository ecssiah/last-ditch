use crate::simulation::{
    consts::*,
    state::{
        population::{
            entity::{self, Agent, Judge},
            nation,
        },
        Population,
    },
};
use glam::Vec3;
use std::collections::HashMap;

pub fn construct(population: &mut Population) {
    setup_judge(&mut population.judge);
    setup_agent_map(&mut population.agent_map);
}

fn setup_judge(judge: &mut Judge) {
    Judge::set_world_position(
        Vec3::new(-0.0, -2.0, -0.0),
        &mut judge.spatial,
        &mut judge.detection,
    );

    Judge::set_size(
        Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z),
        &mut judge.spatial,
        &mut judge.detection,
    );
}

fn setup_agent_map(agent_map: &mut HashMap<entity::ID, Agent>) {
    let mut pathfinding_agent1 = Agent::new(nation::Kind::Eagle);
    let mut pathfinding_agent2 = Agent::new(nation::Kind::Wolf);
    let mut pathfinding_agent3 = Agent::new(nation::Kind::Lion);
    let mut pathfinding_agent4 = Agent::new(nation::Kind::Horse);

    Agent::set_world_position(
        Vec3::new(0.0, 6.0, 9.0),
        &mut pathfinding_agent1.spatial,
        &mut pathfinding_agent1.detection,
    );

    Agent::set_size(
        Vec3::new(0.3, 2.5, 0.3),
        &mut pathfinding_agent1.spatial,
        &mut pathfinding_agent1.detection,
    );

    Agent::set_world_position(
        Vec3::new(0.0, 6.0, 9.0),
        &mut pathfinding_agent2.spatial,
        &mut pathfinding_agent2.detection,
    );

    Agent::set_size(
        Vec3::new(0.3, 2.0, 0.3),
        &mut pathfinding_agent2.spatial,
        &mut pathfinding_agent2.detection,
    );

    Agent::set_world_position(
        Vec3::new(0.0, 6.0, 9.0),
        &mut pathfinding_agent3.spatial,
        &mut pathfinding_agent3.detection,
    );

    Agent::set_size(
        Vec3::new(0.3, 1.5, 0.3),
        &mut pathfinding_agent3.spatial,
        &mut pathfinding_agent3.detection,
    );

    Agent::set_world_position(
        Vec3::new(0.0, 6.0, 9.0),
        &mut pathfinding_agent4.spatial,
        &mut pathfinding_agent4.detection,
    );

    Agent::set_size(
        Vec3::new(0.1, 1.0, 0.1),
        &mut pathfinding_agent4.spatial,
        &mut pathfinding_agent4.detection,
    );

    agent_map.insert(pathfinding_agent1.info.entity_id, pathfinding_agent1);
    agent_map.insert(pathfinding_agent2.info.entity_id, pathfinding_agent2);
    agent_map.insert(pathfinding_agent3.info.entity_id, pathfinding_agent3);
    agent_map.insert(pathfinding_agent4.info.entity_id, pathfinding_agent4);

}
