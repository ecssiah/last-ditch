use crate::{
    simulation::{
        constants::*,
        state::{
            population::{
                nation::{self, Nation},
                person::Person,
            },
            world::{
                block,
                grid::{self, Area, Axis, Connection, Line, Quadrant},
                object, structure, Tower,
            },
            State, World,
        },
    },
    utils::ld_math::rand_chacha_ext::{self, gen_bool, gen_range_i32},
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
                Self::construct_building_frame(&mut state.world);
                Self::construct_tower_exterior(&mut state.world);
            }
            2 => {
                Self::layout_areas(&mut state.world);

                Self::subdivide_areas(&mut state.world);
                Self::subdivide_areas(&mut state.world);
                Self::subdivide_areas(&mut state.world);

                // Self::layout_connections(&mut state.world);

                Self::construct_areas(&mut state.world);
            }
            3 => {
                // Self::construct_elevator_shaft(&mut state.world);
                // Self::construct_halls(&mut state.world);

                Self::construct_trade_platforms(&mut state.world);
            }
            4 => {
                Self::setup_judge(&mut state.population.person_map);
                Self::setup_nation_blocks(&state.population.nation_map, &mut state.world);

                World::set_block(
                    IVec3::zero(),
                    block::Kind::Ornate1,
                    &mut state.world.sector_vec,
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
        tracing::info!("Constructing Frame");

        for floor_number in -(TOWER_FLOOR_COUNT as i32)..0 {
            let floor_min = Tower::get_floor_min(floor_number);
            let floor_max = Tower::get_floor_max(floor_number);

            tracing::info!(
                "Floor: {:?} Min: {:?} Max: {:?}",
                floor_number,
                floor_min,
                floor_max
            );

            World::set_cube(
                floor_min,
                IVec3::new(floor_max.x, floor_max.y, floor_min.z),
                block::Kind::Panel2,
                &mut world.sector_vec,
            );

            World::set_cube(
                IVec3::new(floor_min.x, floor_min.y, floor_max.z),
                floor_max,
                block::Kind::Panel2,
                &mut world.sector_vec,
            );

            World::set_wireframe_box(
                floor_min,
                floor_max,
                block::Kind::Caution,
                &mut world.sector_vec,
            );
        }

        let roof_min = Tower::get_floor_min(0);
        let roof_max = Tower::get_floor_max(0);

        World::set_cube(
            roof_min,
            IVec3::new(roof_max.x, roof_max.y, roof_min.z),
            block::Kind::Stone3,
            &mut world.sector_vec,
        );
    }

    fn construct_tower_exterior(world: &mut World) {
        tracing::info!("Constructing Tower Exterior");

        let tower_radius = TOWER_RADIUS as i32;

        for floor_number in -(TOWER_FLOOR_COUNT as i32)..0 {
            let floor_min = Tower::get_floor_min(floor_number);
            let floor_max = Tower::get_floor_max(floor_number);

            tracing::info!(
                "Floor: {:?} Min: {:?} Max: {:?}",
                floor_number,
                floor_min,
                floor_max
            );

            for y in -tower_radius + 1..=tower_radius - 1 {
                let floor_z_random =
                    gen_range_i32(floor_min.z + 1, floor_max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_cube(
                        IVec3::new(-tower_radius, y, floor_min.z + 1),
                        IVec3::new(-tower_radius, y, floor_z_random),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(-tower_radius, y, floor_z_random),
                        IVec3::new(-tower_radius, y, floor_max.z - 1),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                }

                let floor_z_random =
                    gen_range_i32(floor_min.z + 1, floor_max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_cube(
                        IVec3::new(tower_radius, y, floor_min.z + 1),
                        IVec3::new(tower_radius, y, floor_z_random),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(tower_radius, y, floor_z_random),
                        IVec3::new(tower_radius, y, floor_max.z - 1),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                }
            }

            for x in -tower_radius + 1..=tower_radius - 1 {
                let floor_z_random =
                    gen_range_i32(floor_min.z + 1, floor_max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_cube(
                        IVec3::new(x, -tower_radius, floor_min.z + 1),
                        IVec3::new(x, -tower_radius, floor_z_random),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(x, -tower_radius, floor_z_random),
                        IVec3::new(x, -tower_radius, floor_max.z - 1),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                }

                let floor_z_random =
                    gen_range_i32(floor_min.z + 1, floor_max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_cube(
                        IVec3::new(x, tower_radius, floor_min.z + 1),
                        IVec3::new(x, tower_radius, floor_z_random),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(x, tower_radius, floor_z_random),
                        IVec3::new(x, tower_radius, floor_max.z - 1),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                }
            }
        }
    }

    fn layout_areas(world: &mut World) {
        tracing::info!("Laying out tower areas");

        for floor_number in -(TOWER_FLOOR_COUNT as i32)..=-1 {
            tracing::info!("Floor: {:?}", floor_number);

            let ne_area = Area {
                area_id: World::get_next_area_id(world),
                min: Tower::get_quadrant_min(Quadrant::NE, floor_number),
                max: Tower::get_quadrant_max(Quadrant::NE, floor_number),
                connection_vec: Vec::new(),
            };

            let nw_area = Area {
                area_id: World::get_next_area_id(world),
                min: Tower::get_quadrant_min(Quadrant::NW, floor_number),
                max: Tower::get_quadrant_max(Quadrant::NW, floor_number),
                connection_vec: Vec::new(),
            };

            let sw_area = Area {
                area_id: World::get_next_area_id(world),
                min: Tower::get_quadrant_min(Quadrant::SW, floor_number),
                max: Tower::get_quadrant_max(Quadrant::SW, floor_number),
                connection_vec: Vec::new(),
            };

            let se_area = Area {
                area_id: World::get_next_area_id(world),
                min: Tower::get_quadrant_min(Quadrant::SE, floor_number),
                max: Tower::get_quadrant_max(Quadrant::SE, floor_number),
                connection_vec: Vec::new(),
            };

            world.area_map.insert(ne_area.area_id, ne_area);
            world.area_map.insert(nw_area.area_id, nw_area);
            world.area_map.insert(sw_area.area_id, sw_area);
            world.area_map.insert(se_area.area_id, se_area);
        }
    }

    fn subdivide_areas(world: &mut World) {
        let mut area_map_subdivided = HashMap::new();

        let area_size_min = 5;

        for (_, area) in world.area_map.clone() {
            if gen_bool(&mut world.rng) {
                let split_point = gen_range_i32(area.min.x + 2, area.max.x - 2, &mut world.rng);

                if split_point - area.min.x + 1 >= area_size_min
                    && area.max.x - split_point + 1 >= area_size_min
                {
                    let area1 = Area {
                        area_id: World::get_next_area_id(world),
                        min: area.min,
                        max: IVec3::new(split_point, area.max.y, area.max.z),
                        connection_vec: Vec::new(),
                    };

                    let area2 = Area {
                        area_id: World::get_next_area_id(world),
                        min: IVec3::new(split_point, area.min.y, area.min.z),
                        max: area.max,
                        connection_vec: Vec::new(),
                    };

                    area_map_subdivided.insert(area1.area_id, area1);
                    area_map_subdivided.insert(area2.area_id, area2);
                } else {
                    area_map_subdivided.insert(area.area_id, area);
                }
            } else {
                let split_point = gen_range_i32(area.min.y + 2, area.max.y - 2, &mut world.rng);

                if split_point - area.min.y + 1 >= area_size_min
                    && area.max.y - split_point + 1 >= area_size_min
                {
                    let area1 = Area {
                        area_id: World::get_next_area_id(world),
                        min: area.min,
                        max: IVec3::new(area.max.x, split_point, area.max.z),
                        connection_vec: Vec::new(),
                    };

                    let area2 = Area {
                        area_id: World::get_next_area_id(world),
                        min: IVec3::new(area.min.x, split_point, area.min.z),
                        max: area.max,
                        connection_vec: Vec::new(),
                    };

                    area_map_subdivided.insert(area1.area_id, area1);
                    area_map_subdivided.insert(area2.area_id, area2);
                } else {
                    area_map_subdivided.insert(area.area_id, area);
                }
            }
        }

        world.area_map = area_map_subdivided;
    }

    fn layout_connections(world: &mut World) {
        let mut connection_candidate_vec = Vec::new();

        for (area1_id, area1) in &world.area_map {
            for (area2_id, area2) in &world.area_map {
                if area1_id >= area2_id {
                    continue;
                }

                if let Some(shared_line) = Area::find_shared_line(area1, area2) {
                    let entrance_vec = vec![Line::midpoint(&shared_line)];
                    let cost = rand_chacha_ext::gen_f32(&mut world.rng);

                    let connection_candidate = Connection {
                        area_id1: *area1_id,
                        area_id2: *area2_id,
                        entrance_vec,
                        line: shared_line,
                        cost,
                    };

                    connection_candidate_vec.push(connection_candidate);
                }
            }
        }

        for connection in connection_candidate_vec {
            if let Some(area1) = world.area_map.get_mut(&connection.area_id1) {
                area1.connection_vec.push(connection.clone());
            }

            if let Some(area2) = world.area_map.get_mut(&connection.area_id2) {
                area2.connection_vec.push(connection.clone());
            }
        }
    }

    fn construct_areas(world: &mut World) {
        for (_, area) in world.area_map.clone() {
            Self::construct_room(&area, world);

            for connection in &area.connection_vec {
                let direction = match connection.line.axis {
                    Axis::X => grid::Direction::North,
                    Axis::Y => grid::Direction::East,
                    Axis::Z => grid::Direction::Up,
                };

                World::set_cube(
                    connection.entrance_vec[0] + 1 * IVec3::unit_z(),
                    connection.entrance_vec[0] + 2 * IVec3::unit_z(),
                    block::Kind::None,
                    &mut world.sector_vec,
                );

                World::set_object(
                    connection.entrance_vec[0] + 1 * IVec3::unit_z(),
                    direction,
                    object::Kind::DoorOpen,
                    world,
                );
            }
        }
    }

    fn construct_room(area: &Area, world: &mut World) {
        // World::set_cube(
        //     area.min,
        //     IVec3::new(area.max.x, area.max.y, area.min.z),
        //     block::Kind::CarvedStone1,
        //     &mut world.sector_vec,
        // );

        World::set_wireframe_box(
            area.min,
            area.max,
            block::Kind::Metal1,
            &mut world.sector_vec,
        );
    }

    fn construct_elevator_shaft(world: &mut World) {
        // let shaft_radius = CENTRAL_ELEVATOR_SHAFT_RADIUS as i32;

        // World::set_shell(
        //     IVec3::new(
        //         -shaft_radius,
        //         -shaft_radius,
        //         World::get_tower_floor_height(-(LOWER_FLOOR_COUNT as i32)),
        //     ),
        //     IVec3::new(shaft_radius, shaft_radius, 6),
        //     block::Kind::Metal3,
        //     &mut world.sector_vec,
        // );

        // World::set_box(
        //     IVec3::new(
        //         -(shaft_radius - 2),
        //         -(shaft_radius - 2),
        //         World::get_tower_floor_height(-(LOWER_FLOOR_COUNT as i32)) + 1,
        //     ),
        //     IVec3::new(shaft_radius - 2, shaft_radius - 2, 5),
        //     block::Kind::None,
        //     &mut world.sector_vec,
        // );
    }

    fn construct_trade_platforms(world: &mut World) {
        let platform_radius = TOWER_RADIUS as i32 + 1;

        Self::construct_trade_platform(
            IVec3::new(platform_radius, 0, 0),
            grid::Direction::East,
            world,
        );

        Self::construct_trade_platform(
            IVec3::new(-platform_radius, 0, 0),
            grid::Direction::West,
            world,
        );

        Self::construct_trade_platform(
            IVec3::new(0, platform_radius, 0),
            grid::Direction::North,
            world,
        );

        Self::construct_trade_platform(
            IVec3::new(0, -platform_radius, 0),
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
                nation.home_position,
                Nation::block(nation_kind),
                &mut world.sector_vec,
            );
        }
    }
}
