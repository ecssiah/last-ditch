use std::collections::HashMap;
use ultraviolet::Vec3;

use crate::{simulation::{constants::*, state::{Population, State, World, population::{agent::Agent, judge::Judge, spatial::Spatial}}}, utils::ld_math::rand_chacha_ext};

#[derive(Clone)]
pub struct GeneratePopulationData {
    pub stage_index: usize,
    pub stage_cost_map: HashMap<usize, u32>,
}

impl GeneratePopulationData {
    pub fn new() -> Self {
        let stage_index = 0;

        #[rustfmt::skip]
        let stage_cost_map = HashMap::from([
            (0, 100), 
        ]);

        Self {
            stage_index,
            stage_cost_map,
        }
    }

    pub fn cost(generation_data: &Self) -> u32 {
        generation_data.stage_cost_map[&generation_data.stage_index]
    }

    pub fn step(state: &mut State, generation_data: &mut Self) -> bool {
        match generation_data.stage_index {
            0 => {
                Self::setup_agent_map(&state.world, &mut state.population);
            }
            _ => unreachable!(),
        }

        Self::next_stage(generation_data)
    }

    fn next_stage(generation_data: &mut Self) -> bool {
        generation_data.stage_index += 1;

        generation_data.stage_index >= generation_data.stage_cost_map.len()
    }

    fn setup_agent_map(_world: &World, population: &mut Population) {
        population.agent_map.clear();

        let nation_map = population.nation_map.clone();

        for (nation_kind, nation) in nation_map {
            let home_position = Vec3::from(nation.home_position);

            for _ in 1..=AGENT_INITIAL_POPULATION {
                let offset = Vec3::new(
                    rand_chacha_ext::gen_range_f32(-4.0, 4.0, &mut population.rng),
                    rand_chacha_ext::gen_range_f32(-4.0, 4.0, &mut population.rng),
                    0.0,
                );

                let agent_id = Population::get_next_entity_id(population);

                let mut agent = Agent::new(agent_id, nation_kind);

                let world_position = home_position + offset;

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