use crate::{
    simulation::{
        constants::*,
        state::{
            population::{
                agent::Agent,
                judge::Judge,
                nation::{self, Nation},
                sight::Sight,
                spatial::Spatial,
            },
            world::{block, grid},
            Population, State, World,
        },
    },
    utils::ld_math::rand_chacha_ext,
};
use ultraviolet::{IVec3, Vec3};

#[derive(Clone)]
pub struct GenerationData {
    pub stage: usize,
}

impl GenerationData {
    pub fn new() -> Self {
        let stage = 1;

        Self { stage }
    }

    pub fn cost(generation_data: &GenerationData) -> u32 {
        match generation_data.stage {
            1 => 100,
            2 => 100,
            3 => 100,
            4 => 100,
            5 => 100,
            6 => 100,
            _ => panic!("Requesting an invalid state cost"),
        }
    }

    pub fn step(state: &mut State, generation_data: &mut GenerationData) -> bool {
        match generation_data.stage {
            1 => {
                GenerationData::build_central_stage(&mut state.world);
                GenerationData::build_ground(&mut state.world);

                generation_data.stage += 1;

                false
            }
            2 => {
                GenerationData::build_compass(&mut state.world);

                GenerationData::build_temple(34, 0, 0, nation::Kind::Wolf, &mut state.world);
                GenerationData::build_temple(-34, 0, 0, nation::Kind::Lion, &mut state.world);
                GenerationData::build_temple(0, 34, 0, nation::Kind::Eagle, &mut state.world);
                GenerationData::build_temple(0, -34, 0, nation::Kind::Horse, &mut state.world);

                generation_data.stage += 1;

                false
            }
            3 => {
                GenerationData::build_observation_deck(&mut state.world);

                generation_data.stage += 1;

                false
            }
            4 => {
                GenerationData::setup_nations(&mut state.population);

                generation_data.stage += 1;

                false
            }
            5 => {
                GenerationData::setup_judge(&mut state.population);
                GenerationData::setup_agent_map(&state.world, &mut state.population);

                true
            }
            _ => unreachable!(),
        }
    }

    pub fn build_ground(world: &mut World) {
        let ground_boundary = (WORLD_RADIUS_IN_CELLS - SECTOR_SIZE_IN_CELLS) as i32;

        for z in -1..=0 {
            for y in -ground_boundary..=ground_boundary {
                for x in -ground_boundary..=ground_boundary {
                    let position = IVec3::new(x as i32, y as i32, z as i32);

                    let sector_coordinate = grid::grid_position_to_sector_coordinate(position);

                    let component_sum =
                        sector_coordinate.x + sector_coordinate.y + sector_coordinate.z;

                    let kind = if component_sum % 2 == 0 {
                        block::Kind::Polished1
                    } else {
                        block::Kind::Polished2
                    };

                    World::set_block(position, kind, &world.block_info_map, &mut world.sector_vec);
                }
            }
        }
    }

