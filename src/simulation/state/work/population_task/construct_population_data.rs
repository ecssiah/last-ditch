use crate::{
    simulation::{
        constants::*,
        state::{
            population::{agent::Agent, judge::Judge, nation, sight::Sight, spatial::Spatial},
            Population, World,
        },
    },
    utils::ld_math::rand_chacha_ext,
};
use ultraviolet::Vec3;

#[derive(Clone)]
pub struct ConstructPopulationData {
    pub stage: usize,
}

impl ConstructPopulationData {
    pub fn cost(construct_population_data: &ConstructPopulationData) -> u32 {
        match construct_population_data.stage {
            0 => 10,
            1 => 10,
            _ => 0,
        }
    }

    pub fn step(
        world: &World,
        population: &mut Population,
        construct_population_data: &mut ConstructPopulationData,
    ) -> bool {
        match construct_population_data.stage {
            0 => {
                ConstructPopulationData::setup_judge(world, population);

                construct_population_data.stage += 1;

                false
            }
            1 => {
                ConstructPopulationData::setup_agent_map(world, population);

                construct_population_data.stage += 1;

                false
            }
            _ => true,
        }
    }

    pub fn setup_judge(_world: &World, population: &mut Population) {
        let judge = &mut population.judge;

        Judge::set_world_position(Vec3::new(0.0, 0.0, 1.0), judge);
        Judge::set_rotation(0.0, 0.0, judge);

        Sight::set_range(40.0, &mut judge.sight);
    }

    pub fn setup_agent_map(world: &World, population: &mut Population) {
        for nation_kind in nation::Kind::ALL {
            if let Some(flag_position) = world.flag_position_map.get(&nation_kind) {
                let flag_position = Vec3::from(*flag_position);

                for _ in 1..=AGENT_INITIAL_POPULATION {
                    let offset = Vec3::new(
                        rand_chacha_ext::gen_range_f32(-4.0, 4.0, &mut population.rng),
                        rand_chacha_ext::gen_range_f32(-4.0, 4.0, &mut population.rng),
                        0.0,
                    );

                    let agent_id = Population::get_next_entity_id(population);

                    let mut agent = Agent::new(agent_id, nation_kind);

                    let world_position = flag_position + offset;

                    Spatial::set_world_position(world_position, &mut agent.spatial);

                    let agent_size = Vec3::new(
                        AGENT_DEFAULT_SIZE_X,
                        AGENT_DEFAULT_SIZE_Y,
                        rand_chacha_ext::gen_range_f32(
                            AGENT_DEFAULT_SIZE_Z - 0.2,
                            AGENT_DEFAULT_SIZE_Z + 0.2,
                            &mut population.rng,
                        ),
                    );

                    agent.kinematic.speed = AGENT_DEFAULT_SPEED;
                    agent.kinematic.jump_speed = AGENT_DEFAULT_JUMP_SPEED;

                    Spatial::set_size(agent_size, &mut agent.spatial);

                    population.agent_map.insert(agent.entity_id, agent);
                }
            }
        }
    }
}
