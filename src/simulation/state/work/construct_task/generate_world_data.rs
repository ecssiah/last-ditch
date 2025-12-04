use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

use crate::{simulation::{constants::*, state::{State, World, population::judge::Judge, world::{block, grid, structure}}}, utils::ld_math::rand_chacha_ext};

#[derive(Clone)]
pub struct GenerateWorldData {
    pub stage_index: usize,
    pub stage_cost_map: HashMap<usize, u32>,
}

impl GenerateWorldData {
        pub fn new() -> Self {
        let stage_index = 0;

        #[rustfmt::skip]
        let stage_cost_map = HashMap::from([
            (0, 100), 
            (1, 100), 
            (2, 100), 
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
                Self::setup_judge(&mut state.population.judge);
            }
            1 => {
                Self::construct_elevator_shaft(&mut state.world);
                Self::construct_building_frame(&mut state.world);
                Self::construct_halls(&mut state.world);
            }
            2 => {
                Self::construct_fascade(&mut state.world);
                Self::construct_central_shaft(&mut state.world);
                Self::construct_trade_platforms(&mut state.world);
            }
            _ => unreachable!(),
        }

        Self::next_stage(generation_data)
    }

    fn next_stage(generation_data: &mut Self) -> bool {
        generation_data.stage_index += 1;

        generation_data.stage_index >= generation_data.stage_cost_map.len()
    }

    fn setup_judge(judge: &mut Judge) {
        Judge::set_world_position(Vec3::new(0.0, -4.0, 1.0), judge);
        Judge::set_rotation(0.0, 0.0, judge);
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

}