    pub fn build_compass(world: &mut World) {
        let radius = 4;
        let height = 2;

        World::set_block(
            IVec3::new(0, 0, 0),
            block::Kind::Stone2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_block(
            IVec3::new(0, radius, height),
            block::Kind::NorthBlock,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_block(
            IVec3::new(-radius, 0, height),
            block::Kind::WestBlock,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_block(
            IVec3::new(0, -radius, height),
            block::Kind::SouthBlock,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_block(
            IVec3::new(radius, 0, height),
            block::Kind::EastBlock,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    pub fn build_temple(x: i32, y: i32, z: i32, nation_kind: nation::Kind, world: &mut World) {
        World::set_block(
            IVec3::new(x, y, z + 6),
            Nation::block(&nation_kind),
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 8, y - 8, z + 1),
            IVec3::new(x + 8, y + 8, z + 1),
            block::Kind::Stone1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 7, y - 7, z + 2),
            IVec3::new(x + 7, y + 7, z + 2),
            block::Kind::Stone1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 6, y - 6, z + 8),
            IVec3::new(x + 6, y + 6, z + 8),
            block::Kind::Stone1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 5, y - 5, z + 9),
            IVec3::new(x + 5, y + 5, z + 9),
            block::Kind::Stone1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 5, y - 5, z + 8),
            IVec3::new(x + 5, y + 5, z + 8),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x + 5, y + 5, z + 1),
            IVec3::new(x + 5, y + 5, z + 8),
            block::Kind::Engraved1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 5, y + 5, z + 1),
            IVec3::new(x - 5, y + 5, z + 8),
            block::Kind::Engraved1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x + 5, y - 5, z + 1),
            IVec3::new(x + 5, y - 5, z + 8),
            block::Kind::Engraved1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(x - 5, y - 5, z + 1),
            IVec3::new(x - 5, y - 5, z + 8),
            block::Kind::Engraved1,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    pub fn build_observation_deck(world: &mut World) {
        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;
        let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;

        let height = 16;
        let center = 3 * sector_size_in_cells;

        World::set_cube(
            IVec3::new(-center + 1, -center + 1, height),
            IVec3::new(-center - 1, -center - 1, 0),
            block::Kind::Polished2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(center + 1, -center + 1, height),
            IVec3::new(center - 1, -center - 1, 0),
            block::Kind::Polished2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-center + 1, center + 1, height),
            IVec3::new(-center - 1, center - 1, 0),
            block::Kind::Polished2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(center + 1, center + 1, height),
            IVec3::new(center - 1, center - 1, 0),
            block::Kind::Polished2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(
                -center - sector_radius_in_cells,
                -center - sector_radius_in_cells,
                height,
            ),
            IVec3::new(
                center + sector_radius_in_cells,
                center + sector_radius_in_cells,
                height,
            ),
            block::Kind::Polished1,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(
                -center + sector_radius_in_cells + 1,
                -center + sector_radius_in_cells + 1,
                height,
            ),
            IVec3::new(
                center - sector_radius_in_cells - 1,
                center - sector_radius_in_cells - 1,
                height,
            ),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    fn build_central_stage(world: &mut World) {
        let sector_radius_in_cells = SECTOR_RADIUS_IN_CELLS as i32;
        let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;

        World::set_box(
            IVec3::broadcast(-sector_radius_in_cells),
            IVec3::broadcast(sector_radius_in_cells),
            block::Kind::Stone2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        // East

        World::set_cube(
            IVec3::new(sector_radius_in_cells, -2, -1),
            IVec3::new(sector_radius_in_cells, 2, 4),
            block::Kind::Engraved2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(sector_radius_in_cells, -1, 0),
            IVec3::new(sector_radius_in_cells, 1, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(sector_size_in_cells - 1, -1, 0),
            IVec3::new(sector_size_in_cells + 1, 1, 12),
            block::Kind::WolfStone,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        // West

        World::set_cube(
            IVec3::new(-sector_radius_in_cells, -2, -1),
            IVec3::new(-sector_radius_in_cells, 2, 4),
            block::Kind::Engraved2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-sector_radius_in_cells, -1, 0),
            IVec3::new(-sector_radius_in_cells, 1, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-sector_size_in_cells - 1, -1, 0),
            IVec3::new(-sector_size_in_cells + 1, 1, 12),
            block::Kind::LionStone,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        // North

        World::set_cube(
            IVec3::new(-2, sector_radius_in_cells, -1),
            IVec3::new(2, sector_radius_in_cells, 4),
            block::Kind::Engraved2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, sector_radius_in_cells, 0),
            IVec3::new(1, sector_radius_in_cells, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, sector_size_in_cells - 1, 0),
            IVec3::new(1, sector_size_in_cells + 1, 12),
            block::Kind::EagleStone,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        // South

        World::set_cube(
            IVec3::new(-2, -sector_radius_in_cells, -1),
            IVec3::new(2, -sector_radius_in_cells, 4),
            block::Kind::Engraved2,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, -sector_radius_in_cells, 0),
            IVec3::new(1, -sector_radius_in_cells, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, -sector_size_in_cells - 1, 0),
            IVec3::new(1, -sector_size_in_cells + 1, 12),
            block::Kind::HorseStone,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    pub fn setup_nations(population: &mut Population) {
        let sector_size_in_cells = SECTOR_SIZE_IN_CELLS as i32;

        let wolf_nation = Nation {
            home_position: IVec3::new(sector_size_in_cells, 0, 0),
        };

        let lion_nation = Nation {
            home_position: IVec3::new(-sector_size_in_cells, 0, 0),
        };

        let eagle_nation = Nation {
            home_position: IVec3::new(0, sector_size_in_cells, 0),
        };

        let horse_nation = Nation {
            home_position: IVec3::new(0, -sector_size_in_cells, 0),
        };

        population
            .nation_map
            .insert(nation::Kind::Wolf, wolf_nation);
        population
            .nation_map
            .insert(nation::Kind::Lion, lion_nation);
        population
            .nation_map
            .insert(nation::Kind::Eagle, eagle_nation);
        population
            .nation_map
            .insert(nation::Kind::Horse, horse_nation);
    }

    pub fn setup_judge(population: &mut Population) {
        let judge = &mut population.judge;

        Judge::set_world_position(Vec3::new(0.0, 0.0, 1.0), judge);
        Judge::set_rotation(0.0, 0.0, judge);

        Sight::set_range(40.0, &mut judge.sight);
    }

    pub fn setup_agent_map(_world: &World, population: &mut Population) {
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
