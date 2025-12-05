use std::collections::HashMap;
use ultraviolet::Vec3;

use crate::{
    simulation::{
        constants::*,
        state::{
            population::{person::Person, spatial::Spatial},
            Population, State, World,
        },
    },
    utils::ld_math::rand_chacha_ext,
};

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

    pub fn cost(generate_population_data: &Self) -> u32 {
        generate_population_data.stage_cost_map[&generate_population_data.stage_index]
    }

    pub fn step(state: &mut State, generate_population_data: &mut Self) -> bool {
        match generate_population_data.stage_index {
            0 => {
                Self::setup_person_map(&state.world, &mut state.population);
            }
            _ => unreachable!(),
        }

        Self::next_stage(generate_population_data)
    }

    fn next_stage(generate_population_data: &mut Self) -> bool {
        generate_population_data.stage_index += 1;

        generate_population_data.stage_index >= generate_population_data.stage_cost_map.len()
    }

    fn setup_person_map(_world: &World, population: &mut Population) {
        let nation_map = population.nation_map.clone();

        for nation in nation_map.values() {
            let home_position = Vec3::from(nation.home_position);

            for _ in 1..=INITIAL_NATION_POPULATION {
                let offset = Vec3::new(
                    rand_chacha_ext::gen_range_f32(-4.0, 4.0, &mut population.rng),
                    rand_chacha_ext::gen_range_f32(-4.0, 4.0, &mut population.rng),
                    0.0,
                );

                let person_id = Population::get_next_entity_id(population);

                let mut person = Person::new(person_id);

                let world_position = home_position + offset;

                Spatial::set_world_position(world_position, &mut person.spatial);

                let person_size = Vec3::new(
                    PERSON_DEFAULT_SIZE_X,
                    PERSON_DEFAULT_SIZE_Y,
                    rand_chacha_ext::gen_range_f32(
                        PERSON_DEFAULT_SIZE_Z - 0.2,
                        PERSON_DEFAULT_SIZE_Z + 0.2,
                        &mut population.rng,
                    ),
                );

                person.kinematic.speed = PERSON_DEFAULT_SPEED;
                person.kinematic.jump_speed = PERSON_DEFAULT_JUMP_SPEED;

                Spatial::set_size(person_size, &mut person.spatial);

                population.person_map.insert(person.person_id, person);
            }
        }
    }
}
