//! Entities acting in the simulated environment

pub mod identity;
pub mod kinematic;
pub mod nation;
pub mod person;
pub mod sight;
pub mod spatial;

use ultraviolet::{IVec3, Vec3};

use crate::simulation::{
    constants::{JUDGE_DEFAULT_SIZE_X, JUDGE_DEFAULT_SIZE_Y, JUDGE_DEFAULT_SIZE_Z},
    state::{
        population::{nation::Nation, person::Person, sight::Sight, spatial::Spatial},
        world::World,
    },
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::collections::HashMap;

pub struct Population {
    pub active: bool,
    pub rng: ChaCha8Rng,
    pub judge_id: u64,
    pub next_person_id: u64,
    pub person_map: HashMap<u64, Person>,
    pub nation_map: HashMap<nation::Kind, Nation>,
}

impl Population {
    pub fn new(seed: u64) -> Self {
        let active = false;
        let rng = ChaCha8Rng::seed_from_u64(seed);

        let judge_id = 0;
        let next_person_id = 1;

        let person_map = Self::setup_person_map();
        let nation_map = Self::setup_nation_map();

        Self {
            active,
            rng,
            judge_id,
            next_person_id,
            person_map,
            nation_map,
        }
    }

    fn setup_person_map() -> HashMap<u64, Person> {
        let mut judge = Person::new(0);

        Person::set_world_position(Vec3::new(0.0, -8.0, 2.0), &mut judge);
        Person::set_rotation(0.0, 0.0, &mut judge);

        Spatial::set_size(
            Vec3::new(
                JUDGE_DEFAULT_SIZE_X,
                JUDGE_DEFAULT_SIZE_Y,
                JUDGE_DEFAULT_SIZE_Z,
            ),
            &mut judge.spatial,
        );

        Sight::set_range(100.0, &mut judge.sight);

        let person_map = HashMap::from([(0, judge)]);

        person_map
    }

    fn setup_nation_map() -> HashMap<nation::Kind, Nation> {
        let radius = 20;

        let wolf_nation = Nation {
            home_position: IVec3::new(radius, 0, 0),
        };

        let lion_nation = Nation {
            home_position: IVec3::new(-radius, 0, 0),
        };

        let eagle_nation = Nation {
            home_position: IVec3::new(0, radius, 0),
        };

        let horse_nation = Nation {
            home_position: IVec3::new(0, -radius, 0),
        };

        let nation_map = HashMap::from([
            (nation::Kind::Wolf, wolf_nation),
            (nation::Kind::Lion, lion_nation),
            (nation::Kind::Eagle, eagle_nation),
            (nation::Kind::Horse, horse_nation),
        ]);

        nation_map
    }

    pub fn get_next_entity_id(population: &mut Self) -> u64 {
        let person_id = population.next_person_id;

        population.next_person_id += 1;

        person_id
    }

    pub fn tick(world: &World, population: &mut Self) {
        let _ = tracing::info_span!("population_tick").entered();

        if !population.active {
            return;
        }

        for person in population.person_map.values_mut() {
            Person::tick(world, person);
        }
    }
}
