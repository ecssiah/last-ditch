//! Entities acting in the simulated environment

pub mod identity;
pub mod kinematic;
pub mod nation;
pub mod person;
pub mod sight;
pub mod transform;

use crate::simulation::{
    constants::TOWER_RADIUS,
    state::population::{nation::Nation, person::Person},
    utils::IDGenerator,
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::collections::HashMap;
use ultraviolet::IVec3;

pub struct Population {
    pub active: bool,
    pub rng: ChaCha8Rng,
    pub id_generator: IDGenerator,
    pub nation_map: HashMap<nation::Kind, Nation>,
    pub person_map: HashMap<u64, Person>,
}

impl Population {
    pub fn new(seed: u64) -> Self {
        let active = false;
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let id_generator = IDGenerator::new();
        let nation_map = Self::setup_nation_map();
        let person_map = HashMap::new();

        Self {
            active,
            rng,
            id_generator,
            nation_map,
            person_map,
        }
    }

    pub fn reset(population: &mut Self) {
        population.nation_map = Self::setup_nation_map();
        population.person_map = HashMap::new();
    }

    fn setup_nation_map() -> HashMap<nation::Kind, Nation> {
        let tower_radius = TOWER_RADIUS as i32;

        let home_radius = tower_radius - 10;
        let home_height = 0;

        let mut lion_nation = Nation::new(nation::Kind::Lion);
        lion_nation.home_grid_position = IVec3::new(0, home_radius, home_height);

        let mut eagle_nation = Nation::new(nation::Kind::Eagle);
        eagle_nation.home_grid_position = IVec3::new(-home_radius, 0, home_height);

        let mut horse_nation = Nation::new(nation::Kind::Horse);
        horse_nation.home_grid_position = IVec3::new(0, -home_radius, home_height);

        let mut wolf_nation = Nation::new(nation::Kind::Wolf);
        wolf_nation.home_grid_position = IVec3::new(home_radius, 0, home_height);

        let nation_map = HashMap::from([
            (nation::Kind::Lion, lion_nation),
            (nation::Kind::Eagle, eagle_nation),
            (nation::Kind::Horse, horse_nation),
            (nation::Kind::Wolf, wolf_nation),
        ]);

        nation_map
    }

    pub fn tick(population: &mut Self) {
        let _ = tracing::info_span!("population_tick").entered();

        if !population.active {
            return;
        }
    }
}
