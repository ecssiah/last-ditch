use std::collections::HashMap;
use crate::{
    simulation::{
        constants::*,
        state::{
            Population, State, World, population::{
                agent::Agent,
                judge::Judge,
                nation::{self, Nation},
                sight::Sight,
                spatial::Spatial,
            }, world::{block, grid::{self}, structure}
        },
    },
    utils::ld_math::rand_chacha_ext,
};
use ultraviolet::{IVec3, Vec3};

#[derive(Clone)]
pub struct GenerationData {
    pub stage_index: usize,
    pub stage_cost_map: HashMap<usize, u32>,
}

impl GenerationData {
    pub fn new() -> Self {
        let stage_index = 0;

        #[rustfmt::skip]
        let stage_cost_map = HashMap::from([
            (0, 100), 
            (1, 100), 
            (2, 100), 
            (3, 100), 
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
                Self::construct_elevator_shaft(&mut state.world);
                Self::construct_building_frame(&mut state.world);
            }
            1 => {
                Self::construct_halls(&mut state.world);
                Self::construct_fascade(&mut state.world);
                Self::construct_central_shaft(&mut state.world);
            }
            2 => {
                Self::construct_trade_platforms(&mut state.world);
            }
            3 => {
                Self::setup_nations(&mut state.population);
                Self::setup_judge(&mut state.population);
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

    fn construct_building_frame(world: &mut World) {
        let floor_height = FLOOR_HEIGHT as i32;
        let building_radius = BUILDING_RADIUS as i32;

        for floor_number in 1..=FLOOR_COUNT {
            let floor_position = -(floor_number as i32) * floor_height;

            World::set_cube(
                IVec3::new(-building_radius, -building_radius, floor_position),
                IVec3::new(building_radius, building_radius, floor_position),
                block::Kind::Polished2,
                &world.block_info_map,
                &mut world.sector_vec,
            );

            World::set_cube(
                IVec3::new(
                    -building_radius,
                    -building_radius,
                    floor_position + floor_height - 1,
                ),
                IVec3::new(
                    building_radius,
                    building_radius,
                    floor_position + floor_height - 1,
                ),
                block::Kind::Polished2,
                &world.block_info_map,
                &mut world.sector_vec,
            );

            World::set_wireframe_box(
                IVec3::new(-building_radius, -building_radius, floor_position),
                IVec3::new(
                    building_radius,
                    building_radius,
                    floor_position + floor_height,
                ),
                block::Kind::Engraved1,
                &world.block_info_map,
                &mut world.sector_vec,
            );
        }

        World::set_block(
            IVec3::new(0, 0, 0),
            block::Kind::EsayaBlock,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    fn construct_halls(world: &mut World) {
        let floor_height = FLOOR_HEIGHT as i32;
        let building_radius = BUILDING_RADIUS as i32;

        for floor_number in 1..=FLOOR_COUNT {
            let floor_position = -(floor_number as i32) * floor_height;

            World::set_cube(
                IVec3::new(-building_radius + 1, -1, floor_position),
                IVec3::new(building_radius - 1, 1, floor_position),
                block::Kind::Stone1,
                &world.block_info_map,
                &mut world.sector_vec,
            );

            World::set_cube(
                IVec3::new(-building_radius + 1, -1, floor_position + 1),
                IVec3::new(building_radius - 1, 1, floor_position + 4),
                block::Kind::None,
                &world.block_info_map,
                &mut world.sector_vec,
            );

            World::set_cube(
                IVec3::new(-1, -building_radius + 1, floor_position),
                IVec3::new(1, building_radius - 1, floor_position),
                block::Kind::Stone1,
                &world.block_info_map,
                &mut world.sector_vec,
            );

            World::set_cube(
                IVec3::new(-1, -building_radius + 1, floor_position + 1),
                IVec3::new(1, building_radius - 1, floor_position + 4),
                block::Kind::None,
                &world.block_info_map,
                &mut world.sector_vec,
            );
        }

        World::set_cube(
            IVec3::new(-building_radius + 1, -1, 0),
            IVec3::new(building_radius - 1, 1, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, -building_radius + 1, 0),
            IVec3::new(1, building_radius - 1, 3),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    fn construct_fascade(world: &mut World) {
        let floor_height = FLOOR_HEIGHT as i32;
        let building_radius = BUILDING_RADIUS as i32;

        for floor_number in 1..=FLOOR_COUNT {
            let floor_position = -(floor_number as i32) * floor_height;

            let wall_height_min = floor_position + 1;
            let wall_height_max = floor_position + floor_height - 2;

            for y in -building_radius + 1..=building_radius - 1 {
                let coin_flip = rand_chacha_ext::gen_range_i32(0, 1, &mut world.rng);

                let wall_height_random = rand_chacha_ext::gen_range_i32(
                    wall_height_min,
                    wall_height_max,
                    &mut world.rng,
                );

                if coin_flip == 0 {
                    World::set_cube(
                        IVec3::new(-building_radius, y, wall_height_min),
                        IVec3::new(-building_radius, y, wall_height_random),
                        block::Kind::Polished1,
                        &world.block_info_map,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(-building_radius, y, wall_height_random),
                        IVec3::new(-building_radius, y, wall_height_max),
                        block::Kind::Polished1,
                        &world.block_info_map,
                        &mut world.sector_vec,
                    );
                }

                let coin_flip = rand_chacha_ext::gen_range_i32(0, 1, &mut world.rng);

                let wall_height_random = rand_chacha_ext::gen_range_i32(
                    wall_height_min,
                    wall_height_max,
                    &mut world.rng,
                );

                if coin_flip == 0 {
                    World::set_cube(
                        IVec3::new(building_radius, y, wall_height_min),
                        IVec3::new(building_radius, y, wall_height_random),
                        block::Kind::Polished1,
                        &world.block_info_map,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(building_radius, y, wall_height_random),
                        IVec3::new(building_radius, y, wall_height_max),
                        block::Kind::Polished1,
                        &world.block_info_map,
                        &mut world.sector_vec,
                    );
                }
            }

            for x in -building_radius + 1..=building_radius - 1 {
                let coin_flip = rand_chacha_ext::gen_range_i32(0, 1, &mut world.rng);

                let wall_height_random = rand_chacha_ext::gen_range_i32(
                    wall_height_min,
                    wall_height_max,
                    &mut world.rng,
                );

                if coin_flip == 0 {
                    World::set_cube(
                        IVec3::new(x, -building_radius, wall_height_min),
                        IVec3::new(x, -building_radius, wall_height_random),
                        block::Kind::Polished1,
                        &world.block_info_map,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(x, -building_radius, wall_height_random),
                        IVec3::new(x, -building_radius, wall_height_max),
                        block::Kind::Polished1,
                        &world.block_info_map,
                        &mut world.sector_vec,
                    );
                }

                let coin_flip = rand_chacha_ext::gen_range_i32(0, 1, &mut world.rng);

                let wall_height_random = rand_chacha_ext::gen_range_i32(
                    wall_height_min,
                    wall_height_max,
                    &mut world.rng,
                );

                if coin_flip == 0 {
                    World::set_cube(
                        IVec3::new(x, building_radius, wall_height_min),
                        IVec3::new(x, building_radius, wall_height_random),
                        block::Kind::Polished1,
                        &world.block_info_map,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(x, building_radius, wall_height_random),
                        IVec3::new(x, building_radius, wall_height_max),
                        block::Kind::Polished1,
                        &world.block_info_map,
                        &mut world.sector_vec,
                    );
                }
            }
        }
    }

    fn construct_elevator_shaft(world: &mut World) {
        let shaft_radius = CENTRAL_ELEVATOR_SHAFT_RADIUS as i32;

        World::set_shell(
            IVec3::new(
                -shaft_radius,
                -shaft_radius,
                -(WORLD_RADIUS_IN_CELLS as i32),
            ),
            IVec3::new(shaft_radius, shaft_radius, 6),
            block::Kind::Stone2,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    fn construct_central_shaft(world: &mut World) {
        let shaft_radius = CENTRAL_ELEVATOR_SHAFT_RADIUS as i32;

        World::set_box(
            IVec3::new(
                -(shaft_radius - 2),
                -(shaft_radius - 2),
                -(WORLD_RADIUS_IN_CELLS as i32) + 1,
            ),
            IVec3::new(shaft_radius - 2, shaft_radius - 2, 5),
            block::Kind::None,
            &world.block_info_map,
            &mut world.sector_vec,
        );
    }

    fn construct_trade_platforms(world: &mut World) {
        let building_radius = BUILDING_RADIUS as i32 + 1;

        Self::construct_trade_platform(IVec3::new(building_radius, 0, 0), grid::Direction::East, world);
        Self::construct_trade_platform(IVec3::new(-building_radius, 0, 0), grid::Direction::West, world);
        Self::construct_trade_platform(IVec3::new(0, building_radius, 0), grid::Direction::North, world);
        Self::construct_trade_platform(IVec3::new(0, -building_radius, 0), grid::Direction::South, world);
    }

    fn construct_trade_platform(grid_position: IVec3, direction: grid::Direction, world: &mut World) {
        for (block_kind, block_grid_position) in structure::template::trade_platform(direction) {
            World::set_block(grid_position + block_grid_position, block_kind, &world.block_info_map, &mut world.sector_vec);
        }
    }

    fn setup_nations(population: &mut Population) {
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

        Judge::set_world_position(Vec3::new(0.0, -4.0, 1.0), judge);
        Judge::set_rotation(0.0, 0.0, judge);

        Sight::set_range(100.0, &mut judge.sight);
    }

    fn setup_agent_map(_world: &World, population: &mut Population) {
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
