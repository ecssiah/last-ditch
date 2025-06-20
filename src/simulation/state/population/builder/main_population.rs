use glam::Vec3;
use rand::Rng;

use crate::simulation::{
    consts::*,
    state::{
        physics::aabb::AABB,
        population::{entity, Agent, Population},
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

        let judge = &mut population.judge;

        let judge_world_position = Vec3::new(0.0, 2.0, 0.0);
        let judge_size = Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z);
        let judge_aabb = AABB::new(judge_world_position + Vec3::Y * judge_size.y, judge_size);

        judge.spatial.world_position = judge_world_position;
        judge.spatial.aabb = judge_aabb;

        population.judge.set_rotation(0.0, 0.0);
    }

    fn setup_agents(population: &mut Population, world: &World) {
        log::info!("Setup Agents");

        let mut rng = rand::thread_rng();

        for kind in entity::Kind::all() {
            if let Some(flag_position) = world.get_flag(kind) {
                let flag_position = flag_position.as_vec3();

                for _ in 0..AGENT_INITIAL_POPULATION {
                    let offset =
                        Vec3::new(rng.gen_range(-4.0..=4.0), 0.0, rng.gen_range(-4.0..=4.0));
                    let world_position = flag_position + offset;

                    let mut agent = Agent::new();

                    let agent_size =
                        Vec3::new(0.6, rng.gen_range(AGENT_SIZE_MIN..=AGENT_SIZE_MAX), 0.6);
                    let agent_aabb = AABB::new(world_position + Vec3::Y * agent_size.y, agent_size);

                    agent.kind = kind;
                    agent.spatial.world_position = world_position;
                    agent.spatial.aabb = agent_aabb;

                    population.agent_map.insert(agent.id, agent);
                }
            }
        }
    }
}
