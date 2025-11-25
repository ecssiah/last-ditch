use crate::simulation::{
    constants::*,
    state::{
        population::{
            agent::{self, Agent},
            judge::Judge,
            nation,
            spatial::Spatial,
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
    Spatial::set_world_position(Vec3::new(-0.0, -2.0, -0.0), &mut judge.spatial);

    Spatial::set_size(
        Vec3::new(
            JUDGE_DEFAULT_SIZE_X,
            JUDGE_DEFAULT_SIZE_Y,
            JUDGE_DEFAULT_SIZE_Z,
        ),
        &mut judge.spatial,
    );
}

fn setup_agent_map(agent_map: &mut HashMap<agent::ID, Agent>) {
    let mut pathfinding_agent1 = Agent::new(nation::Kind::Eagle);
    let mut pathfinding_agent2 = Agent::new(nation::Kind::Wolf);
    let mut pathfinding_agent3 = Agent::new(nation::Kind::Lion);
    let mut pathfinding_agent4 = Agent::new(nation::Kind::Horse);

    Spatial::set_world_position(Vec3::new(0.0, 6.0, 9.0), &mut pathfinding_agent1.spatial);
    Spatial::set_size(Vec3::new(0.3, 2.5, 0.3), &mut pathfinding_agent1.spatial);
    Spatial::set_world_position(Vec3::new(0.0, 6.0, 9.0), &mut pathfinding_agent2.spatial);
    Spatial::set_size(Vec3::new(0.3, 2.0, 0.3), &mut pathfinding_agent2.spatial);
    Spatial::set_world_position(Vec3::new(0.0, 6.0, 9.0), &mut pathfinding_agent3.spatial);
    Spatial::set_size(Vec3::new(0.3, 1.5, 0.3), &mut pathfinding_agent3.spatial);
    Spatial::set_world_position(Vec3::new(0.0, 6.0, 9.0), &mut pathfinding_agent4.spatial);
    Spatial::set_size(Vec3::new(0.1, 1.0, 0.1), &mut pathfinding_agent4.spatial);

    agent_map.insert(pathfinding_agent1.id, pathfinding_agent1);
    agent_map.insert(pathfinding_agent2.id, pathfinding_agent2);
    agent_map.insert(pathfinding_agent3.id, pathfinding_agent3);
    agent_map.insert(pathfinding_agent4.id, pathfinding_agent4);
}
