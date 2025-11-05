use crate::simulation::{
    self,
    state::world::{block, grid::Grid, sector, World},
};
use glam::{IVec3, Vec3};

struct BlockIDToBlockCoordinatesCase {
    description: String,
    block_id: block::ID,
    expected_block_coordinates: IVec3,
}

impl BlockIDToBlockCoordinatesCase {
    pub fn check(&self, world: &World) {
        let block_coordinates = Grid::block_id_to_block_coordinates(&world.grid, self.block_id);

        assert_ne!(block_coordinates, IVec3::MAX, "{:?}", self.description);
        assert_eq!(
            block_coordinates, self.expected_block_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn block_id_to_block_coordinates() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let test_cases = vec![
        BlockIDToBlockCoordinatesCase {
            description: "block id min".to_string(),
            block_id: block::ID(0),
            expected_block_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
        BlockIDToBlockCoordinatesCase {
            description: "block id at origin".to_string(),
            block_id: block::ID((world.grid.sector_volume_in_cells - 1) / 2),
            expected_block_coordinates: IVec3::splat(0),
        },
        BlockIDToBlockCoordinatesCase {
            description: "block id max".to_string(),
            block_id: block::ID(world.grid.sector_volume_in_cells - 1),
            expected_block_coordinates: IVec3::splat(sector_radius_in_cells),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct BlockCoordinatesToBlockIDCase {
    description: String,
    block_coordinates: IVec3,
    expected_block_id: block::ID,
}

impl BlockCoordinatesToBlockIDCase {
    pub fn check(&self, world: &World) {
        let block_id = Grid::block_coordinates_to_block_id(&world.grid, self.block_coordinates);

        assert_ne!(block_id, block::ID::MAX, "{:?}", self.description);
        assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
    }
}

#[test]
fn block_coordinates_to_block_id() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let test_cases = vec![
        BlockCoordinatesToBlockIDCase {
            description: "block coordinates min".to_string(),
            block_coordinates: IVec3::splat(-sector_radius_in_cells),
            expected_block_id: block::ID(0),
        },
        BlockCoordinatesToBlockIDCase {
            description: "block coordinates origin".to_string(),
            block_coordinates: IVec3::splat(0),
            expected_block_id: block::ID((world.grid.sector_volume_in_cells - 1) / 2),
        },
        BlockCoordinatesToBlockIDCase {
            description: "block coordinates max".to_string(),
            block_coordinates: IVec3::splat(sector_radius_in_cells),
            expected_block_id: block::ID(world.grid.sector_volume_in_cells - 1),
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
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

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
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

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
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

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
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

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
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

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

struct PositionToBlockCoordinatesCase {
    description: String,
    position: IVec3,
    expected_block_coordinates: IVec3,
}

impl PositionToBlockCoordinatesCase {
    pub fn check(&self, world: &World) {
        let block_coordinates = Grid::position_to_block_coordinates(&world.grid, self.position);

        assert_ne!(block_coordinates, IVec3::MAX, "{:?}", self.description);
        assert_eq!(
            block_coordinates, self.expected_block_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn position_to_block_coordinates() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;
    let sector_size_in_cells = world.grid.sector_size_in_cells as i32;
    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        PositionToBlockCoordinatesCase {
            description: "origin sector min".to_string(),
            position: IVec3::splat(-sector_radius_in_cells),
            expected_block_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
        PositionToBlockCoordinatesCase {
            description: "origin sector origin".to_string(),
            position: IVec3::splat(0),
            expected_block_coordinates: IVec3::splat(0),
        },
        PositionToBlockCoordinatesCase {
            description: "origin sector max".to_string(),
            position: IVec3::splat(sector_radius_in_cells),
            expected_block_coordinates: IVec3::splat(sector_radius_in_cells),
        },
        PositionToBlockCoordinatesCase {
            description: "world min sector min".to_string(),
            position: IVec3::splat(-world_radius_in_cells),
            expected_block_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
        PositionToBlockCoordinatesCase {
            description: "world min sector origin".to_string(),
            position: IVec3::splat(-world_radius_in_cells + sector_radius_in_cells),
            expected_block_coordinates: IVec3::splat(0),
        },
        PositionToBlockCoordinatesCase {
            description: "world min sector max".to_string(),
            position: IVec3::splat(-world_radius_in_cells + sector_size_in_cells - 1),
            expected_block_coordinates: IVec3::splat(sector_radius_in_cells),
        },
        PositionToBlockCoordinatesCase {
            description: "world max sector min".to_string(),
            position: IVec3::splat(world_radius_in_cells - sector_size_in_cells + 1),
            expected_block_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
        PositionToBlockCoordinatesCase {
            description: "world max sector origin".to_string(),
            position: IVec3::splat(world_radius_in_cells - sector_radius_in_cells),
            expected_block_coordinates: IVec3::splat(0),
        },
        PositionToBlockCoordinatesCase {
            description: "world max sector max".to_string(),
            position: IVec3::splat(world_radius_in_cells),
            expected_block_coordinates: IVec3::splat(sector_radius_in_cells),
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
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

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

struct PositionToBlockIDCase {
    description: String,
    position: IVec3,
    expected_block_id: block::ID,
}

impl PositionToBlockIDCase {
    pub fn check(&self, world: &World) {
        let block_id = Grid::position_to_block_id(&world.grid, self.position);

        assert_ne!(block_id, block::ID::MAX, "{:?}", self.description);
        assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
    }
}

#[test]
fn position_to_block_id() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;

    let test_cases = vec![
        PositionToBlockIDCase {
            description: "position min".to_string(),
            position: IVec3::splat(-sector_radius_in_cells),
            expected_block_id: block::ID(0),
        },
        PositionToBlockIDCase {
            description: "position origin".to_string(),
            position: IVec3::splat(0),
            expected_block_id: block::ID((world.grid.sector_volume_in_cells - 1) / 2),
        },
        PositionToBlockIDCase {
            description: "position max".to_string(),
            position: IVec3::splat(sector_radius_in_cells),
            expected_block_id: block::ID(world.grid.sector_volume_in_cells - 1),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct IDsToPositionCase {
    description: String,
    sector_id: sector::ID,
    block_id: block::ID,
    expected_position: IVec3,
}

impl IDsToPositionCase {
    pub fn check(&self, world: &World) {
        let position = Grid::ids_to_position(&world.grid, self.sector_id, self.block_id);

        assert_ne!(position, IVec3::MAX, "{:?}", self.description);
        assert_eq!(position, self.expected_position, "{:?}", self.description);
    }
}

#[test]
fn ids_to_position() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        IDsToPositionCase {
            description: "ids at min".to_string(),
            sector_id: sector::ID(0),
            block_id: block::ID(0),
            expected_position: IVec3::splat(-world_radius_in_cells),
        },
        IDsToPositionCase {
            description: "ids at origin".to_string(),
            sector_id: sector::ID((world.grid.world_volume_in_sectors - 1) / 2),
            block_id: block::ID((world.grid.sector_volume_in_cells - 1) / 2),
            expected_position: IVec3::splat(0),
        },
        IDsToPositionCase {
            description: "ids at max".to_string(),
            sector_id: sector::ID(world.grid.world_volume_in_sectors - 1),
            block_id: block::ID(world.grid.sector_volume_in_cells - 1),
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
    expected_ids: (sector::ID, block::ID),
}

impl PositionToIDsCase {
    pub fn check(&self, world: &World) {
        let (sector_id, block_id) = Grid::position_to_ids(&world.grid, self.position);

        assert_ne!(sector_id, sector::ID::MAX, "{:?}", self.description);
        assert_ne!(block_id, block::ID::MAX, "{:?}", self.description);
        assert_eq!(
            (sector_id, block_id),
            self.expected_ids,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn position_to_ids() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as i32;

    let test_cases = vec![
        PositionToIDsCase {
            description: "position at min".to_string(),
            position: IVec3::splat(-world_radius_in_cells),
            expected_ids: (sector::ID(0), block::ID(0)),
        },
        PositionToIDsCase {
            description: "position at origin".to_string(),
            position: IVec3::splat(0),
            expected_ids: (
                sector::ID((world.grid.world_volume_in_sectors - 1) / 2),
                block::ID((world.grid.sector_volume_in_cells - 1) / 2),
            ),
        },
        PositionToIDsCase {
            description: "position at max".to_string(),
            position: IVec3::splat(world_radius_in_cells),
            expected_ids: (
                sector::ID(world.grid.world_volume_in_sectors - 1),
                block::ID(world.grid.sector_volume_in_cells - 1),
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
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

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
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

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
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

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

struct WorldToBlockIDCase {
    description: String,
    world_position: Vec3,
    expected_block_id: block::ID,
}

impl WorldToBlockIDCase {
    pub fn check(&self, world: &World) {
        let block_id = Grid::world_to_block_id(&world.grid, self.world_position);

        assert_eq!(block_id, self.expected_block_id, "{:?}", self.description);
    }
}

#[test]
fn world_to_block_id() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_radius_in_cells = world.grid.world_radius_in_cells as f32;

    let test_cases = vec![
        WorldToBlockIDCase {
            description: "world min".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells),
            expected_block_id: block::ID(0),
        },
        WorldToBlockIDCase {
            description: "world min - 1.0".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells - 1.0),
            expected_block_id: block::ID::MAX,
        },
        WorldToBlockIDCase {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_block_id: block::ID((world.grid.sector_volume_in_cells - 1) / 2),
        },
        WorldToBlockIDCase {
            description: "world max".to_string(),
            world_position: Vec3::splat(world_radius_in_cells),
            expected_block_id: block::ID(world.grid.sector_volume_in_cells - 1),
        },
        WorldToBlockIDCase {
            description: "world max + 1.0".to_string(),
            world_position: Vec3::splat(world_radius_in_cells + 1.0),
            expected_block_id: block::ID::MAX,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct WorldToBlockCoordinates {
    description: String,
    world_position: Vec3,
    expected_block_coordinates: IVec3,
}

impl WorldToBlockCoordinates {
    pub fn check(&self, world: &World) {
        let block_coordinates = Grid::world_to_block_coordinates(&world.grid, self.world_position);

        assert_eq!(
            block_coordinates, self.expected_block_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn world_to_block_coordinates() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let sector_radius_in_cells = world.grid.sector_radius_in_cells as i32;
    let world_radius_in_cells = world.grid.world_radius_in_cells as f32;

    let test_cases = vec![
        WorldToBlockCoordinates {
            description: "world min".to_string(),
            world_position: Vec3::splat(world_radius_in_cells),
            expected_block_coordinates: IVec3::splat(sector_radius_in_cells),
        },
        WorldToBlockCoordinates {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_block_coordinates: IVec3::splat(0),
        },
        WorldToBlockCoordinates {
            description: "world max".to_string(),
            world_position: Vec3::splat(-world_radius_in_cells),
            expected_block_coordinates: IVec3::splat(-sector_radius_in_cells),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
