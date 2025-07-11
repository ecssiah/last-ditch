use crate::simulation::{
    consts::*,
    state::{
        population::{
            entity::{self, Agent, Judge},
            Population,
        },
        world::World,
    },
};
use glam::Vec3;
use rand::Rng;

pub fn construct(population: &mut Population, world: &World) {
    setup_judge(population);
    setup_agents(population, world);
}

fn setup_judge(population: &mut Population) {
    let judge = &mut population.judge;

    Judge::set_world_position(
        Vec3::new(0.0, 2.0, 0.0),
        &mut judge.spatial,
        &mut judge.detection,
    );

    Judge::set_size(
        Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z),
        &mut judge.detection,
    );

    Judge::set_rotation(0.0, 0.0, &mut judge.spatial, &mut judge.kinematic);
}

fn setup_agents(population: &mut Population, world: &World) {
    let mut rng = rand::thread_rng();

    let agent_initial_population = 16;
    let agent_size_bounds = (0.6, 2.2);

    for kind in entity::Kind::all() {
        if let Some(flag_position) = world.flag_position_map.get(&kind) {
            let flag_position = flag_position.as_vec3();

            for _ in 0..agent_initial_population {
                let offset = Vec3::new(
                    rng.gen_range(-4..=4) as f32,
                    0.0,
                    rng.gen_range(-4..=4) as f32,
                );

                let world_position = flag_position + offset;

                let mut agent = Agent::new();

                agent.info.kind = kind;

                Agent::set_world_position(world_position, &mut agent.spatial, &mut agent.detection);

                Agent::set_size(
                    Vec3::new(
                        0.6,
                        rng.gen_range(agent_size_bounds.0..=agent_size_bounds.1),
                        0.6,
                    ),
                    &mut agent.detection,
                );

                population.agent_map.insert(agent.info.entity_id, agent);
            }
        }
    }
}
