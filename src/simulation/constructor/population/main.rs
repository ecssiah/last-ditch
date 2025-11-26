use crate::simulation::{
    constants::*,
    state::{
        population::{agent::Agent, nation, sight::Sight, spatial::Spatial},
        world::World,
        Population,
    },
};
use rand::Rng;
use ultraviolet::Vec3;

pub fn run(world: &World, population: &mut Population) {
    setup_judge(world, population);
    setup_agent_map(world, population);
}

fn setup_judge(world: &World, population: &mut Population) {
    let judge = &mut population.judge;

    Spatial::set_world_position(Vec3::new(0.0, 0.0, 6.0), &mut judge.spatial);

    Spatial::set_size(
        Vec3::new(
            JUDGE_DEFAULT_SIZE_X,
            JUDGE_DEFAULT_SIZE_Y,
            JUDGE_DEFAULT_SIZE_Z,
        ),
        &mut judge.spatial,
    );

    judge.kinematic.speed = JUDGE_DEFAULT_SPEED;
    judge.kinematic.jump_speed = JUDGE_DEFAULT_JUMP_SPEED;

    Spatial::set_rotation(0.0, 0.0, &mut judge.spatial);
    Sight::set_range(20.0, &world.grid, &mut judge.sight);
}

fn setup_agent_map(world: &World, population: &mut Population) {
    let mut rng = rand::thread_rng();

    for nation_kind in nation::Kind::all() {
        if let Some(flag_position) = world.flag_position_map.get(&nation_kind) {
            let flag_position = Vec3::from(*flag_position);

            for _ in 1..=AGENT_INITIAL_POPULATION {
                let offset = Vec3::new(
                    rng.gen_range(-4..=4) as f32,
                    rng.gen_range(-4..=4) as f32,
                    0.0,
                );

                let world_position = flag_position + offset;

                let mut agent = Agent::new(nation_kind);

                Spatial::set_world_position(world_position, &mut agent.spatial);

                let agent_size = Vec3::new(
                    AGENT_DEFAULT_SIZE_X,
                    AGENT_DEFAULT_SIZE_Y,
                    rng.gen_range((AGENT_DEFAULT_SIZE_Z - 0.2)..=(AGENT_DEFAULT_SIZE_Z + 0.2)),
                );

                agent.kinematic.speed = AGENT_DEFAULT_SPEED;
                agent.kinematic.jump_speed = AGENT_DEFAULT_JUMP_SPEED;

                Spatial::set_size(agent_size, &mut agent.spatial);

                population.agent_map.insert(agent.id, agent);
            }
        }
    }
}
