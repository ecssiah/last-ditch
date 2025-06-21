use crate::simulation::{
    consts::*,
    state::{
        population::{entity, Agent, Population},
        world::World,
    },
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

        let judge = &mut population.judge;

        judge.set_world_position(Vec3::new(0.0, -2.0, 0.0));
        judge.set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));
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

        let mut agent = Agent::new();

        let world_position = (chunk_west_position + IVec3::new(2, -2, 2)).as_vec3();

        agent.kind = entity::Kind::Eagle;
        agent.set_world_position(world_position);
        agent.set_size(Vec3::new(0.6, 1.8, 0.6));

        population.agent_map.insert(agent.id, agent);
    }

    fn setup_horse_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new();

        let world_position = (chunk_west_position + IVec3::new(-2, -2, 2)).as_vec3();

        agent.kind = entity::Kind::Horse;
        agent.set_world_position(world_position);
        agent.set_size(Vec3::new(0.6, 1.8, 0.6));

        population.agent_map.insert(agent.id, agent);
    }

    fn setup_wolf_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new();

        let world_position = (chunk_west_position + IVec3::new(2, -2, -2)).as_vec3();

        agent.kind = entity::Kind::Wolf;
        agent.set_world_position(world_position);
        agent.set_size(Vec3::new(0.6, 1.8, 0.6));

        population.agent_map.insert(agent.id, agent);
    }

    fn setup_lion_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new();

        let world_position = (chunk_west_position + IVec3::new(-2, -2, -2)).as_vec3();

        agent.kind = entity::Kind::Lion;
        agent.set_world_position(world_position);
        agent.set_size(Vec3::new(0.6, 1.8, 0.6));

        population.agent_map.insert(agent.id, agent);
    }
}
