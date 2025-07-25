use crate::simulation::{
    self,
    state::world::{block, chunk, grid::Grid, World},
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

    let chunk_radius = world.grid.chunk_radius as i32;

    let test_cases = vec![
        BlockIDToBlockCoordinatesCase {
            description: "block id min".to_string(),
            block_id: block::ID(0),
            expected_block_coordinates: IVec3::splat(-chunk_radius),
        },
        BlockIDToBlockCoordinatesCase {
            description: "block id at origin".to_string(),
            block_id: block::ID(world.grid.block_index_max / 2),
            expected_block_coordinates: IVec3::splat(0),
        },
        BlockIDToBlockCoordinatesCase {
            description: "block id max".to_string(),
            block_id: block::ID(world.grid.block_index_max),
            expected_block_coordinates: IVec3::splat(chunk_radius),
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

    let chunk_radius = world.grid.chunk_radius as i32;

    let test_cases = vec![
        BlockCoordinatesToBlockIDCase {
            description: "block coordinates min".to_string(),
            block_coordinates: IVec3::splat(-chunk_radius),
            expected_block_id: block::ID(0),
        },
        BlockCoordinatesToBlockIDCase {
            description: "block coordinates origin".to_string(),
            block_coordinates: IVec3::splat(0),
            expected_block_id: block::ID(world.grid.block_index_max / 2),
        },
        BlockCoordinatesToBlockIDCase {
            description: "block coordinates max".to_string(),
            block_coordinates: IVec3::splat(chunk_radius),
            expected_block_id: block::ID(world.grid.block_index_max),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct ChunkIDToChunkCoordinates {
    description: String,
    chunk_id: chunk::ID,
    expected_chunk_coordinates: IVec3,
}

impl ChunkIDToChunkCoordinates {
    pub fn check(&self, world: &World) {
        let chunk_coordinates = Grid::chunk_id_to_chunk_coordinates(&world.grid, self.chunk_id);

        assert_ne!(chunk_coordinates, IVec3::MAX, "{:?}", self.description);
        assert_eq!(
            chunk_coordinates, self.expected_chunk_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn chunk_id_to_chunk_coordinates() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_radius = world.grid.world_radius as i32;

    let test_cases = vec![
        ChunkIDToChunkCoordinates {
            description: "chunk id min".to_string(),
            chunk_id: chunk::ID(0),
            expected_chunk_coordinates: IVec3::splat(-world_radius),
        },
        ChunkIDToChunkCoordinates {
            description: "chunk id at origin".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max / 2),
            expected_chunk_coordinates: IVec3::splat(0),
        },
        ChunkIDToChunkCoordinates {
            description: "chunk id max".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max),
            expected_chunk_coordinates: IVec3::splat(world_radius),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct ChunkCoordinatesToChunkIDCase {
    description: String,
    chunk_coordinates: IVec3,
    expected_chunk_id: chunk::ID,
}

impl ChunkCoordinatesToChunkIDCase {
    pub fn check(&self, world: &World) {
        let chunk_id = Grid::chunk_coordinates_to_chunk_id(&world.grid, self.chunk_coordinates);

        assert_ne!(chunk_id, chunk::ID::MAX, "{:?}", self.description);
        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
    }
}

#[test]
fn chunk_coordinates_to_chunk_id() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_radius = world.grid.world_radius as i32;

    let test_cases = vec![
        ChunkCoordinatesToChunkIDCase {
            description: "chunk coordinates min".to_string(),
            chunk_coordinates: IVec3::splat(-world_radius),
            expected_chunk_id: chunk::ID(0),
        },
        ChunkCoordinatesToChunkIDCase {
            description: "chunk coordinates origin".to_string(),
            chunk_coordinates: IVec3::splat(0),
            expected_chunk_id: chunk::ID(world.grid.chunk_index_max / 2),
        },
        ChunkCoordinatesToChunkIDCase {
            description: "chunk coordinates max".to_string(),
            chunk_coordinates: IVec3::splat(world_radius),
            expected_chunk_id: chunk::ID(world.grid.chunk_index_max),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct ChunkCoordinatesToPositionCase {
    description: String,
    chunk_coordinates: IVec3,
    expected_position: IVec3,
}

impl ChunkCoordinatesToPositionCase {
    pub fn check(&self, world: &World) {
        let position = Grid::chunk_coordinates_to_position(&world.grid, self.chunk_coordinates);

        assert_ne!(position, IVec3::MAX, "{:?}", self.description);
        assert_eq!(position, self.expected_position, "{:?}", self.description);
    }
}

#[test]
fn chunk_coordinates_to_position() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let chunk_radius = world.grid.chunk_radius as i32;
    let world_radius = world.grid.world_radius as i32;
    let world_limit = world.grid.world_limit as i32;

    let test_cases = vec![
        ChunkCoordinatesToPositionCase {
            description: "chunk min".to_string(),
            chunk_coordinates: IVec3::splat(-world_radius),
            expected_position: IVec3::splat(-world_limit + chunk_radius),
        },
        ChunkCoordinatesToPositionCase {
            description: "chunk origin".to_string(),
            chunk_coordinates: IVec3::splat(0),
            expected_position: IVec3::splat(0),
        },
        ChunkCoordinatesToPositionCase {
            description: "chunk max".to_string(),
            chunk_coordinates: IVec3::splat(world_radius),
            expected_position: IVec3::splat(world_limit - chunk_radius),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct ChunkIDToPositionCase {
    description: String,
    chunk_id: chunk::ID,
    expected_position: IVec3,
}

impl ChunkIDToPositionCase {
    pub fn check(&self, world: &World) {
        let position = Grid::chunk_id_to_position(&world.grid, self.chunk_id);

        assert_ne!(position, IVec3::MAX, "{:?}", self.description);
        assert_eq!(position, self.expected_position, "{:?}", self.description);
    }
}

#[test]
fn chunk_id_to_position() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_limit = world.grid.world_limit as i32;
    let chunk_radius = world.grid.chunk_radius as i32;

    let test_cases = vec![
        ChunkIDToPositionCase {
            description: "chunk_id min".to_string(),
            chunk_id: chunk::ID(0),
            expected_position: IVec3::splat(-world_limit + chunk_radius),
        },
        ChunkIDToPositionCase {
            description: "chunk_id origin".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max / 2),
            expected_position: IVec3::splat(0),
        },
        ChunkIDToPositionCase {
            description: "chunk_id max".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max),
            expected_position: IVec3::splat(world_limit - chunk_radius),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PositionToChunkCoordinatesCase {
    description: String,
    position: IVec3,
    expected_chunk_coordinates: IVec3,
}

impl PositionToChunkCoordinatesCase {
    pub fn check(&self, world: &World) {
        let chunk_coordinates = Grid::position_to_chunk_coordinates(&world.grid, self.position);

        assert_ne!(chunk_coordinates, IVec3::MAX, "{:?}", self.description);
        assert_eq!(
            chunk_coordinates, self.expected_chunk_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn position_to_chunk_coordinates() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let chunk_radius = world.grid.chunk_radius as i32;
    let world_radius = world.grid.world_radius as i32;
    let world_limit = world.grid.world_limit as i32;

    let test_cases = vec![
        PositionToChunkCoordinatesCase {
            description: "chunk min".to_string(),
            position: IVec3::splat(-world_limit + chunk_radius),
            expected_chunk_coordinates: IVec3::splat(-world_radius),
        },
        PositionToChunkCoordinatesCase {
            description: "chunk origin".to_string(),
            position: IVec3::splat(0),
            expected_chunk_coordinates: IVec3::splat(0),
        },
        PositionToChunkCoordinatesCase {
            description: "chunk max".to_string(),
            position: IVec3::splat(world_limit - chunk_radius),
            expected_chunk_coordinates: IVec3::splat(world_radius),
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

    let chunk_radius = world.grid.chunk_radius as i32;
    let chunk_size = world.grid.chunk_size as i32;
    let world_limit = world.grid.world_limit as i32;

    let test_cases = vec![
        PositionToBlockCoordinatesCase {
            description: "origin chunk min".to_string(),
            position: IVec3::splat(-chunk_radius),
            expected_block_coordinates: IVec3::splat(-chunk_radius),
        },
        PositionToBlockCoordinatesCase {
            description: "origin chunk origin".to_string(),
            position: IVec3::splat(0),
            expected_block_coordinates: IVec3::splat(0),
        },
        PositionToBlockCoordinatesCase {
            description: "origin chunk max".to_string(),
            position: IVec3::splat(chunk_radius),
            expected_block_coordinates: IVec3::splat(chunk_radius),
        },
        PositionToBlockCoordinatesCase {
            description: "world min chunk min".to_string(),
            position: IVec3::splat(-world_limit),
            expected_block_coordinates: IVec3::splat(-chunk_radius),
        },
        PositionToBlockCoordinatesCase {
            description: "world min chunk origin".to_string(),
            position: IVec3::splat(-world_limit + chunk_radius),
            expected_block_coordinates: IVec3::splat(0),
        },
        PositionToBlockCoordinatesCase {
            description: "world min chunk max".to_string(),
            position: IVec3::splat(-world_limit + chunk_size - 1),
            expected_block_coordinates: IVec3::splat(chunk_radius),
        },
        PositionToBlockCoordinatesCase {
            description: "world max chunk min".to_string(),
            position: IVec3::splat(world_limit - chunk_size + 1),
            expected_block_coordinates: IVec3::splat(-chunk_radius),
        },
        PositionToBlockCoordinatesCase {
            description: "world max chunk origin".to_string(),
            position: IVec3::splat(world_limit - chunk_radius),
            expected_block_coordinates: IVec3::splat(0),
        },
        PositionToBlockCoordinatesCase {
            description: "world max chunk max".to_string(),
            position: IVec3::splat(world_limit),
            expected_block_coordinates: IVec3::splat(chunk_radius),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PositionToChunkIDCase {
    description: String,
    position: IVec3,
    expected_chunk_id: chunk::ID,
}

impl PositionToChunkIDCase {
    pub fn check(&self, world: &World) {
        let chunk_id = Grid::position_to_chunk_id(&world.grid, self.position);

        assert_ne!(chunk_id, chunk::ID::MAX, "{:?}", self.description);
        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
    }
}

#[test]
fn position_to_chunk_id() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_limit = world.grid.world_limit as i32;

    let test_cases = vec![
        PositionToChunkIDCase {
            description: "position min".to_string(),
            position: IVec3::splat(-world_limit),
            expected_chunk_id: chunk::ID(0),
        },
        PositionToChunkIDCase {
            description: "position origin".to_string(),
            position: IVec3::splat(0),
            expected_chunk_id: chunk::ID(world.grid.chunk_index_max / 2),
        },
        PositionToChunkIDCase {
            description: "position max".to_string(),
            position: IVec3::splat(world_limit),
            expected_chunk_id: chunk::ID(world.grid.chunk_index_max),
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

    let chunk_radius = world.grid.chunk_radius as i32;

    let test_cases = vec![
        PositionToBlockIDCase {
            description: "position min".to_string(),
            position: IVec3::splat(-chunk_radius),
            expected_block_id: block::ID(0),
        },
        PositionToBlockIDCase {
            description: "position origin".to_string(),
            position: IVec3::splat(0),
            expected_block_id: block::ID(world.grid.block_index_max / 2),
        },
        PositionToBlockIDCase {
            description: "position max".to_string(),
            position: IVec3::splat(chunk_radius),
            expected_block_id: block::ID(world.grid.block_index_max),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct IDsToPositionCase {
    description: String,
    chunk_id: chunk::ID,
    block_id: block::ID,
    expected_position: IVec3,
}

impl IDsToPositionCase {
    pub fn check(&self, world: &World) {
        let position = Grid::ids_to_position(&world.grid, self.chunk_id, self.block_id);

        assert_ne!(position, IVec3::MAX, "{:?}", self.description);
        assert_eq!(position, self.expected_position, "{:?}", self.description);
    }
}

#[test]
fn ids_to_position() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_limit = world.grid.world_limit as i32;

    let test_cases = vec![
        IDsToPositionCase {
            description: "ids at min".to_string(),
            chunk_id: chunk::ID(0),
            block_id: block::ID(0),
            expected_position: IVec3::splat(-world_limit),
        },
        IDsToPositionCase {
            description: "ids at origin".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max / 2),
            block_id: block::ID(world.grid.block_index_max / 2),
            expected_position: IVec3::splat(0),
        },
        IDsToPositionCase {
            description: "ids at max".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max),
            block_id: block::ID(world.grid.block_index_max),
            expected_position: IVec3::splat(world_limit),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct PositionToIDsCase {
    description: String,
    position: IVec3,
    expected_ids: (chunk::ID, block::ID),
}

impl PositionToIDsCase {
    pub fn check(&self, world: &World) {
        let (chunk_id, block_id) = Grid::position_to_ids(&world.grid, self.position);

        assert_ne!(chunk_id, chunk::ID::MAX, "{:?}", self.description);
        assert_ne!(block_id, block::ID::MAX, "{:?}", self.description);
        assert_eq!(
            (chunk_id, block_id),
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

    let world_limit = world.grid.world_limit as i32;

    let test_cases = vec![
        PositionToIDsCase {
            description: "position at min".to_string(),
            position: IVec3::splat(-world_limit),
            expected_ids: (chunk::ID(0), block::ID(0)),
        },
        PositionToIDsCase {
            description: "position at origin".to_string(),
            position: IVec3::splat(0),
            expected_ids: (
                chunk::ID(world.grid.chunk_index_max / 2),
                block::ID(world.grid.block_index_max / 2),
            ),
        },
        PositionToIDsCase {
            description: "position at max".to_string(),
            position: IVec3::splat(world_limit),
            expected_ids: (
                chunk::ID(world.grid.chunk_index_max),
                block::ID(world.grid.block_index_max),
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

    let world_limit = world.grid.world_limit as f32;

    let test_cases = vec![
        WorldToPositionCase {
            description: "world min".to_string(),
            world_position: Vec3::splat(-world_limit),
            expected_position: IVec3::splat(-world_limit as i32),
        },
        WorldToPositionCase {
            description: "world min - 1.0".to_string(),
            world_position: Vec3::splat(-world_limit - 1.0),
            expected_position: IVec3::MAX,
        },
        WorldToPositionCase {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_position: IVec3::splat(0),
        },
        WorldToPositionCase {
            description: "world max".to_string(),
            world_position: Vec3::splat(world_limit),
            expected_position: IVec3::splat(world_limit as i32),
        },
        WorldToPositionCase {
            description: "world max + 1.0".to_string(),
            world_position: Vec3::splat(world_limit + 1.0),
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

struct WorldToChunkIDCase {
    description: String,
    world_position: Vec3,
    expected_chunk_id: chunk::ID,
}

impl WorldToChunkIDCase {
    pub fn check(&self, world: &World) {
        let chunk_id = Grid::world_to_chunk_id(&world.grid, self.world_position);

        assert_eq!(chunk_id, self.expected_chunk_id, "{:?}", self.description);
    }
}

#[test]
fn world_to_chunk_id() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_limit = world.grid.world_limit as f32;

    let test_cases = vec![
        WorldToChunkIDCase {
            description: "world min".to_string(),
            world_position: Vec3::splat(-world_limit),
            expected_chunk_id: chunk::ID(0),
        },
        WorldToChunkIDCase {
            description: "world min - 1.0".to_string(),
            world_position: Vec3::splat(-world_limit - 1.0),
            expected_chunk_id: chunk::ID::MAX,
        },
        WorldToChunkIDCase {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_chunk_id: chunk::ID(world.grid.chunk_index_max / 2),
        },
        WorldToChunkIDCase {
            description: "world max".to_string(),
            world_position: Vec3::splat(world_limit),
            expected_chunk_id: chunk::ID(world.grid.chunk_index_max),
        },
        WorldToChunkIDCase {
            description: "world max + 1.0".to_string(),
            world_position: Vec3::splat(world_limit + 1.0),
            expected_chunk_id: chunk::ID::MAX,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct WorldToChunkCoordinates {
    description: String,
    world_position: Vec3,
    expected_chunk_coordinates: IVec3,
}

impl WorldToChunkCoordinates {
    pub fn check(&self, world: &World) {
        let chunk_coordinates = Grid::world_to_chunk_coordinates(&world.grid, self.world_position);

        assert_eq!(
            chunk_coordinates, self.expected_chunk_coordinates,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn world_to_chunk_coordinates() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_radius = world.grid.world_radius as i32;
    let world_limit = world.grid.world_limit as f32;

    let test_cases = vec![
        WorldToChunkCoordinates {
            description: "world min".to_string(),
            world_position: Vec3::splat(world_limit),
            expected_chunk_coordinates: IVec3::splat(world_radius),
        },
        WorldToChunkCoordinates {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_chunk_coordinates: IVec3::splat(0),
        },
        WorldToChunkCoordinates {
            description: "world max".to_string(),
            world_position: Vec3::splat(-world_limit),
            expected_chunk_coordinates: IVec3::splat(-world_radius),
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

    let world_limit = world.grid.world_limit as f32;

    let test_cases = vec![
        WorldToBlockIDCase {
            description: "world min".to_string(),
            world_position: Vec3::splat(-world_limit),
            expected_block_id: block::ID(0),
        },
        WorldToBlockIDCase {
            description: "world min - 1.0".to_string(),
            world_position: Vec3::splat(-world_limit - 1.0),
            expected_block_id: block::ID::MAX,
        },
        WorldToBlockIDCase {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_block_id: block::ID(world.grid.block_index_max / 2),
        },
        WorldToBlockIDCase {
            description: "world max".to_string(),
            world_position: Vec3::splat(world_limit),
            expected_block_id: block::ID(world.grid.block_index_max),
        },
        WorldToBlockIDCase {
            description: "world max + 1.0".to_string(),
            world_position: Vec3::splat(world_limit + 1.0),
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

    let chunk_radius = world.grid.chunk_radius as i32;
    let world_limit = world.grid.world_limit as f32;

    let test_cases = vec![
        WorldToBlockCoordinates {
            description: "world min".to_string(),
            world_position: Vec3::splat(world_limit),
            expected_block_coordinates: IVec3::splat(chunk_radius),
        },
        WorldToBlockCoordinates {
            description: "world origin".to_string(),
            world_position: Vec3::splat(0.0),
            expected_block_coordinates: IVec3::splat(0),
        },
        WorldToBlockCoordinates {
            description: "world max".to_string(),
            world_position: Vec3::splat(-world_limit),
            expected_block_coordinates: IVec3::splat(-chunk_radius),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
