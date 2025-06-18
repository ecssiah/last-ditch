use crate::simulation::{
    consts::*,
    state::world::{block, chunk, World},
};
use glam::IVec3;

struct BlockIDValidCase {
    description: String,
    block_id: block::ID,
    expected_valid: bool,
}

impl BlockIDValidCase {
    pub fn check(&self, world: &World) {
        let valid = world.grid.block_id_valid(self.block_id);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn block_id_valid() {
    let world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

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
        let valid = world.grid.chunk_id_valid(self.chunk_id);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn chunk_id_valid() {
    let world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

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
        let valid = world.grid.position_valid(self.position);

        assert_eq!(valid, self.expected_valid, "{:?}", self.description);
    }
}

#[test]
fn position_valid() {
    let world = World::new(TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS);

    let world_boundary = world.grid.world_boundary as i32;

    let test_cases = vec![
        PositionValidCase {
            description: "(0, 0, 0)".to_string(),
            position: IVec3::new(0, 0, 0),
            expected_valid: true,
        },
        PositionValidCase {
            description: "(world_boundary, world_boundary, world_boundary)".to_string(),
            position: IVec3::splat(world_boundary),
            expected_valid: true,
        },
        PositionValidCase {
            description: "(-world_boundary, -world_boundary, -world_boundary)".to_string(),
            position: IVec3::splat(-world_boundary),
            expected_valid: true,
        },
        PositionValidCase {
            description: "(world_boundary + 1, world_boundary + 1, world_boundary + 1)".to_string(),
            position: IVec3::splat(world_boundary + 1),
            expected_valid: false,
        },
        PositionValidCase {
            description: "(-world_boundary - 1, -world_boundary - 1, -world_boundary - 1)"
                .to_string(),
            position: IVec3::splat(-world_boundary - 1),
            expected_valid: false,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
