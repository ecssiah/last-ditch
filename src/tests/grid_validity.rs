use crate::simulation::{
    constructor,
    state::{
        self,
        world::{cell, grid::Grid, sector, World},
    },
};
use ultraviolet::IVec3;

struct CellIDValidCase {
    description: String,
    cell_id: cell::ID,
    expected_valid: bool,
}

impl CellIDValidCase {
    pub fn check(&self, world: &World) {
        let valid = Grid::cell_id_valid(self.cell_id, &world.grid);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn cell_id_valid() {
    let state_template = state::Template::Test;

    let mut world = World::new(state_template);
    constructor::world_template::construct(state_template, &mut world);

    let test_cases = vec![
        CellIDValidCase {
            description: "cell_id 0".to_string(),
            cell_id: cell::ID(0),
            expected_valid: true,
        },
        CellIDValidCase {
            description: "cell_id max".to_string(),
            cell_id: cell::ID(world.grid.sector_volume_in_cells - 1),
            expected_valid: true,
        },
        CellIDValidCase {
            description: "cell_id max + 1".to_string(),
            cell_id: cell::ID(world.grid.sector_volume_in_cells - 1 + 1),
            expected_valid: false,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct SectorIDValidCase {
    description: String,
    sector_id: sector::ID,
    expected_valid: bool,
}

impl SectorIDValidCase {
    pub fn check(&self, world: &World) {
        let valid = Grid::sector_id_valid(self.sector_id, &world.grid);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn sector_id_valid() {
    let state_template = state::Template::Test;

    let mut world = World::new(state_template);
    constructor::world_template::construct(state_template, &mut world);

    let test_cases = vec![
        SectorIDValidCase {
            description: "sector_id 0".to_string(),
            sector_id: sector::ID(0),
            expected_valid: true,
        },
        SectorIDValidCase {
            description: "sector_id max".to_string(),
            sector_id: sector::ID(world.grid.world_volume_in_sectors - 1),
            expected_valid: true,
        },
        SectorIDValidCase {
            description: "sector_id max + 1".to_string(),
            sector_id: sector::ID(world.grid.world_volume_in_sectors - 1 + 1),
            expected_valid: false,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PositionValidCase {
    description: String,
    position: IVec3,
    expected_valid: bool,
}

impl PositionValidCase {
    pub fn check(&self, world: &World) {
        let valid = Grid::position_valid(self.position, &world.grid);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn position_valid() {
    let state_template = state::Template::Empty;

    let mut world = World::new(state_template);
    constructor::world_template::construct(state_template, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        PositionValidCase {
            description: "(0, 0, 0)".to_string(),
            position: IVec3::new(0, 0, 0),
            expected_valid: true,
        },
        PositionValidCase {
            description: "(world_radius_in_cells, world_radius_in_cells, world_radius_in_cells)"
                .to_string(),
            position: IVec3::broadcast(world_radius_in_cells),
            expected_valid: true,
        },
        PositionValidCase {
            description: "(-world_radius_in_cells, -world_radius_in_cells, -world_radius_in_cells)"
                .to_string(),
            position: IVec3::broadcast(-world_radius_in_cells),
            expected_valid: true,
        },
        PositionValidCase {
            description:
                "(world_radius_in_cells + 1, world_radius_in_cells + 1, world_radius_in_cells + 1)"
                    .to_string(),
            position: IVec3::broadcast(world_radius_in_cells + 1),
            expected_valid: false,
        },
        PositionValidCase {
            description:
                "(-world_radius_in_cells - 1, -world_radius_in_cells - 1, -world_radius_in_cells - 1)"
                    .to_string(),
            position: IVec3::broadcast(-world_radius_in_cells - 1),
            expected_valid: false,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
