use glam::Vec3;
use rand::Rng;

use crate::simulation::{
    consts::*,
    state::{
        physics::dynamic_object::DynamicObject,
        population::{agent, Agent, Population},
        world::World,
    },
};

pub struct MainPopulation {}

impl MainPopulation {
    pub fn build(population: &mut Population, world: &World) {
        Self::setup_judge(population);
        Self::setup_agents(population, world);
    }

    fn setup_judge(population: &mut Population) {
        log::info!("Setup Judge");

        population
            .judge
            .set_size(Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z));
        population.judge.set_world_position(0.0, 2.0, 0.0);
        population.judge.set_rotation(0.0, 0.0);
    }

    fn setup_agents(population: &mut Population, world: &World) {
        log::info!("Setup Agents");

        let mut rng = rand::thread_rng();

        for kind in agent::Kind::all() {
            if let Some(flag_position) = world.get_flag(kind) {
                let flag_position = flag_position.as_vec3();

                for _ in 0..AGENT_INITIAL_POPULATION {
                    let offset =
                        Vec3::new(rng.gen_range(-4.0..=4.0), 0.0, rng.gen_range(-4.0..=4.0));
                    let world_position = flag_position + offset;

                    let mut agent = Agent::new(agent::ID::allocate());

                    agent.world_position = world_position;
                    agent.target_world_position = world_position;
                    agent.kind = kind;
                    agent.height = rng.gen_range(AGENT_SIZE_MIN..=AGENT_SIZE_MAX);

                    population.agent_map.insert(agent.id, agent);
                }
            }
        }
    }
}
