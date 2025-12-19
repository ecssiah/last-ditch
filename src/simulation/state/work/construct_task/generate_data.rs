use crate::{
    simulation::{
        constants::*,
        state::{
            population::{
                identity,
                nation::{self},
                person::Person,
            },
            world::{
                area::{
                    self,
                    template::{
                        ElevatorCapTemplate, ElevatorTemplate, GenericRoomTemplate, Template,
                        TempleTemplate, TradingPlatformTemplate, WireframeTemplate,
                    },
                    Connection,
                },
                block,
                grid::{self, Direction, Line},
                tower::{self, Tower},
                Area,
            },
            Population, State, World,
        },
        utils::IDGenerator,
    },
    utils::ldmath::rand_chacha_ext::{self, gen_bool, gen_range_i32},
};
use std::collections::HashMap;
use ultraviolet::{IVec3, Vec3};

#[derive(Clone)]
pub struct GenerateData {
    pub stage_index: usize,
    pub stage_cost_map: HashMap<usize, u32>,
}

impl GenerateData {
    pub fn new() -> Self {
        let stage_index = 0;
        let stage_cost_map = HashMap::from([(0, 100), (1, 100), (2, 100), (3, 100), (4, 100)]);

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
                Population::reset(&mut state.population);
                World::reset(&mut state.world);
            }
            1 => {
                Self::generate_population(&state.world, &mut state.population);
            }
            2 => {
                Self::construct_floor_map(&mut state.world);
                Self::construct_building_frame(&mut state.world);
                Self::construct_tower_exterior(&mut state.world);
                Self::construct_roof(&mut state.world);
                Self::construct_nation_temples(&state.population, &mut state.world);
            }
            3 => {
                Self::subdivide_room_areas(&mut state.world);
                Self::subdivide_room_areas(&mut state.world);
                Self::subdivide_room_areas(&mut state.world);

                Self::layout_connections(&mut state.world);
            }
            4 => {
                Self::construct_areas(&mut state.world);
            }
            _ => unreachable!(),
        }

        Self::next_stage(generate_world_data)
    }

    fn next_stage(generate_world_data: &mut Self) -> bool {
        generate_world_data.stage_index += 1;

        generate_world_data.stage_index >= generate_world_data.stage_cost_map.len()
    }

    fn generate_population(_world: &World, population: &mut Population) {
        tracing::info!("Generating Judge 1");

        let mut judge = Person::new(ID_JUDGE_1);

        Person::set_world_position(Vec3::new(0.0, -8.0, 2.0), &mut judge);
        Person::set_rotation(0.0, 0.0, &mut judge);

        Person::set_size(
            Vec3::new(
                JUDGE_DEFAULT_SIZE_X,
                JUDGE_DEFAULT_SIZE_Y,
                JUDGE_DEFAULT_SIZE_Z,
            ),
            &mut judge,
        );

        population.person_map.insert(judge.person_id, judge);

        tracing::info!("Generating Nations");

        let nation_map = population.nation_map.clone();

        for (_, nation) in nation_map {
            for _ in 1..=NATION_INITIAL_POPULATION {
                let mut person = Person::new(IDGenerator::allocate(&mut population.id_generator));

                let sex = match gen_range_i32(0, 1, &mut population.rng) {
                    0 => identity::Sex::Male,
                    _ => identity::Sex::Female,
                };

                person.identity.sex = sex;

                let temple_radius_x = TEMPLE_RADIUS_X as i32;
                let temple_radius_y = TEMPLE_RADIUS_Y as i32;

                let home_offset = IVec3::new(
                    gen_range_i32(
                        -temple_radius_x + 2,
                        temple_radius_x - 2,
                        &mut population.rng,
                    ),
                    gen_range_i32(
                        -temple_radius_y + 2,
                        temple_radius_y - 2,
                        &mut population.rng,
                    ),
                    2,
                );

                let grid_position = nation.home_grid_position + home_offset;
                let world_position = grid::grid_position_to_world_position(grid_position);

                Person::set_world_position(world_position, &mut person);

                Person::set_size(
                    Vec3::new(
                        PERSON_DEFAULT_SIZE_X,
                        PERSON_DEFAULT_SIZE_Y,
                        rand_chacha_ext::gen_range_f32(
                            PERSON_DEFAULT_SIZE_Z - 0.2,
                            PERSON_DEFAULT_SIZE_Z + 0.2,
                            &mut population.rng,
                        ),
                    ),
                    &mut person,
                );

                let direction = match nation.nation_kind {
                    nation::Kind::Lion => Direction::South,
                    nation::Kind::Eagle => Direction::East,
                    nation::Kind::Horse => Direction::North,
                    nation::Kind::Wolf => Direction::West,
                };

                let rotation_xy = Direction::to_rotation(direction);

                Person::set_rotation(rotation_xy, 0.0, &mut person);

                person.kinematic.speed = PERSON_DEFAULT_SPEED;
                person.kinematic.jump_speed = PERSON_DEFAULT_JUMP_SPEED;

                population.person_map.insert(person.person_id, person);
            }
        }
    }

    fn construct_floor_map(world: &mut World) {
        tracing::info!("Constructing Floors");

        let tower_floor_count = TOWER_FLOOR_COUNT as i32;

        for floor_number in -tower_floor_count..0 {
            tracing::info!("Constructing Floor {:?}", floor_number);

            let floor = tower::Floor::new(floor_number, &mut world.area_id_generator);

            world.tower.floor_map.insert(floor_number, floor);
        }
    }

    fn construct_building_frame(world: &mut World) {
        let tower_floor_count = TOWER_FLOOR_COUNT as i32;

        let floor_size = Tower::get_floor_size();

        let base_ibox = grid::get_grid_ibox(
            Tower::get_floor_grid_position(-tower_floor_count) + IVec3::new(0, 0, -1),
            floor_size,
        );

        World::set_block_cube(
            base_ibox.min,
            IVec3::new(base_ibox.max.x, base_ibox.max.y, base_ibox.min.z),
            block::Kind::Stone3,
            world,
        );

        for floor_number in -tower_floor_count..0 {
            let floor = world
                .tower
                .floor_map
                .get_mut(&floor_number)
                .expect("Floors should exist!");

            let floor_ibox = grid::get_grid_ibox(floor.grid_position, floor_size);

            tracing::info!("Constructing Frame, Floor: {:?}", floor.floor_number);

            World::set_block_cube(
                floor_ibox.min,
                IVec3::new(floor_ibox.max.x, floor_ibox.max.y, floor_ibox.min.z),
                block::Kind::Panel2,
                world,
            );

            World::set_block_cube(
                IVec3::new(floor_ibox.min.x, floor_ibox.min.y, floor_ibox.max.z),
                floor_ibox.max,
                block::Kind::Panel2,
                world,
            );

            World::set_block_wireframe(floor_ibox.min, floor_ibox.max, block::Kind::Caution, world);
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

            let floor_ibox = grid::get_grid_ibox(floor.grid_position, floor_size);

            tracing::info!("Constructing Exterior, Floor: {:?}", floor.floor_number);

            for y in -tower_radius + 1..=tower_radius - 1 {
                let floor_z_random =
                    gen_range_i32(floor_ibox.min.z + 1, floor_ibox.max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_block_cube(
                        IVec3::new(-tower_radius, y, floor_ibox.min.z + 1),
                        IVec3::new(-tower_radius, y, floor_z_random),
                        block::Kind::Metal3,
                        world,
                    );
                } else {
                    World::set_block_cube(
                        IVec3::new(-tower_radius, y, floor_z_random),
                        IVec3::new(-tower_radius, y, floor_ibox.max.z - 1),
                        block::Kind::Metal3,
                        world,
                    );
                }

                let floor_z_random =
                    gen_range_i32(floor_ibox.min.z + 1, floor_ibox.max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_block_cube(
                        IVec3::new(tower_radius, y, floor_ibox.min.z + 1),
                        IVec3::new(tower_radius, y, floor_z_random),
                        block::Kind::Metal3,
                        world,
                    );
                } else {
                    World::set_block_cube(
                        IVec3::new(tower_radius, y, floor_z_random),
                        IVec3::new(tower_radius, y, floor_ibox.max.z - 1),
                        block::Kind::Metal3,
                        world,
                    );
                }
            }

            for x in -tower_radius + 1..=tower_radius - 1 {
                let floor_z_random =
                    gen_range_i32(floor_ibox.min.z + 1, floor_ibox.max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_block_cube(
                        IVec3::new(x, -tower_radius, floor_ibox.min.z + 1),
                        IVec3::new(x, -tower_radius, floor_z_random),
                        block::Kind::Metal3,
                        world,
                    );
                } else {
                    World::set_block_cube(
                        IVec3::new(x, -tower_radius, floor_z_random),
                        IVec3::new(x, -tower_radius, floor_ibox.max.z - 1),
                        block::Kind::Metal3,
                        world,
                    );
                }

                let floor_z_random =
                    gen_range_i32(floor_ibox.min.z + 1, floor_ibox.max.z - 1, &mut world.rng);

                if gen_bool(&mut world.rng) {
                    World::set_block_cube(
                        IVec3::new(x, tower_radius, floor_ibox.min.z + 1),
                        IVec3::new(x, tower_radius, floor_z_random),
                        block::Kind::Metal3,
                        world,
                    );
                } else {
                    World::set_block_cube(
                        IVec3::new(x, tower_radius, floor_z_random),
                        IVec3::new(x, tower_radius, floor_ibox.max.z - 1),
                        block::Kind::Metal3,
                        world,
                    );
                }
            }
        }
    }

    fn construct_roof(world: &mut World) {
        tracing::info!("Constructing Roof");

        let floor_size = Tower::get_floor_size();

        let roof_ibox = grid::get_grid_ibox(Tower::get_floor_grid_position(0), floor_size);

        World::set_block_cube(
            roof_ibox.min,
            IVec3::new(roof_ibox.max.x, roof_ibox.max.y, roof_ibox.min.z),
            block::Kind::Stone3,
            world,
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

            tracing::info!("Subdividing Rooms, Floor {:?}", floor.floor_number);

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

            tracing::info!("Connecting Rooms, Floor: {:?}", floor.floor_number);

            let mut candidate_connection_vec = Vec::new();

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

                            candidate_connection_vec.push(connection_candidate.clone());
                        }
                    }
                }
            }

            for connection in candidate_connection_vec {
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

            tracing::info!("Constructing Areas, Floor {:?}", floor_number);

            let area_id_map = floor.area_id_map.clone();

            let (center_area_id_map, other_area_id_map): (Vec<_>, Vec<_>) = area_id_map
                .iter()
                .partition(|(_, area)| area.kind == area::Kind::Center);

            for (_, area) in other_area_id_map {
                Self::construct_area(area, world);
            }

            for (_, area) in center_area_id_map {
                Self::construct_area(area, world);
            }
        }

        for (_, area) in world.tower.area_map.clone() {
            Self::construct_area(&area, world);
        }
    }

    fn construct_nation_temples(population: &Population, world: &mut World) {
        tracing::info!("Constructing Nation Temples");

        for (nation_kind, nation) in &population.nation_map {
            tracing::info!("Constructing {:?} Temple", nation.nation_kind);

            let mut temple_area = Area::new(IDGenerator::allocate(&mut world.area_id_generator));

            let temple_radius_x = TEMPLE_RADIUS_X as i32;
            let temple_radius_y = TEMPLE_RADIUS_Y as i32;
            let temple_size_z = TEMPLE_SIZE_Z as i32;

            let temple_grid_position = nation.home_grid_position
                + match nation_kind {
                    nation::Kind::Lion => IVec3::new(temple_radius_x, temple_radius_y, 0),
                    nation::Kind::Eagle => IVec3::new(-temple_radius_y, temple_radius_x, 0),
                    nation::Kind::Horse => IVec3::new(-temple_radius_x, -temple_radius_y, 0),
                    nation::Kind::Wolf => IVec3::new(temple_radius_y, -temple_radius_x, 0),
                };

            let temple_direction = match nation_kind {
                nation::Kind::Lion => Direction::South,
                nation::Kind::Eagle => Direction::East,
                nation::Kind::Horse => Direction::North,
                nation::Kind::Wolf => Direction::West,
            };

            temple_area.grid_position = temple_grid_position;
            temple_area.size = IVec3::new(
                2 * temple_radius_x + 1,
                2 * temple_radius_y + 1,
                temple_size_z,
            );
            temple_area.direction = temple_direction;
            temple_area.style = area::Style::Temple {
                nation_kind: *nation_kind,
            };

            Self::construct_area(&temple_area, world);

            world
                .tower
                .area_map
                .insert(temple_area.area_id, temple_area);
        }
    }

    fn construct_area(area: &Area, world: &mut World) {
        match &area.style {
            area::Style::None => (),
            area::Style::Wireframe => WireframeTemplate::construct(area, world),
            area::Style::GenericRoom => GenericRoomTemplate::construct(area, world),
            area::Style::Elevator => ElevatorTemplate::construct(area, world),
            area::Style::ElevatorCap => ElevatorCapTemplate::construct(area, world),
            area::Style::TradingPlatform => TradingPlatformTemplate::construct(area, world),
            area::Style::Temple { nation_kind: _ } => TempleTemplate::construct(area, world),
        }
    }
}
