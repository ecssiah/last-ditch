use crate::simulation::{
    self,
    state::world::{block, chunk, grid::Grid, World},
};
use glam::IVec3;

struct BlockIDValidCase {
    description: String,
    block_id: block::ID,
    expected_valid: bool,
}

impl BlockIDValidCase {
    pub fn check(&self, world: &World) {
        let valid = Grid::block_id_valid(&world.grid, self.block_id);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn block_id_valid() {
    let kind = simulation::Kind::WorldTest;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        BlockIDValidCase {
            description: "block_id 0".to_string(),
            block_id: block::ID(0),
            expected_valid: true,
        },
        BlockIDValidCase {
            description: "block_id max".to_string(),
            block_id: block::ID(world.grid.block_index_max),
            expected_valid: true,
        },
        BlockIDValidCase {
            description: "block_id max + 1".to_string(),
            block_id: block::ID(world.grid.block_index_max + 1),
            expected_valid: false,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct ChunkIDValidCase {
    description: String,
    chunk_id: chunk::ID,
    expected_valid: bool,
}

impl ChunkIDValidCase {
    pub fn check(&self, world: &World) {
        let valid = Grid::chunk_id_valid(&world.grid, self.chunk_id);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn chunk_id_valid() {
    let kind = simulation::Kind::WorldTest;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        ChunkIDValidCase {
            description: "chunk_id 0".to_string(),
            chunk_id: chunk::ID(0),
            expected_valid: true,
        },
        ChunkIDValidCase {
            description: "chunk_id max".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max),
            expected_valid: true,
        },
        ChunkIDValidCase {
            description: "chunk_id max + 1".to_string(),
            chunk_id: chunk::ID(world.grid.chunk_index_max + 1),
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
        let valid = Grid::position_valid(&world.grid, self.position);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn position_valid() {
    let kind = simulation::Kind::Empty;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let world_limit = world.grid.world_limit as i32;

    let test_cases = vec![
        PositionValidCase {
            description: "(0, 0, 0)".to_string(),
            position: IVec3::new(0, 0, 0),
            expected_valid: true,
        },
        PositionValidCase {
            description: "(world_limit, world_limit, world_limit)".to_string(),
            position: IVec3::splat(world_limit),
            expected_valid: true,
        },
        PositionValidCase {
            description: "(-world_limit, -world_limit, -world_limit)".to_string(),
            position: IVec3::splat(-world_limit),
            expected_valid: true,
        },
        PositionValidCase {
            description: "(world_limit + 1, world_limit + 1, world_limit + 1)".to_string(),
            position: IVec3::splat(world_limit + 1),
            expected_valid: false,
        },
        PositionValidCase {
            description: "(-world_limit - 1, -world_limit - 1, -world_limit - 1)".to_string(),
            position: IVec3::splat(-world_limit - 1),
            expected_valid: false,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
