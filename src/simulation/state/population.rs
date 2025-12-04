//! Entities acting in the simulated environment

pub mod agent;
pub mod identity;
pub mod judge;
pub mod kinematic;
pub mod nation;
pub mod role;
pub mod sight;
pub mod spatial;

pub use role::Role;
use ultraviolet::{IVec3, Vec3};

use crate::simulation::state::{
    navigation::Navigation,
    population::{agent::Agent, judge::Judge, nation::Nation, sight::Sight},
    world::World,
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::collections::HashMap;

pub struct Population {
    pub active: bool,
    pub rng: ChaCha8Rng,
    pub judge: Judge,
    pub agent_map: HashMap<u64, Agent>,
    pub nation_map: HashMap<nation::Kind, Nation>,
    pub next_entity_id: u64,
}

impl Population {
    pub fn new(seed: u64) -> Self {
        let active = false;
        let rng = ChaCha8Rng::seed_from_u64(seed);
        let judge = Self::setup_judge();
        let agent_map = HashMap::new();
        let nation_map = Self::setup_nation_map();
        let next_entity_id = 1;

        Self {
            active,
            next_entity_id,
            rng,
            judge,
            agent_map,
            nation_map,
        }
    }

    fn setup_judge() -> Judge {
        let mut judge = Judge::new(0);

        Judge::set_world_position(Vec3::new(0.0, 0.0, 0.0), &mut judge);
        Judge::set_rotation(0.0, 0.0, &mut judge);

        Sight::set_range(100.0, &mut judge.sight);

        judge
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
        let entity_id = population.next_entity_id;

        population.next_entity_id += 1;

        entity_id
    }

    pub fn tick(world: &World, navigation: &mut Navigation, population: &mut Self) {
        let _ = tracing::info_span!("population_tick").entered();

        if !population.active {
            return;
        }

        Judge::tick(world, &mut population.judge);

        for agent in population.agent_map.values_mut() {
            Agent::tick(navigation, agent);
        }
    }
}
