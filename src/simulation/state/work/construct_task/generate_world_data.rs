use crate::{
    simulation::{
        constants::*,
        state::{
            population::{
                nation::{self, Nation},
                person::Person,
            },
            world::{
                self, block,
                grid::{self, Axis},
                object, structure,
            },
            State, World,
        },
    },
    utils::ld_math::rand_chacha_ext,
};
use std::collections::HashMap;
use ultraviolet::IVec3;

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
            (3, 100),
            (4, 100),
        ]);

        Self {
            stage_index,
            stage_cost_map,
        }
    }

    pub fn cost(generate_world_data: &Self) -> u32 {
        generate_world_data.stage_cost_map[&generate_world_data.stage_index]
    }

    pub fn step(state: &mut State, generate_world_data: &mut Self) -> bool {
        match generate_world_data.stage_index {
            0 => {
                World::reset(&mut state.world);
            }
            1 => {
                Self::layout_areas(&mut state.world);
                Self::subdivide_areas(&mut state.world);
                Self::subdivide_areas(&mut state.world);
                Self::layout_connections(&mut state.world);
            }
            2 => {
                Self::construct_areas(&mut state.world);
            }
            3 => {
                Self::construct_building_frame(&mut state.world);
                Self::construct_fascade(&mut state.world);

                Self::construct_elevator_shaft(&mut state.world);
                Self::construct_halls(&mut state.world);

                Self::construct_trade_platforms(&mut state.world);
            }
            4 => {
                Self::setup_judge(&mut state.population.person_map);
                Self::setup_nation_blocks(&state.population.nation_map, &mut state.world);

                World::set_object(
                    IVec3::new(0, 8, 0),
                    grid::Direction::East,
                    object::Kind::Stairs,
                    &mut state.world,
                );

                World::set_object(
                    IVec3::new(-1, 8, 1),
                    grid::Direction::East,
                    object::Kind::Stairs,
                    &mut state.world,
                );

                World::set_object(
                    IVec3::new(0, 10, 0),
                    grid::Direction::East,
                    object::Kind::Platform,
                    &mut state.world,
                );

                World::set_object(
                    IVec3::new(0, 12, 0),
                    grid::Direction::East,
                    object::Kind::DoorOpen,
                    &mut state.world,
                );

                World::set_object(
                    IVec3::new(0, 14, 0),
                    grid::Direction::East,
                    object::Kind::DoorClosed,
                    &mut state.world,
                );
            }
            _ => unreachable!(),
        }

        Self::next_stage(generate_world_data)
    }

    fn next_stage(generate_world_data: &mut Self) -> bool {
        generate_world_data.stage_index += 1;

        generate_world_data.stage_index >= generate_world_data.stage_cost_map.len()
    }

    fn construct_building_frame(world: &mut World) {
        let building_radius = BUILDING_RADIUS as i32;
        let building_size = 2 * BUILDING_RADIUS + 1;

        let floor_height = FLOOR_HEIGHT as i32;
        let lower_floor_count = LOWER_FLOOR_COUNT as i32;

        for floor_number in -lower_floor_count..=-1 {
            let floor_position = World::get_floor_position(floor_number);

            let cornerstone_grid_position =
                IVec3::new(-building_radius, -building_radius, floor_position);

            World::set_wireframe_box(
                cornerstone_grid_position,
                cornerstone_grid_position
                    + IVec3::new(
                        building_size as i32 - 1,
                        building_size as i32 - 1,
                        floor_height - 1,
                    ),
                block::Kind::Metal3,
                &mut world.sector_vec,
            );

            World::set_plane(
                IVec3::new(-building_radius + 1, -building_radius + 1, floor_position),
                (building_size - 2, building_size - 2),
                Axis::Z,
                block::Kind::PolishedStone2,
                world,
            );

            World::set_plane(
                IVec3::new(
                    -building_radius + 1,
                    -building_radius + 1,
                    floor_position + floor_height - 1,
                ),
                (building_size - 2, building_size - 2),
                Axis::Z,
                block::Kind::PolishedStone2,
                world,
            );
        }

        World::set_plane(
            IVec3::new(-building_radius + 1, -building_radius + 1, -1),
            (building_size - 2, building_size - 2),
            Axis::Z,
            block::Kind::PolishedStone2,
            world,
        );

        World::set_wireframe_box(
            IVec3::new(-building_radius, -building_radius, -1),
            IVec3::new(building_radius, building_radius, 0),
            block::Kind::Metal2,
            &mut world.sector_vec,
        );
    }

    fn construct_fascade(world: &mut World) {
        let building_radius = BUILDING_RADIUS as i32;

        let floor_height = FLOOR_HEIGHT as i32;
        let lower_floor_count = LOWER_FLOOR_COUNT as i32;

        for floor_number in -lower_floor_count..=-1 {
            let floor_position = World::get_floor_position(floor_number);

            let wall_height_min = floor_position + 1;
            let wall_height_max = floor_position + floor_height - 2;

            for y in -building_radius + 1..=building_radius - 1 {
                let coin_flip =
                    rand_chacha_ext::gen_range_i32(0, 1, &mut world.random_number_generator);

                let wall_height_random = rand_chacha_ext::gen_range_i32(
                    wall_height_min,
                    wall_height_max,
                    &mut world.random_number_generator,
                );

                if coin_flip == 0 {
                    World::set_cube(
                        IVec3::new(-building_radius, y, wall_height_min),
                        IVec3::new(-building_radius, y, wall_height_random),
                        block::Kind::Metal2,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(-building_radius, y, wall_height_random),
                        IVec3::new(-building_radius, y, wall_height_max),
                        block::Kind::Metal2,
                        &mut world.sector_vec,
                    );
                }

                let coin_flip =
                    rand_chacha_ext::gen_range_i32(0, 1, &mut world.random_number_generator);

                let wall_height_random = rand_chacha_ext::gen_range_i32(
                    wall_height_min,
                    wall_height_max,
                    &mut world.random_number_generator,
                );

                if coin_flip == 0 {
                    World::set_cube(
                        IVec3::new(building_radius, y, wall_height_min),
                        IVec3::new(building_radius, y, wall_height_random),
                        block::Kind::Metal2,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(building_radius, y, wall_height_random),
                        IVec3::new(building_radius, y, wall_height_max),
                        block::Kind::Metal2,
                        &mut world.sector_vec,
                    );
                }
            }

            for x in -building_radius + 1..=building_radius - 1 {
                let coin_flip =
                    rand_chacha_ext::gen_range_i32(0, 1, &mut world.random_number_generator);

                let wall_height_random = rand_chacha_ext::gen_range_i32(
                    wall_height_min,
                    wall_height_max,
                    &mut world.random_number_generator,
                );

                if coin_flip == 0 {
                    World::set_cube(
                        IVec3::new(x, -building_radius, wall_height_min),
                        IVec3::new(x, -building_radius, wall_height_random),
                        block::Kind::Metal2,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(x, -building_radius, wall_height_random),
                        IVec3::new(x, -building_radius, wall_height_max),
                        block::Kind::Metal2,
                        &mut world.sector_vec,
                    );
                }

                let coin_flip =
                    rand_chacha_ext::gen_range_i32(0, 1, &mut world.random_number_generator);

                let wall_height_random = rand_chacha_ext::gen_range_i32(
                    wall_height_min,
                    wall_height_max,
                    &mut world.random_number_generator,
                );

                if coin_flip == 0 {
                    World::set_cube(
                        IVec3::new(x, building_radius, wall_height_min),
                        IVec3::new(x, building_radius, wall_height_random),
                        block::Kind::Metal2,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(x, building_radius, wall_height_random),
                        IVec3::new(x, building_radius, wall_height_max),
                        block::Kind::Metal2,
                        &mut world.sector_vec,
                    );
                }
            }
        }
    }

    fn layout_areas(world: &mut World) {
        let external_hall_radius = 1;
        let external_hall_size = 2 * external_hall_radius + 1;

        let building_radius = BUILDING_RADIUS as i32;

        let central_elevator_shaft_radius = CENTRAL_ELEVATOR_SHAFT_RADIUS as i32;

        let floor_height = FLOOR_HEIGHT as i32;
        let lower_floor_count = LOWER_FLOOR_COUNT as i32;
        let quadrant_size =
            building_radius - external_hall_radius - central_elevator_shaft_radius - 3;

        for floor_number in -lower_floor_count..=-1 {
            let floor_position = World::get_floor_position(floor_number);

            let quadrant1_grid_position = IVec3::new(
                -(building_radius - 1) + external_hall_size,
                central_elevator_shaft_radius + 1,
                floor_position,
            );

            let quadrant2_grid_position = IVec3::new(
                central_elevator_shaft_radius + 1,
                central_elevator_shaft_radius + 1,
                floor_position,
            );

            let quadrant3_grid_position = IVec3::new(
                -(building_radius - 1) + external_hall_size,
                -(building_radius - 1) + external_hall_size,
                floor_position,
            );

            let quadrant4_grid_position = IVec3::new(
                central_elevator_shaft_radius + 1,
                -(building_radius - 1) + external_hall_size,
                floor_position,
            );

            let quadrant1_area = world::Area {
                area_id: World::get_next_area_id(world),
                grid_position: quadrant1_grid_position,
                size: IVec3::new(quadrant_size, quadrant_size, floor_height),
                connection_vec: Vec::new(),
            };

            let quadrant2_area = world::Area {
                area_id: World::get_next_area_id(world),
                grid_position: quadrant2_grid_position,
                size: IVec3::new(quadrant_size, quadrant_size, floor_height),
                connection_vec: Vec::new(),
            };

            let quadrant3_area = world::Area {
                area_id: World::get_next_area_id(world),
                grid_position: quadrant3_grid_position,
                size: IVec3::new(quadrant_size, quadrant_size, floor_height),
                connection_vec: Vec::new(),
            };

            let quadrant4_area = world::Area {
                area_id: World::get_next_area_id(world),
                grid_position: quadrant4_grid_position,
                size: IVec3::new(quadrant_size, quadrant_size, floor_height),
                connection_vec: Vec::new(),
            };

            world
                .area_map
                .insert(quadrant1_area.area_id, quadrant1_area);
            world
                .area_map
                .insert(quadrant2_area.area_id, quadrant2_area);
            world
                .area_map
                .insert(quadrant3_area.area_id, quadrant3_area);
            world
                .area_map
                .insert(quadrant4_area.area_id, quadrant4_area);
        }
    }

    fn subdivide_areas(world: &mut World) {
        let area_map = world.area_map.clone();
        let area_size_min = 3;

        for (area_id, area) in area_map {
            let axis_index =
                rand_chacha_ext::gen_range_i32(0, 1, &mut world.random_number_generator) as usize;

            let midpoint = area.size[axis_index] / 2;
            let midpoint_offset =
                rand_chacha_ext::gen_range_i32(-2, 2, &mut world.random_number_generator);
            let split_offset = midpoint + midpoint_offset;

            if split_offset <= area_size_min
                || split_offset >= area.size[axis_index] - area_size_min
            {
                continue;
            }

            let area1_grid_position = area.grid_position;

            let mut area1_size = area.size;
            area1_size[axis_index] = split_offset + 1;

            let mut area2_grid_position = area.grid_position;
            area2_grid_position[axis_index] = area1_grid_position[axis_index] + split_offset;

            let mut area2_size = area.size;
            area2_size[axis_index] = area.size[axis_index] - split_offset;

            let area1 = world::Area {
                area_id: World::get_next_area_id(world),
                grid_position: area1_grid_position,
                size: area1_size,
                connection_vec: Vec::new(),
            };

            let area2 = world::Area {
                area_id: World::get_next_area_id(world),
                grid_position: area2_grid_position,
                size: area2_size,
                connection_vec: Vec::new(),
            };

            world.area_map.remove(&area_id);

            world.area_map.insert(area1.area_id, area1);
            world.area_map.insert(area2.area_id, area2);
        }
    }

    fn layout_connections(_world: &mut World) {}

    fn construct_areas(world: &mut World) {
        for (_, area) in world.area_map.clone() {
            Self::construct_room(&area, world);
        }
    }

    fn construct_room(area: &world::Area, world: &mut World) {
        World::set_box(
            area.grid_position,
            area.grid_position + area.size - IVec3::broadcast(1),
            block::Kind::Metal1,
            &mut world.sector_vec,
        );
    }

    fn construct_halls(world: &mut World) {
        let floor_height = FLOOR_HEIGHT as i32;
        let internal_hall_radius = CENTRAL_ELEVATOR_SHAFT_RADIUS as i32 - 1;
        let building_radius = BUILDING_RADIUS as i32;

        for floor_number in 0..LOWER_FLOOR_COUNT {
            let floor_position = -((LOWER_FLOOR_COUNT - floor_number) as i32) * floor_height - 1;

            World::set_cube(
                IVec3::new(
                    -building_radius + internal_hall_radius,
                    -1,
                    floor_position + 1,
                ),
                IVec3::new(
                    building_radius - internal_hall_radius,
                    1,
                    floor_position + 4,
                ),
                block::Kind::None,
                &mut world.sector_vec,
            );

            World::set_cube(
                IVec3::new(
                    -1,
                    -building_radius + internal_hall_radius,
                    floor_position + 1,
                ),
                IVec3::new(
                    1,
                    building_radius - internal_hall_radius,
                    floor_position + 4,
                ),
                block::Kind::None,
                &mut world.sector_vec,
            );
        }

        World::set_cube(
            IVec3::new(-building_radius + 1, -1, 0),
            IVec3::new(building_radius - 1, 1, 3),
            block::Kind::None,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(-1, -building_radius + 1, 0),
            IVec3::new(1, building_radius - 1, 3),
            block::Kind::None,
            &mut world.sector_vec,
        );
    }

    fn construct_elevator_shaft(world: &mut World) {
        let shaft_radius = CENTRAL_ELEVATOR_SHAFT_RADIUS as i32;

        World::set_shell(
            IVec3::new(
                -shaft_radius,
                -shaft_radius,
                World::get_floor_position(-(LOWER_FLOOR_COUNT as i32)),
            ),
            IVec3::new(shaft_radius, shaft_radius, 6),
            block::Kind::Metal3,
            &mut world.sector_vec,
        );

        World::set_box(
            IVec3::new(
                -(shaft_radius - 2),
                -(shaft_radius - 2),
                World::get_floor_position(-(LOWER_FLOOR_COUNT as i32)) + 1,
            ),
            IVec3::new(shaft_radius - 2, shaft_radius - 2, 5),
            block::Kind::None,
            &mut world.sector_vec,
        );
    }

    fn construct_trade_platforms(world: &mut World) {
        let building_radius = BUILDING_RADIUS as i32 + 1;

        Self::construct_trade_platform(
            IVec3::new(building_radius, 0, 0),
            grid::Direction::East,
            world,
        );

        Self::construct_trade_platform(
            IVec3::new(-building_radius, 0, 0),
            grid::Direction::West,
            world,
        );

        Self::construct_trade_platform(
            IVec3::new(0, building_radius, 0),
            grid::Direction::North,
            world,
        );

        Self::construct_trade_platform(
            IVec3::new(0, -building_radius, 0),
            grid::Direction::South,
            world,
        );
    }

    fn construct_trade_platform(
        grid_position: IVec3,
        direction: grid::Direction,
        world: &mut World,
    ) {
        for (block_kind, block_grid_position) in structure::template::trade_platform(direction) {
            World::set_block(
                grid_position + block_grid_position,
                block_kind,
                &mut world.sector_vec,
            );
        }
    }

    fn setup_judge(_person_map: &mut HashMap<u64, Person>) {
        // Judge::set_world_position(Vec3::new(0.0, -8.0, 2.0), judge);
        // Judge::set_rotation(0.0, 0.0, judge);
    }

    fn setup_nation_blocks(nation_map: &HashMap<nation::Kind, Nation>, world: &mut World) {
        for (nation_kind, nation) in nation_map {
            World::set_block(
                nation.home_position - IVec3::unit_z(),
                Nation::block(nation_kind),
                &mut world.sector_vec,
            );
        }
    }
}
