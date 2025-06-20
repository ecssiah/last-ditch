use crate::simulation::{
    consts::*,
    state::{
        physics::aabb::AABB,
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

        let judge_world_position = Vec3::new(0.0, -2.0, 0.0);
        let judge_size = Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z);
        let judge_aabb = AABB::new(judge_world_position + Vec3::Y * judge_size.y, judge_size);

        judge.spatial.world_position = judge_world_position;
        judge.spatial.aabb = judge_aabb;
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

        let agent_size = Vec3::new(0.6, 1.8, 0.6);
        let agent_aabb = AABB::new(world_position + Vec3::Y * agent_size.y, agent_size);

        agent.kind = entity::Kind::Eagle;
        agent.spatial.world_position = world_position;
        agent.spatial.aabb = agent_aabb;

        population.agent_map.insert(agent.id, agent);
    }

    fn setup_horse_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new();

        let world_position = (chunk_west_position + IVec3::new(-2, -2, 2)).as_vec3();

        let agent_size = Vec3::new(0.6, 1.8, 0.6);
        let agent_aabb = AABB::new(world_position + Vec3::Y * agent_size.y, agent_size);

        agent.kind = entity::Kind::Horse;
        agent.spatial.world_position = world_position;
        agent.spatial.aabb = agent_aabb;

        population.agent_map.insert(agent.id, agent);
    }

    fn setup_wolf_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new();

        let world_position = (chunk_west_position + IVec3::new(2, -2, -2)).as_vec3();

        let agent_size = Vec3::new(0.6, 1.8, 0.6);
        let agent_aabb = AABB::new(world_position + Vec3::Y * agent_size.y, agent_size);

        agent.kind = entity::Kind::Wolf;
        agent.spatial.world_position = world_position;
        agent.spatial.aabb = agent_aabb;

        population.agent_map.insert(agent.id, agent);
    }

    fn setup_lion_agents(population: &mut Population, world: &World) {
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new();

        let world_position = (chunk_west_position + IVec3::new(-2, -2, -2)).as_vec3();

        let agent_size = Vec3::new(0.6, 1.8, 0.6);
        let agent_aabb = AABB::new(world_position + Vec3::Y * agent_size.y, agent_size);

        agent.kind = entity::Kind::Lion;
        agent.spatial.world_position = world_position;
        agent.spatial.aabb = agent_aabb;

        population.agent_map.insert(agent.id, agent);
    }
}
