use crate::simulation::{
    state::world::{
        area,
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
    pub min: IVec3,
    pub max: IVec3,
    pub area_kind_map: HashMap<area::Kind, u64>,
    pub area_id_map: HashMap<u64, Area>,
}

impl Floor {
    pub fn new(floor_number: i32, area_id_generator: &mut IDGenerator) -> Self {
        let min = Tower::get_floor_min(floor_number);
        let max = Tower::get_floor_max(floor_number);

        let center_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::Center,
            style: area::Style::Elevator,
            min: Tower::get_center_min(floor_number),
            max: Tower::get_center_max(floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let center_hall_east_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::CenterHall,
            style: area::Style::None,
            min: Tower::get_center_hall_min(Direction::East, floor_number),
            max: Tower::get_center_hall_max(Direction::East, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let center_hall_west_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::CenterHall,
            style: area::Style::None,
            min: Tower::get_center_hall_min(Direction::West, floor_number),
            max: Tower::get_center_hall_max(Direction::West, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let center_hall_north_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::CenterHall,
            style: area::Style::None,
            min: Tower::get_center_hall_min(Direction::North, floor_number),
            max: Tower::get_center_hall_max(Direction::North, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let center_hall_south_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::CenterHall,
            style: area::Style::None,
            min: Tower::get_center_hall_min(Direction::South, floor_number),
            max: Tower::get_center_hall_max(Direction::South, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let outer_hall_east_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::OuterHall,
            style: area::Style::None,
            min: Tower::get_outer_hall_min(Direction::East, floor_number),
            max: Tower::get_outer_hall_max(Direction::East, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let outer_hall_west_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::OuterHall,
            style: area::Style::None,
            min: Tower::get_outer_hall_min(Direction::West, floor_number),
            max: Tower::get_outer_hall_max(Direction::West, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let outer_hall_north_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::OuterHall,
            style: area::Style::None,
            min: Tower::get_outer_hall_min(Direction::North, floor_number),
            max: Tower::get_outer_hall_max(Direction::North, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let outer_hall_south_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::OuterHall,
            style: area::Style::None,
            min: Tower::get_outer_hall_min(Direction::South, floor_number),
            max: Tower::get_outer_hall_max(Direction::South, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let corner_hall_quadrant_ne_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::CornerHall,
            style: area::Style::None,
            min: Tower::get_corner_hall_min(Quadrant::NE, floor_number),
            max: Tower::get_corner_hall_max(Quadrant::NE, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let corner_hall_quadrant_nw_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::CornerHall,
            style: area::Style::None,
            min: Tower::get_corner_hall_min(Quadrant::NW, floor_number),
            max: Tower::get_corner_hall_max(Quadrant::NW, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let corner_hall_quadrant_sw_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::CornerHall,
            style: area::Style::None,
            min: Tower::get_corner_hall_min(Quadrant::SW, floor_number),
            max: Tower::get_corner_hall_max(Quadrant::SW, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let corner_hall_quadrant_se_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::CornerHall,
            style: area::Style::None,
            min: Tower::get_corner_hall_min(Quadrant::SE, floor_number),
            max: Tower::get_corner_hall_max(Quadrant::SE, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let ne_room_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::LowerRoom,
            style: area::Style::GenericRoom,
            min: Tower::get_quadrant_min(Quadrant::NE, floor_number),
            max: Tower::get_quadrant_max(Quadrant::NE, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let nw_room_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::LowerRoom,
            style: area::Style::GenericRoom,
            min: Tower::get_quadrant_min(Quadrant::NW, floor_number),
            max: Tower::get_quadrant_max(Quadrant::NW, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let sw_room_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::LowerRoom,
            style: area::Style::GenericRoom,
            min: Tower::get_quadrant_min(Quadrant::SW, floor_number),
            max: Tower::get_quadrant_max(Quadrant::SW, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let se_room_area = Area {
            area_id: IDGenerator::allocate(area_id_generator),
            kind: area::Kind::LowerRoom,
            style: area::Style::GenericRoom,
            min: Tower::get_quadrant_min(Quadrant::SE, floor_number),
            max: Tower::get_quadrant_max(Quadrant::SE, floor_number),
            direction: Direction::East,
            connection_vec: Vec::new(),
        };

        let area_kind_map = HashMap::from([
            (area::Kind::Center, center_area.area_id),
            (area::Kind::CenterHall, center_hall_east_area.area_id),
            (area::Kind::CenterHall, center_hall_west_area.area_id),
            (area::Kind::CenterHall, center_hall_north_area.area_id),
            (area::Kind::CenterHall, center_hall_south_area.area_id),
            (area::Kind::OuterHall, outer_hall_east_area.area_id),
            (area::Kind::OuterHall, outer_hall_west_area.area_id),
            (area::Kind::OuterHall, outer_hall_north_area.area_id),
            (area::Kind::OuterHall, outer_hall_south_area.area_id),
            (area::Kind::CornerHall, corner_hall_quadrant_ne_area.area_id),
            (area::Kind::CornerHall, corner_hall_quadrant_nw_area.area_id),
            (area::Kind::CornerHall, corner_hall_quadrant_sw_area.area_id),
            (area::Kind::CornerHall, corner_hall_quadrant_se_area.area_id),
        ]);

        let area_id_map = HashMap::from([
            (center_area.area_id, center_area),
            (center_hall_east_area.area_id, center_hall_east_area),
            (center_hall_west_area.area_id, center_hall_west_area),
            (center_hall_north_area.area_id, center_hall_north_area),
            (center_hall_south_area.area_id, center_hall_south_area),
            (outer_hall_east_area.area_id, outer_hall_east_area),
            (outer_hall_west_area.area_id, outer_hall_west_area),
            (outer_hall_north_area.area_id, outer_hall_north_area),
            (outer_hall_south_area.area_id, outer_hall_south_area),
            (
                corner_hall_quadrant_ne_area.area_id,
                corner_hall_quadrant_ne_area,
            ),
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
            (ne_room_area.area_id, ne_room_area),
            (nw_room_area.area_id, nw_room_area),
            (sw_room_area.area_id, sw_room_area),
            (se_room_area.area_id, se_room_area),
        ]);

        Self {
            floor_number,
            min,
            max,
            area_kind_map,
            area_id_map,
        }
    }
}
