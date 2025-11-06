use crate::simulation::{
    self, constructor, state::world::{World, cell, grid::Grid, sector}
};
use glam::{IVec3, Vec3};

struct CellIDToCellCoordinatesCase {
    description: String,
    cell_id: cell::ID,
    expected_cell_coordinates: IVec3,
}

impl CellIDToCellCoordinatesCase {
    pub fn check(&self, world: &World) {
        let cell_coordinates = Grid::cell_id_to_cell_coordinates(&world.grid, self.cell_id);

        assert_ne!(cell_coordinates, IVec3::MAX, "{:?}", self.description);
        assert_eq!(
            cell_coordinates, self.expected_cell_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn cell_id_to_cell_coordinates() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let test_cases = vec![
        CellIDToCellCoordinatesCase {
            description: "cell id min".to_string(),
            cell_id: cell::ID(0),
            expected_cell_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
        CellIDToCellCoordinatesCase {
            description: "cell id at origin".to_string(),
            cell_id: cell::ID((world.grid.sector_volume_in_cells - 1) / 2),
            expected_cell_coordinates: IVec3::splat(0),
        },
        CellIDToCellCoordinatesCase {
            description: "cell id max".to_string(),
            cell_id: cell::ID(world.grid.sector_volume_in_cells - 1),
            expected_cell_coordinates: IVec3::splat(sector_radius_in_cells),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct CellCoordinatesToCellIDCase {
    description: String,
    cell_coordinates: IVec3,
    expected_cell_id: cell::ID,
}

impl CellCoordinatesToCellIDCase {
    pub fn check(&self, world: &World) {
        let cell_id = Grid::cell_coordinates_to_cell_id(&world.grid, self.cell_coordinates);

        assert_ne!(cell_id, cell::ID::MAX, "{:?}", self.description);
        assert_eq!(cell_id, self.expected_cell_id, "{:?}", self.description);
    }
}

#[test]
fn cell_coordinates_to_cell_id() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let test_cases = vec![
        CellCoordinatesToCellIDCase {
            description: "cell coordinates min".to_string(),
            cell_coordinates: IVec3::splat(-sector_radius_in_cells),
            expected_cell_id: cell::ID(0),
        },
        CellCoordinatesToCellIDCase {
            description: "cell coordinates origin".to_string(),
            cell_coordinates: IVec3::splat(0),
            expected_cell_id: cell::ID((world.grid.sector_volume_in_cells - 1) / 2),
        },
        CellCoordinatesToCellIDCase {
            description: "cell coordinates max".to_string(),
            cell_coordinates: IVec3::splat(sector_radius_in_cells),
            expected_cell_id: cell::ID(world.grid.sector_volume_in_cells - 1),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct SectorIDToSectorCoordinates {
    description: String,
    sector_id: sector::ID,
    expected_sector_coordinates: IVec3,
}

impl SectorIDToSectorCoordinates {
    pub fn check(&self, world: &World) {
        let sector_coordinates = Grid::sector_id_to_sector_coordinates(&world.grid, self.sector_id);

        assert_ne!(sector_coordinates, IVec3::MAX, "{:?}", self.description);
        assert_eq!(
            sector_coordinates, self.expected_sector_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn sector_id_to_sector_coordinates() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_sectors = world.grid.world_radius_in_sectors as i32;

    let test_cases = vec![
        SectorIDToSectorCoordinates {
            description: "sector id min".to_string(),
            sector_id: sector::ID(0),
            expected_sector_coordinates: IVec3::splat(-world_radius_in_sectors),
        },
        SectorIDToSectorCoordinates {
            description: "sector id at origin".to_string(),
            sector_id: sector::ID((world.grid.world_volume_in_sectors - 1) / 2),
            expected_sector_coordinates: IVec3::splat(0),
        },
        SectorIDToSectorCoordinates {
            description: "sector id max".to_string(),
            sector_id: sector::ID(world.grid.world_volume_in_sectors - 1),
            expected_sector_coordinates: IVec3::splat(world_radius_in_sectors),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct SectorCoordinatesToSectorIDCase {
    description: String,
    sector_coordinates: IVec3,
    expected_sector_id: sector::ID,
}

impl SectorCoordinatesToSectorIDCase {
    pub fn check(&self, world: &World) {
        let sector_id = Grid::sector_coordinates_to_sector_id(&world.grid, self.sector_coordinates);

        assert_ne!(sector_id, sector::ID::MAX, "{:?}", self.description);
        assert_eq!(sector_id, self.expected_sector_id, "{:?}", self.description);
    }
}

#[test]
fn sector_coordinates_to_sector_id() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_sectors = world.grid.world_radius_in_sectors as i32;

    let test_cases = vec![
        SectorCoordinatesToSectorIDCase {
            description: "sector coordinates min".to_string(),
            sector_coordinates: IVec3::splat(-world_radius_in_sectors),
            expected_sector_id: sector::ID(0),
        },
        SectorCoordinatesToSectorIDCase {
            description: "sector coordinates origin".to_string(),
            sector_coordinates: IVec3::splat(0),
            expected_sector_id: sector::ID((world.grid.world_volume_in_sectors - 1) / 2),
        },
        SectorCoordinatesToSectorIDCase {
            description: "sector coordinates max".to_string(),
            sector_coordinates: IVec3::splat(world_radius_in_sectors),
            expected_sector_id: sector::ID(world.grid.world_volume_in_sectors - 1),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct SectorCoordinatesToPositionCase {
    description: String,
    sector_coordinates: IVec3,
    expected_position: IVec3,
}

impl SectorCoordinatesToPositionCase {
    pub fn check(&self, world: &World) {
        let position = Grid::sector_coordinates_to_position(&world.grid, self.sector_coordinates);

        assert_ne!(position, IVec3::MAX, "{:?}", self.description);
        assert_eq!(position, self.expected_position, "{:?}", self.description);
    }
}

#[test]
fn sector_coordinates_to_position() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;
    let world_radius_in_sectors = world.grid.world_radius_in_sectors as i32;
    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        SectorCoordinatesToPositionCase {
            description: "sector min".to_string(),
            sector_coordinates: IVec3::splat(-world_radius_in_sectors),
            expected_position: IVec3::splat(-world_radius_in_cells + sector_radius_in_cells),
        },
        SectorCoordinatesToPositionCase {
            description: "sector origin".to_string(),
            sector_coordinates: IVec3::splat(0),
            expected_position: IVec3::splat(0),
        },
        SectorCoordinatesToPositionCase {
            description: "sector max".to_string(),
            sector_coordinates: IVec3::splat(world_radius_in_sectors),
            expected_position: IVec3::splat(world_radius_in_cells - sector_radius_in_cells),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct SectorIDToPositionCase {
    description: String,
    sector_id: sector::ID,
    expected_position: IVec3,
}

impl SectorIDToPositionCase {
    pub fn check(&self, world: &World) {
        let position = Grid::sector_id_to_position(&world.grid, self.sector_id);

        assert_ne!(position, IVec3::MAX, "{:?}", self.description);
        assert_eq!(position, self.expected_position, "{:?}", self.description);
    }
}

#[test]
fn sector_id_to_position() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;
    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let test_cases = vec![
        SectorIDToPositionCase {
            description: "sector_id min".to_string(),
            sector_id: sector::ID(0),
            expected_position: IVec3::splat(-world_radius_in_cells + sector_radius_in_cells),
        },
        SectorIDToPositionCase {
            description: "sector_id origin".to_string(),
            sector_id: sector::ID((world.grid.world_volume_in_sectors - 1) / 2),
            expected_position: IVec3::splat(0),
        },
        SectorIDToPositionCase {
            description: "sector_id max".to_string(),
            sector_id: sector::ID(world.grid.world_volume_in_sectors - 1),
            expected_position: IVec3::splat(world_radius_in_cells - sector_radius_in_cells),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PositionToSectorCoordinatesCase {
    description: String,
    position: IVec3,
    expected_sector_coordinates: IVec3,
}

impl PositionToSectorCoordinatesCase {
    pub fn check(&self, world: &World) {
        let sector_coordinates = Grid::position_to_sector_coordinates(&world.grid, self.position);

        assert_ne!(sector_coordinates, IVec3::MAX, "{:?}", self.description);
        assert_eq!(
            sector_coordinates, self.expected_sector_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn position_to_sector_coordinates() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;
    let world_radius_in_sectors = world.grid.world_radius_in_sectors as i32;
    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        PositionToSectorCoordinatesCase {
            description: "sector min".to_string(),
            position: IVec3::splat(-world_radius_in_cells + sector_radius_in_cells),
            expected_sector_coordinates: IVec3::splat(-world_radius_in_sectors),
        },
        PositionToSectorCoordinatesCase {
            description: "sector origin".to_string(),
            position: IVec3::splat(0),
            expected_sector_coordinates: IVec3::splat(0),
        },
        PositionToSectorCoordinatesCase {
            description: "sector max".to_string(),
            position: IVec3::splat(world_radius_in_cells - sector_radius_in_cells),
            expected_sector_coordinates: IVec3::splat(world_radius_in_sectors),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PositionToCellCoordinatesCase {
    description: String,
    position: IVec3,
    expected_cell_coordinates: IVec3,
}

impl PositionToCellCoordinatesCase {
    pub fn check(&self, world: &World) {
        let cell_coordinates = Grid::position_to_cell_coordinates(&world.grid, self.position);

        assert_ne!(cell_coordinates, IVec3::MAX, "{:?}", self.description);
        assert_eq!(
            cell_coordinates, self.expected_cell_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn position_to_cell_coordinates() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;
    let sector_size_in_cells = world.grid.sector_size_in_cells as i32;
    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        PositionToCellCoordinatesCase {
            description: "origin sector min".to_string(),
            position: IVec3::splat(-sector_radius_in_cells),
            expected_cell_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
        PositionToCellCoordinatesCase {
            description: "origin sector origin".to_string(),
            position: IVec3::splat(0),
            expected_cell_coordinates: IVec3::splat(0),
        },
        PositionToCellCoordinatesCase {
            description: "origin sector max".to_string(),
            position: IVec3::splat(sector_radius_in_cells),
            expected_cell_coordinates: IVec3::splat(sector_radius_in_cells),
        },
        PositionToCellCoordinatesCase {
            description: "world min sector min".to_string(),
            position: IVec3::splat(-world_radius_in_cells),
            expected_cell_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
        PositionToCellCoordinatesCase {
            description: "world min sector origin".to_string(),
            position: IVec3::splat(-world_radius_in_cells + sector_radius_in_cells),
            expected_cell_coordinates: IVec3::splat(0),
        },
        PositionToCellCoordinatesCase {
            description: "world min sector max".to_string(),
            position: IVec3::splat(-world_radius_in_cells + sector_size_in_cells - 1),
            expected_cell_coordinates: IVec3::splat(sector_radius_in_cells),
        },
        PositionToCellCoordinatesCase {
            description: "world max sector min".to_string(),
            position: IVec3::splat(world_radius_in_cells - sector_size_in_cells + 1),
            expected_cell_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
        PositionToCellCoordinatesCase {
            description: "world max sector origin".to_string(),
            position: IVec3::splat(world_radius_in_cells - sector_radius_in_cells),
            expected_cell_coordinates: IVec3::splat(0),
        },
        PositionToCellCoordinatesCase {
            description: "world max sector max".to_string(),
            position: IVec3::splat(world_radius_in_cells),
            expected_cell_coordinates: IVec3::splat(sector_radius_in_cells),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PositionToSectorIDCase {
    description: String,
    position: IVec3,
    expected_sector_id: sector::ID,
}

impl PositionToSectorIDCase {
    pub fn check(&self, world: &World) {
        let sector_id = Grid::position_to_sector_id(&world.grid, self.position);

        assert_ne!(sector_id, sector::ID::MAX, "{:?}", self.description);
        assert_eq!(sector_id, self.expected_sector_id, "{:?}", self.description);
    }
}

#[test]
fn position_to_sector_id() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        PositionToSectorIDCase {
            description: "position min".to_string(),
            position: IVec3::splat(-world_radius_in_cells),
            expected_sector_id: sector::ID(0),
        },
        PositionToSectorIDCase {
            description: "position origin".to_string(),
            position: IVec3::splat(0),
            expected_sector_id: sector::ID((world.grid.world_volume_in_sectors - 1) / 2),
        },
        PositionToSectorIDCase {
            description: "position max".to_string(),
            position: IVec3::splat(world_radius_in_cells),
            expected_sector_id: sector::ID(world.grid.world_volume_in_sectors - 1),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PositionToCellIDCase {
    description: String,
    position: IVec3,
    expected_cell_id: cell::ID,
}

impl PositionToCellIDCase {
    pub fn check(&self, world: &World) {
        let cell_id = Grid::position_to_cell_id(&world.grid, self.position);

        assert_ne!(cell_id, cell::ID::MAX, "{:?}", self.description);
        assert_eq!(cell_id, self.expected_cell_id, "{:?}", self.description);
    }
}

#[test]
fn position_to_cell_id() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let test_cases = vec![
        PositionToCellIDCase {
            description: "position min".to_string(),
            position: IVec3::splat(-sector_radius_in_cells),
            expected_cell_id: cell::ID(0),
        },
        PositionToCellIDCase {
            description: "position origin".to_string(),
            position: IVec3::splat(0),
            expected_cell_id: cell::ID((world.grid.sector_volume_in_cells - 1) / 2),
        },
        PositionToCellIDCase {
            description: "position max".to_string(),
            position: IVec3::splat(sector_radius_in_cells),
            expected_cell_id: cell::ID(world.grid.sector_volume_in_cells - 1),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct IDsToPositionCase {
    description: String,
    sector_id: sector::ID,
    cell_id: cell::ID,
    expected_position: IVec3,
}

impl IDsToPositionCase {
    pub fn check(&self, world: &World) {
        let position = Grid::ids_to_position(&world.grid, self.sector_id, self.cell_id);

        assert_ne!(position, IVec3::MAX, "{:?}", self.description);
        assert_eq!(position, self.expected_position, "{:?}", self.description);
    }
}

#[test]
fn ids_to_position() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        IDsToPositionCase {
            description: "ids at min".to_string(),
            sector_id: sector::ID(0),
            cell_id: cell::ID(0),
            expected_position: IVec3::splat(-world_radius_in_cells),
        },
        IDsToPositionCase {
            description: "ids at origin".to_string(),
            sector_id: sector::ID((world.grid.world_volume_in_sectors - 1) / 2),
            cell_id: cell::ID((world.grid.sector_volume_in_cells - 1) / 2),
            expected_position: IVec3::splat(0),
        },
        IDsToPositionCase {
            description: "ids at max".to_string(),
            sector_id: sector::ID(world.grid.world_volume_in_sectors - 1),
            cell_id: cell::ID(world.grid.sector_volume_in_cells - 1),
            expected_position: IVec3::splat(world_radius_in_cells),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PositionToIDsCase {
    description: String,
    position: IVec3,
    expected_ids: (sector::ID, cell::ID),
}

impl PositionToIDsCase {
    pub fn check(&self, world: &World) {
        let (sector_id, cell_id) = Grid::position_to_ids(&world.grid, self.position);

        assert_ne!(sector_id, sector::ID::MAX, "{:?}", self.description);
        assert_ne!(cell_id, cell::ID::MAX, "{:?}", self.description);
        assert_eq!(
            (sector_id, cell_id),
            self.expected_ids,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn position_to_ids() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        PositionToIDsCase {
            description: "position at min".to_string(),
            position: IVec3::splat(-world_radius_in_cells),
            expected_ids: (sector::ID(0), cell::ID(0)),
        },
        PositionToIDsCase {
            description: "position at origin".to_string(),
            position: IVec3::splat(0),
            expected_ids: (
                sector::ID((world.grid.world_volume_in_sectors - 1) / 2),
                cell::ID((world.grid.sector_volume_in_cells - 1) / 2),
            ),
        },
        PositionToIDsCase {
            description: "position at max".to_string(),
            position: IVec3::splat(world_radius_in_cells),
            expected_ids: (
                sector::ID(world.grid.world_volume_in_sectors - 1),
                cell::ID(world.grid.sector_volume_in_cells - 1),
            ),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct WorldToPositionCase {
    description: String,
    world_position: Vec3,
    expected_position: IVec3,
}

impl WorldToPositionCase {
    pub fn check(&self, world: &World) {
        let position = Grid::world_to_position(&world.grid, self.world_position);

        assert_eq!(position, self.expected_position, "{:?}", self.description,);
    }
}

#[test]
fn world_to_position() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as f32;

    let test_cases = vec![
        WorldToPositionCase {
            description: "world min".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells),
            expected_position: IVec3::splat(-world_radius_in_cells as i32),
        },
        WorldToPositionCase {
            description: "world min - 1.0".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells - 1.0),
            expected_position: IVec3::MAX,
        },
        WorldToPositionCase {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_position: IVec3::splat(0),
        },
        WorldToPositionCase {
            description: "world max".to_string(),
            world_position: Vec3::splat(world_radius_in_cells),
            expected_position: IVec3::splat(world_radius_in_cells as i32),
        },
        WorldToPositionCase {
            description: "world max + 1.0".to_string(),
            world_position: Vec3::splat(world_radius_in_cells + 1.0),
            expected_position: IVec3::MAX,
        },
        WorldToPositionCase {
            description: "standard position".to_string(),
            world_position: Vec3::new(0.0, -3.5, 0.0),
            expected_position: IVec3::new(0, -3, 0),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct WorldToSectorIDCase {
    description: String,
    world_position: Vec3,
    expected_sector_id: sector::ID,
}

impl WorldToSectorIDCase {
    pub fn check(&self, world: &World) {
        let sector_id = Grid::world_to_sector_id(&world.grid, self.world_position);

        assert_eq!(sector_id, self.expected_sector_id, "{:?}", self.description);
    }
}

#[test]
fn world_to_sector_id() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as f32;

    let test_cases = vec![
        WorldToSectorIDCase {
            description: "world min".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells),
            expected_sector_id: sector::ID(0),
        },
        WorldToSectorIDCase {
            description: "world min - 1.0".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells - 1.0),
            expected_sector_id: sector::ID::MAX,
        },
        WorldToSectorIDCase {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_sector_id: sector::ID((world.grid.world_volume_in_sectors - 1) / 2),
        },
        WorldToSectorIDCase {
            description: "world max".to_string(),
            world_position: Vec3::splat(world_radius_in_cells),
            expected_sector_id: sector::ID(world.grid.world_volume_in_sectors - 1),
        },
        WorldToSectorIDCase {
            description: "world max + 1.0".to_string(),
            world_position: Vec3::splat(world_radius_in_cells + 1.0),
            expected_sector_id: sector::ID::MAX,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct WorldToSectorCoordinates {
    description: String,
    world_position: Vec3,
    expected_sector_coordinates: IVec3,
}

impl WorldToSectorCoordinates {
    pub fn check(&self, world: &World) {
        let sector_coordinates =
            Grid::world_to_sector_coordinates(&world.grid, self.world_position);

        assert_eq!(
            sector_coordinates, self.expected_sector_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn world_to_sector_coordinates() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_sectors = world.grid.world_radius_in_sectors as i32;
    let world_radius_in_cells = world.grid.world_radius_in_cells as f32;

    let test_cases = vec![
        WorldToSectorCoordinates {
            description: "world min".to_string(),
            world_position: Vec3::splat(world_radius_in_cells),
            expected_sector_coordinates: IVec3::splat(world_radius_in_sectors),
        },
        WorldToSectorCoordinates {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_sector_coordinates: IVec3::splat(0),
        },
        WorldToSectorCoordinates {
            description: "world max".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells),
            expected_sector_coordinates: IVec3::splat(-world_radius_in_sectors),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct WorldToCellIDCase {
    description: String,
    world_position: Vec3,
    expected_cell_id: cell::ID,
}

impl WorldToCellIDCase {
    pub fn check(&self, world: &World) {
        let cell_id = Grid::world_to_cell_id(&world.grid, self.world_position);

        assert_eq!(cell_id, self.expected_cell_id, "{:?}", self.description);
    }
}

#[test]
fn world_to_cell_id() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as f32;

    let test_cases = vec![
        WorldToCellIDCase {
            description: "world min".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells),
            expected_cell_id: cell::ID(0),
        },
        WorldToCellIDCase {
            description: "world min - 1.0".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells - 1.0),
            expected_cell_id: cell::ID::MAX,
        },
        WorldToCellIDCase {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_cell_id: cell::ID((world.grid.sector_volume_in_cells - 1) / 2),
        },
        WorldToCellIDCase {
            description: "world max".to_string(),
            world_position: Vec3::splat(world_radius_in_cells),
            expected_cell_id: cell::ID(world.grid.sector_volume_in_cells - 1),
        },
        WorldToCellIDCase {
            description: "world max + 1.0".to_string(),
            world_position: Vec3::splat(world_radius_in_cells + 1.0),
            expected_cell_id: cell::ID::MAX,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct WorldToCellCoordinates {
    description: String,
    world_position: Vec3,
    expected_cell_coordinates: IVec3,
}

impl WorldToCellCoordinates {
    pub fn check(&self, world: &World) {
        let cell_coordinates = Grid::world_to_cell_coordinates(&world.grid, self.world_position);

        assert_eq!(
            cell_coordinates, self.expected_cell_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn world_to_cell_coordinates() {
    let simulation_kind = simulation::Kind::Empty;

    let mut world = World::new(simulation_kind);
    constructor::world::construct(simulation_kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;
    let world_radius_in_cells = world.grid.world_radius_in_cells as f32;

    let test_cases = vec![
        WorldToCellCoordinates {
            description: "world min".to_string(),
            world_position: Vec3::splat(world_radius_in_cells),
            expected_cell_coordinates: IVec3::splat(sector_radius_in_cells),
        },
        WorldToCellCoordinates {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_cell_coordinates: IVec3::splat(0),
        },
        WorldToCellCoordinates {
            description: "world max".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells),
            expected_cell_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
