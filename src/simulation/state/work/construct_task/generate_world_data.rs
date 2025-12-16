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
                        ElevatorCapTemplate, ElevatorTemplate, GenericRoomTemplate, Template,
                        TradingPlatformTemplate, WireframeTemplate,
                    },
                    Connection,
                },
                block,
                grid::{self, Direction, Line},
                tower::{self, Tower},
                Area,
            },
            State, World,
        },
        utils::IDGenerator,
    },
    utils::ldmath::rand_chacha_ext::{self, gen_bool, gen_range_i32},
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

                Self::construct_floor_map(&mut state.world);
            }
            1 => {
                Self::construct_building_frame(&mut state.world);
                Self::construct_tower_exterior(&mut state.world);
                Self::construct_roof(&mut state.world);
            }
            2 => {
                Self::subdivide_room_areas(&mut state.world);
                Self::subdivide_room_areas(&mut state.world);
                Self::subdivide_room_areas(&mut state.world);

                Self::layout_connections(&mut state.world);
            }
            3 => {
                Self::construct_areas(&mut state.world);
            }
            4 => {
                Self::setup_judge(&mut state.population.person_map);
                Self::setup_nation_blocks(&state.population.nation_map, &mut state.world);
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

        let floor_size = Tower::get_floor_size();

        let (base_min, base_max) = grid::get_bounds(
            Tower::get_floor_grid_position(-tower_floor_count) + IVec3::new(0, 0, -1),
            floor_size,
        );

        World::set_cube(
            base_min,
            IVec3::new(base_max.x, base_max.y, base_min.z),
            block::Kind::Stone3,
            &mut world.sector_vec,
        );

        for floor_number in -tower_floor_count..0 {
            let floor = world
                .tower
                .floor_map
                .get_mut(&floor_number)
                .expect("Floors should exist!");

            let (floor_min, floor_max) = grid::get_bounds(floor.grid_position, floor_size);

            tracing::info!("Constructing Frame");
            tracing::info!(
                "Floor: {:?} Position: {:?} Size: {:?}",
                floor.floor_number,
                floor.grid_position,
                floor_size,
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

            World::set_wireframe(
                floor_min,
                floor_max,
                block::Kind::Caution,
                &mut world.sector_vec,
            );
        }
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

            let floor_size = Tower::get_floor_size();

            let (floor_min, floor_max) = grid::get_bounds(floor.grid_position, floor_size);

            tracing::info!("Constructing Tower Exterior");
            tracing::info!(
                "Floor: {:?} Position: {:?}",
                floor.floor_number,
                floor.grid_position,
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

    fn construct_roof(world: &mut World) {
        let floor_size = Tower::get_floor_size();

        let (roof_min, roof_max) = grid::get_bounds(Tower::get_floor_grid_position(0), floor_size);

        World::set_cube(
            roof_min,
            IVec3::new(roof_max.x, roof_max.y, roof_min.z),
            block::Kind::Stone3,
            &mut world.sector_vec,
        );

        let roof_elevator_area = Area {
            area_id: IDGenerator::allocate(&mut world.area_id_generator),
            floor_number: 0,
            kind: area::Kind::UpperRoom,
            style: area::Style::ElevatorCap,
            grid_position: Tower::get_center_grid_position(0),
            size: Tower::get_center_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        world
            .tower
            .area_map
            .insert(roof_elevator_area.area_id, roof_elevator_area);

        let tower_radius = TOWER_RADIUS as i32;

        let trading_platform_radius_x = TRADING_PLATFORM_RADIUS_X as i32;
        let trading_platform_radius_y = TRADING_PLATFORM_RADIUS_Y as i32;

        let trading_platform_size = IVec3::new(
            2 * trading_platform_radius_x + 1,
            2 * trading_platform_radius_y + 1,
            1,
        );

        let mut lion_trading_area = Area::new(IDGenerator::allocate(&mut world.area_id_generator));
        lion_trading_area.style = area::Style::TradingPlatform;
        lion_trading_area.direction = Direction::North;
        lion_trading_area.grid_position =
            IVec3::new(-trading_platform_radius_x, tower_radius + 1, 0);
        lion_trading_area.size = trading_platform_size;

        let mut eagle_trading_area = Area::new(IDGenerator::allocate(&mut world.area_id_generator));
        eagle_trading_area.style = area::Style::TradingPlatform;
        eagle_trading_area.direction = Direction::West;
        eagle_trading_area.grid_position =
            IVec3::new(-tower_radius - 1, -trading_platform_radius_x, 0);
        eagle_trading_area.size = trading_platform_size;

        let mut horse_trading_area = Area::new(IDGenerator::allocate(&mut world.area_id_generator));
        horse_trading_area.style = area::Style::TradingPlatform;
        horse_trading_area.direction = Direction::South;
        horse_trading_area.grid_position =
            IVec3::new(trading_platform_radius_x, -tower_radius - 1, 0);
        horse_trading_area.size = trading_platform_size;

        let mut wolf_trading_area = Area::new(IDGenerator::allocate(&mut world.area_id_generator));
        wolf_trading_area.style = area::Style::TradingPlatform;
        wolf_trading_area.direction = Direction::East;
        wolf_trading_area.grid_position =
            IVec3::new(tower_radius + 1, trading_platform_radius_x, 0);
        wolf_trading_area.size = trading_platform_size;

        world
            .tower
            .area_map
            .insert(lion_trading_area.area_id, lion_trading_area);

        world
            .tower
            .area_map
            .insert(eagle_trading_area.area_id, eagle_trading_area);

        world
            .tower
            .area_map
            .insert(horse_trading_area.area_id, horse_trading_area);

        world
            .tower
            .area_map
            .insert(wolf_trading_area.area_id, wolf_trading_area);
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
                "Floor: {:?} Position: {:?}",
                floor.floor_number,
                floor.grid_position,
            );

            let lower_room_id_vec: Vec<u64> = floor
                .area_id_map
                .iter()
                .filter(|(_, area)| area.kind == area::Kind::LowerRoom)
                .map(|(area_id, _)| *area_id)
                .collect();

            let mut new_room_area_map: HashMap<u64, Area> = HashMap::new();

            for area_id in lower_room_id_vec {
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
        let tower_floor_count = TOWER_FLOOR_COUNT as i32;

        for floor_number in -tower_floor_count..0 {
            let floor = world
                .tower
                .floor_map
                .get_mut(&floor_number)
                .expect("Floors should exist!");

            tracing::info!("Connecting rooms");
            tracing::info!(
                "Floor: {:?} Position: {:?}",
                floor.floor_number,
                floor.grid_position,
            );

            let mut cancidate_connection_vec = Vec::new();

            for (area1_id, area1) in &floor.area_id_map {
                for (area2_id, area2) in &floor.area_id_map {
                    if area1_id >= area2_id {
                        continue;
                    }

                    if let Some(contact) = Area::find_contact(area1, area2) {
                        if let Some(line) =
                            Area::find_ground_line(floor.grid_position.z + 1, 3, contact)
                        {
                            let entrance_vec = vec![Line::midpoint(&line)];
                            let cost = rand_chacha_ext::gen_f32(&mut world.rng);

                            let connection_candidate = Connection {
                                area_id1: *area1_id,
                                area_id2: *area2_id,
                                entrance_vec,
                                line,
                                cost,
                            };

                            cancidate_connection_vec.push(connection_candidate.clone());
                        }
                    }
                }
            }

            for connection in cancidate_connection_vec {
                if let Some(area1) = floor.area_id_map.get_mut(&connection.area_id1) {
                    area1.connection_vec.push(connection.clone());
                }

                if let Some(area2) = floor.area_id_map.get_mut(&connection.area_id2) {
                    area2.connection_vec.push(connection.clone());
                }
            }
        }
    }

    fn construct_areas(world: &mut World) {
        let tower_floor_count = TOWER_FLOOR_COUNT as i32;

        for floor_number in -tower_floor_count..0 {
            let floor = world
                .tower
                .floor_map
                .get_mut(&floor_number)
                .expect("Floors should exist!");

            let area_id_map = floor.area_id_map.clone();

            let (center_area_id_map, other_area_id_map): (Vec<_>, Vec<_>) = area_id_map
                .iter()
                .partition(|(_, area)| area.kind == area::Kind::Center);

            for (_, area) in other_area_id_map {
                Self::construct_room(area, world);
            }

            for (_, area) in center_area_id_map {
                Self::construct_room(area, world);
            }
        }

        for (_, area) in world.tower.area_map.clone() {
            Self::construct_room(&area, world);
        }
    }

    fn construct_room(area: &Area, world: &mut World) {
        match &area.style {
            area::Style::None => (),
            area::Style::Wireframe => WireframeTemplate::construct(area, world),
            area::Style::GenericRoom => GenericRoomTemplate::construct(area, world),
            area::Style::Elevator => ElevatorTemplate::construct(area, world),
            area::Style::ElevatorCap => ElevatorCapTemplate::construct(area, world),
            area::Style::TradingPlatform => TradingPlatformTemplate::construct(area, world),
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
