use crate::simulation::{
    consts::*,
    state::{
        population::{
            entity::{Agent, Judge},
            nation, Population,
        },
        world::World,
    },
};
use rand::Rng;
use ultraviolet::Vec3;

pub fn run(world: &World, population: &mut Population) {
    setup_judge(population);
    setup_agent_map(world, population);
}

fn setup_judge(population: &mut Population) {
    let judge = &mut population.judge;

    Judge::set_world_position(
        Vec3::new(0.0, 0.0, 6.0),
        &mut judge.spatial,
        &mut judge.detection,
    );

    Judge::set_size(
        Vec3::new(JUDGE_SIZE_X, JUDGE_SIZE_Y, JUDGE_SIZE_Z),
        &mut judge.spatial,
        &mut judge.detection,
    );

    Judge::set_rotation(0.0, 0.0, &mut judge.spatial, &mut judge.kinematic);
}

fn setup_agent_map(world: &World, population: &mut Population) {
    let mut rng = rand::thread_rng();

    let agent_initial_population = 16;
    let agent_size_bounds = (1.0, 1.0);

    for nation_kind in nation::Kind::all() {
        if let Some(flag_position) = world.flag_position_map.get(&nation_kind) {
            let flag_position = Vec3::from(*flag_position);

            for _ in 0..agent_initial_population {
                let offset = Vec3::new(
                    rng.gen_range(-4..=4) as f32,
                    0.0,
                    rng.gen_range(-4..=4) as f32,
                );

                let world_position = flag_position + offset;

                let mut agent = Agent::new(nation_kind);

                Agent::set_world_position(world_position, &mut agent.spatial, &mut agent.detection);

                let agent_height = rng.gen_range(agent_size_bounds.0..=agent_size_bounds.1);

                Agent::set_size(
                    Vec3::new(0.6, agent_height, 0.6),
                    &mut agent.spatial,
                    &mut agent.detection,
                );

                population.agent_map.insert(agent.info.entity_id, agent);
            }
        }
    }
}
