use crate::simulation::{
    physics::dynamic_object::DynamicObject,
    population::{agent, Agent, Population},
    world::World,
};
use glam::IVec3;

pub struct TestPopulation {}

impl TestPopulation {
    pub fn build(population: &mut Population, world: &World) {
        Self::setup_judge(population);
        Self::setup_agents(population, world);
    }

    fn setup_judge(population: &mut Population) {
        log::info!("Setup Judge");

        population.judge.set_world_position(0.0, 0.0, 0.0);
        population.judge.set_rotation(0.0, 0.0);
    }

    fn setup_agents(population: &mut Population, world: &World) {
        log::info!("Setup Test Agents");

        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        let mut agent = Agent::new(agent::ID::allocate());

        let world_position = (chunk_west_position + IVec3::new(0, -2, 0)).as_vec3();

        agent.world_position = world_position;
        agent.target_world_position = world_position;
        agent.kind = agent::Kind::Eagle;
        agent.height = 0.9;

        population.agent_map.insert(agent.id, agent);
    }
}
