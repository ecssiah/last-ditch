use crate::{
    simulation::{
        constants::*,
        state::{
            population::{
                nation::{self, Nation},
                person::Person,
            },
            world::{
                area::{
                    self,
                    template::{
                        ElevatorTemplate, GenericRoomTemplate, Template, WireframeTemplate,
                    },
                },
                block,
                grid::{self, Axis},
                object, structure,
                tower::{self, Tower},
                Area,
            },
            State, World,
        },
    },
    utils::ld_math::rand_chacha_ext::{gen_bool, gen_range_i32},
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
                Self::construct_floor_map(&mut state.world);
                Self::construct_building_frame(&mut state.world);
                Self::construct_tower_exterior(&mut state.world);
            }
            2 => {
                Self::subdivide_room_areas(&mut state.world);
                Self::subdivide_room_areas(&mut state.world);
                Self::subdivide_room_areas(&mut state.world);

                // Self::layout_connections(&mut state.world);

                Self::construct_areas(&mut state.world);
            }
            3 => {
                Self::construct_elevator_shaft(&mut state.world);
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

    fn construct_floor_map(world: &mut World) {
        let tower_floor_count = TOWER_FLOOR_COUNT as i32;

        for floor_number in -tower_floor_count..0 {
            let floor = tower::Floor::new(floor_number, &mut world.area_id_generator);

            world.tower.floor_map.insert(floor_number, floor);
        }
    }

    fn construct_building_frame(world: &mut World) {
        let tower_floor_count = TOWER_FLOOR_COUNT as i32;

        for floor_number in -tower_floor_count..0 {
            let floor = world
                .tower
                .floor_map
                .get_mut(&floor_number)
                .expect("Floors should exist!");

            tracing::info!("Constructing Frame");
            tracing::info!(
                "Floor: {:?} Min: {:?} Max: {:?}",
                floor.floor_number,
                floor.min,
                floor.max,
            );

            World::set_cube(
                floor.min,
                IVec3::new(floor.max.x, floor.max.y, floor.min.z),
                block::Kind::Panel2,
                &mut world.sector_vec,
            );

            World::set_cube(
                IVec3::new(floor.min.x, floor.min.y, floor.max.z),
                floor.max,
                block::Kind::Panel2,
                &mut world.sector_vec,
            );

            World::set_wireframe_box(
                floor.min,
                floor.max,
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
        let tower_radius = TOWER_RADIUS as i32;
        let tower_floor_count = TOWER_FLOOR_COUNT as i32;

        for floor_number in -tower_floor_count..0 {
            let floor = world
                .tower
                .floor_map
                .get_mut(&floor_number)
                .expect("Floors should exist!");

            tracing::info!("Constructing Tower Exterior");
            tracing::info!(
                "Floor: {:?} Min: {:?} Max: {:?}",
                floor.floor_number,
                floor.min,
                floor.max,
            );

            for y in -tower_radius + 1..=tower_radius - 1 {
                let floor_z_random =
                    gen_range_i32(floor.min.z + 1, floor.max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_cube(
                        IVec3::new(-tower_radius, y, floor.min.z + 1),
                        IVec3::new(-tower_radius, y, floor_z_random),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(-tower_radius, y, floor_z_random),
                        IVec3::new(-tower_radius, y, floor.max.z - 1),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                }

                let floor_z_random =
                    gen_range_i32(floor.min.z + 1, floor.max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_cube(
                        IVec3::new(tower_radius, y, floor.min.z + 1),
                        IVec3::new(tower_radius, y, floor_z_random),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(tower_radius, y, floor_z_random),
                        IVec3::new(tower_radius, y, floor.max.z - 1),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                }
            }

            for x in -tower_radius + 1..=tower_radius - 1 {
                let floor_z_random =
                    gen_range_i32(floor.min.z + 1, floor.max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_cube(
                        IVec3::new(x, -tower_radius, floor.min.z + 1),
                        IVec3::new(x, -tower_radius, floor_z_random),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(x, -tower_radius, floor_z_random),
                        IVec3::new(x, -tower_radius, floor.max.z - 1),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                }

                let floor_z_random =
                    gen_range_i32(floor.min.z + 1, floor.max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_cube(
                        IVec3::new(x, tower_radius, floor.min.z + 1),
                        IVec3::new(x, tower_radius, floor_z_random),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                } else {
                    World::set_cube(
                        IVec3::new(x, tower_radius, floor_z_random),
                        IVec3::new(x, tower_radius, floor.max.z - 1),
                        block::Kind::Metal3,
                        &mut world.sector_vec,
                    );
                }
            }
        }
    }

    fn subdivide_room_areas(world: &mut World) {
        let tower_floor_count = TOWER_FLOOR_COUNT as i32;

        for floor_number in -tower_floor_count..0 {
            let floor = world
                .tower
                .floor_map
                .get_mut(&floor_number)
                .expect("Floors should exist!");

            tracing::info!("Subdividing rooms");
            tracing::info!(
                "Floor: {:?} Min: {:?} Max: {:?}",
                floor.floor_number,
                floor.min,
                floor.max,
            );

            let room_id_vec: Vec<u64> = floor
                .area_id_map
                .iter()
                .filter(|(_, area)| area.kind == area::Kind::Room)
                .map(|(area_id, _)| *area_id)
                .collect();

            let mut new_room_area_map: HashMap<u64, Area> = HashMap::new();

            for area_id in room_id_vec {
                let area = floor.area_id_map.remove(&area_id).unwrap();

                if let Some((area1, area2)) =
                    World::subdivide_area(&area, &mut world.area_id_generator, &mut world.rng)
                {
                    new_room_area_map.insert(area1.area_id, area1);
                    new_room_area_map.insert(area2.area_id, area2);
                } else {
                    new_room_area_map.insert(area.area_id, area);
                }
            }

            floor.area_id_map.extend(new_room_area_map);
        }
    }

    fn layout_connections(world: &mut World) {
        // let mut connection_candidate_vec = Vec::new();

        // for (area1_id, area1) in &world.room_area_map {
        //     for (area2_id, area2) in &world.room_area_map {
        //         if area1_id >= area2_id {
        //             continue;
        //         }

        //         if let Some(shared_line) = Area::find_shared_line(area1, area2) {
        //             let entrance_vec = vec![Line::midpoint(&shared_line)];
        //             let cost = rand_chacha_ext::gen_f32(&mut world.rng);

        //             let connection_candidate = Connection {
        //                 area_id1: *area1_id,
        //                 area_id2: *area2_id,
        //                 entrance_vec,
        //                 line: shared_line,
        //                 cost,
        //             };

        //             connection_candidate_vec.push(connection_candidate);
        //         }
        //     }
        // }

        // for connection in connection_candidate_vec {
        //     if let Some(area1) = world.room_area_map.get_mut(&connection.area_id1) {
        //         area1.connection_vec.push(connection.clone());
        //     }

        //     if let Some(area2) = world.room_area_map.get_mut(&connection.area_id2) {
        //         area2.connection_vec.push(connection.clone());
        //     }
        // }
    }

    fn construct_areas(world: &mut World) {
        let tower_floor_count = TOWER_FLOOR_COUNT as i32;

        for floor_number in -tower_floor_count..0 {
            let floor = world
                .tower
                .floor_map
                .get_mut(&floor_number)
                .expect("Floors should exist!");

            for (_, area) in floor.area_id_map.clone() {
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
    }

    fn construct_room(area: &Area, world: &mut World) {
        match &area.style {
            area::Style::None => (),
            area::Style::Wireframe => WireframeTemplate::construct(area, world),
            area::Style::GenericRoom => GenericRoomTemplate::construct(area, world),
            area::Style::Elevator => ElevatorTemplate::construct(area, world),
        }
    }

    fn construct_elevator_shaft(world: &mut World) {
        let tower_floor_count = TOWER_FLOOR_COUNT as i32;
        let tower_central_hall_radius = TOWER_CENTER_HALL_RADIUS as i32;

        World::set_shell(
            IVec3::new(
                -tower_central_hall_radius,
                -tower_central_hall_radius,
                Tower::get_floor_z_min(-tower_floor_count),
            ),
            IVec3::new(tower_central_hall_radius, tower_central_hall_radius, 6),
            block::Kind::Metal3,
            &mut world.sector_vec,
        );

        World::set_box(
            IVec3::new(
                -(tower_central_hall_radius - 2),
                -(tower_central_hall_radius - 2),
                Tower::get_floor_z_min(-tower_floor_count + 1),
            ),
            IVec3::new(
                tower_central_hall_radius - 2,
                tower_central_hall_radius - 2,
                5,
            ),
            block::Kind::None,
            &mut world.sector_vec,
        );
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
