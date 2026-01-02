use crate::simulation::{
    state::world::{
        area::{self, AreaKind},
        grid::{Direction, Quadrant},
        tower::Tower,
        Area,
    },
    utils::IDGenerator,
};
use std::collections::HashMap;
use ultraviolet::IVec3;

pub struct Floor {
    pub floor_number: i32,
    pub grid_position: IVec3,
    pub area_kind_map: HashMap<AreaKind, u64>,
    pub area_id_map: HashMap<u64, Area>,
}

impl Floor {
    pub fn new(floor_number: i32, area_id_generator: &mut IDGenerator) -> Self {
        let center_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::Center,
            floor_number: floor_number,
            style: area::Style::Elevator,
            grid_position: Tower::get_center_grid_position(floor_number),
            size: Tower::get_center_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let center_hall_north_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::CenterHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_center_hall_grid_position(Direction::North, floor_number),
            size: Tower::get_center_hall_size(Direction::North),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let center_hall_west_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::CenterHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_center_hall_grid_position(Direction::West, floor_number),
            size: Tower::get_center_hall_size(Direction::West),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let center_hall_south_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::CenterHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_center_hall_grid_position(Direction::South, floor_number),
            size: Tower::get_center_hall_size(Direction::South),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let center_hall_east_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::CenterHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_center_hall_grid_position(Direction::East, floor_number),
            size: Tower::get_center_hall_size(Direction::East),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let outer_hall_north_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::OuterHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_outer_hall_grid_position(Direction::North, floor_number),
            size: Tower::get_outer_hall_size(Direction::North),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let outer_hall_west_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::OuterHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_outer_hall_grid_position(Direction::West, floor_number),
            size: Tower::get_outer_hall_size(Direction::West),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let outer_hall_south_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::OuterHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_outer_hall_grid_position(Direction::South, floor_number),
            size: Tower::get_outer_hall_size(Direction::South),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let outer_hall_east_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::OuterHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_outer_hall_grid_position(Direction::East, floor_number),
            size: Tower::get_outer_hall_size(Direction::East),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let corner_hall_quadrant_ne_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::CornerHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_corner_hall_grid_position(Quadrant::NE, floor_number),
            size: Tower::get_corner_hall_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let corner_hall_quadrant_nw_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::CornerHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_corner_hall_grid_position(Quadrant::NW, floor_number),
            size: Tower::get_corner_hall_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let corner_hall_quadrant_sw_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::CornerHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_corner_hall_grid_position(Quadrant::SW, floor_number),
            size: Tower::get_corner_hall_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let corner_hall_quadrant_se_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::CornerHall,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_corner_hall_grid_position(Quadrant::SE, floor_number),
            size: Tower::get_corner_hall_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let ne_room_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::LowerRoom,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_quadrant_grid_position(Quadrant::NE, floor_number),
            size: Tower::get_quadrant_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let nw_room_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::LowerRoom,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_quadrant_grid_position(Quadrant::NW, floor_number),
            size: Tower::get_quadrant_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let sw_room_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::LowerRoom,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_quadrant_grid_position(Quadrant::SW, floor_number),
            size: Tower::get_quadrant_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let se_room_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            area_kind: AreaKind::LowerRoom,
            floor_number: floor_number,
            style: area::Style::Wireframe,
            grid_position: Tower::get_quadrant_grid_position(Quadrant::SE, floor_number),
            size: Tower::get_quadrant_size(),
            direction: Direction::North,
            connection_vec: Vec::new(),
        };

        let area_kind_map = HashMap::from([
            (AreaKind::Center, center_area.area_id),
            (AreaKind::CenterHall, center_hall_north_area.area_id),
            (AreaKind::CenterHall, center_hall_west_area.area_id),
            (AreaKind::CenterHall, center_hall_south_area.area_id),
            (AreaKind::CenterHall, center_hall_east_area.area_id),
            (AreaKind::OuterHall, outer_hall_north_area.area_id),
            (AreaKind::OuterHall, outer_hall_west_area.area_id),
            (AreaKind::OuterHall, outer_hall_south_area.area_id),
            (AreaKind::OuterHall, outer_hall_east_area.area_id),
            (AreaKind::CornerHall, corner_hall_quadrant_nw_area.area_id),
            (AreaKind::CornerHall, corner_hall_quadrant_sw_area.area_id),
            (AreaKind::CornerHall, corner_hall_quadrant_se_area.area_id),
            (AreaKind::CornerHall, corner_hall_quadrant_ne_area.area_id),
        ]);

        let area_id_map = HashMap::from([
            (center_area.area_id, center_area),
            (center_hall_north_area.area_id, center_hall_north_area),
            (center_hall_west_area.area_id, center_hall_west_area),
            (center_hall_south_area.area_id, center_hall_south_area),
            (center_hall_east_area.area_id, center_hall_east_area),
            (outer_hall_north_area.area_id, outer_hall_north_area),
            (outer_hall_west_area.area_id, outer_hall_west_area),
            (outer_hall_south_area.area_id, outer_hall_south_area),
            (outer_hall_east_area.area_id, outer_hall_east_area),
            (
                corner_hall_quadrant_nw_area.area_id,
                corner_hall_quadrant_nw_area,
            ),
            (
                corner_hall_quadrant_sw_area.area_id,
                corner_hall_quadrant_sw_area,
            ),
            (
                corner_hall_quadrant_se_area.area_id,
                corner_hall_quadrant_se_area,
            ),
            (
                corner_hall_quadrant_ne_area.area_id,
                corner_hall_quadrant_ne_area,
            ),
            (nw_room_area.area_id, nw_room_area),
            (sw_room_area.area_id, sw_room_area),
            (se_room_area.area_id, se_room_area),
            (ne_room_area.area_id, ne_room_area),
        ]);

        Self {
            floor_number,
            grid_position: Tower::get_floor_grid_position(floor_number),
            area_kind_map,
            area_id_map,
        }
    }
}
