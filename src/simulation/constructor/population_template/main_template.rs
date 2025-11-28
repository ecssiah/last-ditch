use crate::{
    simulation::{
        constants::*,
        state::{
            population::{agent::Agent, judge::Judge, nation, sight::Sight, spatial::Spatial},
            world::World,
            Population,
        },
    },
    utils::ld_math::rng_ext::RngExt,
};
use ultraviolet::Vec3;

pub fn construct(world: &World, population: &mut Population) {
    setup_judge(world, population);
    setup_agent_map(world, population);
}

fn setup_judge(world: &World, population: &mut Population) {
    let judge = &mut population.judge;

    Judge::set_world_position(Vec3::new(0.0, 0.0, 1.0), judge);
    Judge::set_rotation(180.0, 0.0, judge);

    Sight::set_range(40.0, &world.grid, &mut judge.sight);
}

fn setup_agent_map(world: &World, population: &mut Population) {
    for nation_kind in nation::Kind::ALL {
        if let Some(flag_position) = world.flag_position_map.get(&nation_kind) {
            let flag_position = Vec3::from(*flag_position);

            for _ in 1..=AGENT_INITIAL_POPULATION {
                let offset = Vec3::new(
                    RngExt::gen_range_f32(-4.0, 4.0, &mut population.rng),
                    RngExt::gen_range_f32(-4.0, 4.0, &mut population.rng),
                    0.0,
                );

                let world_position = flag_position + offset;

                let mut agent = Agent::new(nation_kind);

                Spatial::set_world_position(world_position, &mut agent.spatial);

                let agent_size = Vec3::new(
                    AGENT_DEFAULT_SIZE_X,
                    AGENT_DEFAULT_SIZE_Y,
                    RngExt::gen_range_f32(
                        AGENT_DEFAULT_SIZE_Z - 0.2,
                        AGENT_DEFAULT_SIZE_Z + 0.2,
                        &mut population.rng,
                    ),
                );

                agent.kinematic.speed = AGENT_DEFAULT_SPEED;
                agent.kinematic.jump_speed = AGENT_DEFAULT_JUMP_SPEED;

                Spatial::set_size(agent_size, &mut agent.spatial);

                population.agent_map.insert(agent.id, agent);
            }
        }
    }
}
