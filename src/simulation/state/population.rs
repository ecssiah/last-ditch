//! Entities acting in the simulated environment

pub mod identity;
pub mod kinematic;
pub mod leadership;
pub mod nation;
pub mod person;
pub mod sight;
pub mod transform;

use crate::simulation::{
    constants::{
        INITIAL_PERSON_ID, JUDGE_DEFAULT_SIZE_X, JUDGE_DEFAULT_SIZE_Y, JUDGE_DEFAULT_SIZE_Z,
        JUDGE_ID_0, TOWER_RADIUS,
    },
    state::population::{leadership::Leadership, nation::Nation, person::Person},
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

pub struct Population {
    pub active: bool,
    pub rng: ChaCha8Rng,
    pub next_person_id: u64,
    pub person_map: HashMap<u64, Person>,
    pub nation_map: HashMap<nation::Kind, Nation>,
    pub leadership: Leadership,
}

impl Population {
    pub fn new(seed: u64) -> Self {
        let active = false;
        let rng = ChaCha8Rng::seed_from_u64(seed);

        let next_person_id = INITIAL_PERSON_ID;

        let person_map = Self::setup_person_map();
        let nation_map = Self::setup_nation_map();

        let leadership = Leadership {
            judge_id: JUDGE_ID_0,
        };

        Self {
            active,
            rng,
            leadership,
            next_person_id,
            person_map,
            nation_map,
        }
    }

    fn setup_person_map() -> HashMap<u64, Person> {
        let mut judge = Person::new(JUDGE_ID_0);

        Person::set_world_position(Vec3::new(0.0, -8.0, 2.0), &mut judge);
        Person::set_rotation(0.0, 0.0, &mut judge);

        Person::set_size(
            Vec3::new(
                JUDGE_DEFAULT_SIZE_X,
                JUDGE_DEFAULT_SIZE_Y,
                JUDGE_DEFAULT_SIZE_Z,
            ),
            &mut judge,
        );

        let person_map = HashMap::from([(judge.person_id, judge)]);

        person_map
    }

    fn setup_nation_map() -> HashMap<nation::Kind, Nation> {
        let tower_radius = TOWER_RADIUS as i32;

        let home_radius = tower_radius - 6;
        let home_height = 0;

        let wolf_nation = Nation {
            home_position: IVec3::new(home_radius, 0, home_height),
        };

        let lion_nation = Nation {
            home_position: IVec3::new(-home_radius, 0, home_height),
        };

        let eagle_nation = Nation {
            home_position: IVec3::new(0, home_radius, home_height),
        };

        let horse_nation = Nation {
            home_position: IVec3::new(0, -home_radius, home_height),
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

    pub fn tick(population: &mut Self) {
        let _ = tracing::info_span!("population_tick").entered();

        if !population.active {
            return;
        }
    }
}
