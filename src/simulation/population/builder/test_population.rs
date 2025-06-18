use crate::simulation::{
    consts::*,
    physics::dynamic_object::DynamicObject,
    population::{agent, Agent, Population},
    world::World,
};
use glam::{IVec3, Vec3};

pub struct TestPopulation {}

impl TestPopulation {
    pub fn build(population: &mut Population, world: &World) {
        Self::setup_judge(population);
        Self::setup_agents(population, world);
    }

    fn setup_judge(population: &mut Population) {
        log::info!("Setup Test Judge");

        population
            .judge
            .set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));
        population.judge.set_world_position(0.0, -2.0, 0.0);
        population.judge.set_rotation(0.0, 0.0);
    }

    fn setup_agents(population: &mut Population, world: &World) {
        log::info!("Setup Test Agents");

        Self::setup_eagle_agents(population, world);
        Self::setup_lion_agents(population, world);
        Self::setup_wolf_agents(population, world);
        Self::setup_horse_agents(population, world);
    }

    fn setup_eagle_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new(agent::ID::allocate());

        let world_position = (chunk_west_position + IVec3::new(2, -2, 2)).as_vec3();

        agent.world_position = world_position;
        agent.target_world_position = world_position;
        agent.kind = agent::Kind::Eagle;
        agent.height = 0.9;

        population.agent_map.insert(agent.id, agent);
    }

    fn setup_horse_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new(agent::ID::allocate());

        let world_position = (chunk_west_position + IVec3::new(-2, -2, 2)).as_vec3();

        agent.world_position = world_position;
        agent.target_world_position = world_position;
        agent.kind = agent::Kind::Horse;
        agent.height = 0.9;

        population.agent_map.insert(agent.id, agent);
    }

    fn setup_wolf_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new(agent::ID::allocate());

        let world_position = (chunk_west_position + IVec3::new(2, -2, -2)).as_vec3();

        agent.world_position = world_position;
        agent.target_world_position = world_position;
        agent.kind = agent::Kind::Wolf;
        agent.height = 0.9;

        population.agent_map.insert(agent.id, agent);
    }

    fn setup_lion_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new(agent::ID::allocate());

        let world_position = (chunk_west_position + IVec3::new(-2, -2, -2)).as_vec3();

        agent.world_position = world_position;
        agent.target_world_position = world_position;
        agent.kind = agent::Kind::Lion;
        agent.height = 0.9;

        population.agent_map.insert(agent.id, agent);
    }
}
