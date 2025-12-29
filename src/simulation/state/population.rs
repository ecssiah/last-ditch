//! Entities acting in the simulated environment

pub mod identity;
pub mod motion;
pub mod nation;
pub mod person;
pub mod sight;
pub mod transform;

use crate::{
    simulation::{
        constants::*,
        state::{
            physics::{
                body::Body,
                collider::{self, box_collider::BoxCollider},
            },
            population::{nation::Nation, person::Person},
        },
        utils::IDGenerator,
    },
    utils::ldmath::rand_chacha_ext::gen_bool,
};
use rand_chacha::{rand_core::SeedableRng, ChaCha8Rng};
use std::collections::HashMap;
use tracing::instrument;
use ultraviolet::{IVec3, Vec3};

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
        let home_height = 1;

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

    pub fn generate_person(population: &mut Self) -> Person {
        let person_id = IDGenerator::allocate(&mut population.id_generator);

        let mut person = Person::new(person_id);

        person.identity.sex = match gen_bool(&mut population.rng) {
            true => identity::Sex::Female,
            false => identity::Sex::Male,
        };

        let person_radius = Vec3::new(
            PERSON_DEFAULT_RADIUS_X,
            PERSON_DEFAULT_RADIUS_Y,
            PERSON_DEFAULT_RADIUS_Z,
        );

        let core_collider = Body::get_collider_mut(collider::Label::Core, &mut person.body)
            .expect("Body has no core");

        BoxCollider::set_radius(person_radius, core_collider);

        person
    }

    #[instrument(skip_all)]
    pub fn tick(_population: &mut Self) {}
}
