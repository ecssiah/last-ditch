use crate::simulation::{
    self, constructor,
    consts::*,
    state::world::{grid::Grid, World},
};
use ultraviolet::IVec3;

struct GetClearanceCase {
    description: String,
    sector_coordinates: IVec3,
    cell_coordinates: IVec3,
    expected_clearance: u32,
}

impl GetClearanceCase {
    pub fn check(&self, world: &World) {
        let sector_position =
            Grid::sector_coordinates_to_position(&world.grid, self.sector_coordinates);

        assert_ne!(
            sector_position,
            IVec3::new(i32::MAX, i32::MAX, i32::MAX),
            "{:?}",
            self.description
        );

        let position = sector_position + self.cell_coordinates;
        let clearance = World::get_clearance(position, &world.grid, &world.sector_vec);

        assert_eq!(clearance, self.expected_clearance, "{:?}", self.description);
    }
}

#[test]
fn get_clearance() {
    let simulation_kind = simulation::Kind::Test;

    let mut world = World::new(simulation_kind);
    constructor::world::run(simulation_kind, &mut world);

    let test_cases = vec![
        GetClearanceCase {
            description: String::from("clearance 0"),
            sector_coordinates: IVec3::new(0, 0, 1),
            cell_coordinates: IVec3::new(-4, -4, 4),
            expected_clearance: 0,
        },
        GetClearanceCase {
            description: String::from("clearance 1"),
            sector_coordinates: IVec3::new(0, 0, 1),
            cell_coordinates: IVec3::new(-3, -4, 4),
            expected_clearance: 1,
        },
        GetClearanceCase {
            description: String::from("clearance 2"),
            sector_coordinates: IVec3::new(0, 0, 1),
            cell_coordinates: IVec3::new(-2, -4, 4),
            expected_clearance: 2,
        },
        GetClearanceCase {
            description: String::from("clearance 3"),
            sector_coordinates: IVec3::new(0, 0, 1),
            cell_coordinates: IVec3::new(-1, -4, 4),
            expected_clearance: 3,
        },
        GetClearanceCase {
            description: String::from("clearance 4"),
            sector_coordinates: IVec3::new(0, 0, 1),
            cell_coordinates: IVec3::new(0, -4, 4),
            expected_clearance: 4,
        },
        GetClearanceCase {
            description: String::from("clearance 5"),
            sector_coordinates: IVec3::new(0, 0, 1),
            cell_coordinates: IVec3::new(1, -4, 4),
            expected_clearance: 5,
        },
        GetClearanceCase {
            description: String::from("clearance 6"),
            sector_coordinates: IVec3::new(0, 0, 1),
            cell_coordinates: IVec3::new(2, -4, 4),
            expected_clearance: MAXIMUM_CLEARANCE,
        },
        GetClearanceCase {
            description: String::from("clearance 7"),
            sector_coordinates: IVec3::new(0, 0, 1),
            cell_coordinates: IVec3::new(3, -4, 4),
            expected_clearance: MAXIMUM_CLEARANCE,
        },
        GetClearanceCase {
            description: String::from("clearance 8"),
            sector_coordinates: IVec3::new(0, 0, 1),
            cell_coordinates: IVec3::new(4, -4, 4),
            expected_clearance: MAXIMUM_CLEARANCE,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
