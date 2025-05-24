use glam::{IVec3, Vec3};
use last_ditch::simulation::{
    consts::*,
    world::{block, chunk, grid},
};

struct WorldToGridTestCase {
    world_position: Vec3,
    expected_grid_position: Option<IVec3>,
}

#[test]
fn world_to_grid() {
    let test_cases = vec![
        WorldToGridTestCase {
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        WorldToGridTestCase {
            world_position: Vec3::new(
                GRID_BOUNDARY as f32,
                GRID_BOUNDARY as f32,
                GRID_BOUNDARY as f32,
            ),
            expected_grid_position: Some(IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            )),
        },
        WorldToGridTestCase {
            world_position: Vec3::new(
                (GRID_BOUNDARY as f32) + 1.0,
                (GRID_BOUNDARY as f32) + 1.0,
                (GRID_BOUNDARY as f32) + 1.0,
            ),
            expected_grid_position: None,
        },
        WorldToGridTestCase {
            world_position: Vec3::new(
                -(GRID_BOUNDARY as f32),
                -(GRID_BOUNDARY as f32),
                -(GRID_BOUNDARY as f32),
            ),
            expected_grid_position: Some(IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            )),
        },
        WorldToGridTestCase {
            world_position: Vec3::new(
                -(GRID_BOUNDARY as f32) - 1.0,
                -(GRID_BOUNDARY as f32) - 1.0,
                -(GRID_BOUNDARY as f32) - 1.0,
            ),
            expected_grid_position: None,
        },
    ];

    for test_case in test_cases {
        let grid_position = grid::world_to_grid(test_case.world_position);

        assert_eq!(
            grid_position, test_case.expected_grid_position,
            "Expected {:?} to be {:?}",
            grid_position, test_case.expected_grid_position
        );
    }
}

struct GridToChunkTestCase {
    grid_position: IVec3,
    expected_chunk_position: Option<IVec3>,
}

#[test]
fn grid_to_chunk() {
    let test_cases = vec![
        GridToChunkTestCase {
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        GridToChunkTestCase {
            grid_position: IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            ),
            expected_chunk_position: Some(IVec3::new(
                WORLD_RADIUS as i32,
                WORLD_RADIUS as i32,
                WORLD_RADIUS as i32,
            )),
        },
        GridToChunkTestCase {
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_chunk_position: None,
        },
        GridToChunkTestCase {
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            ),
            expected_chunk_position: Some(IVec3::new(
                -(WORLD_RADIUS as i32),
                -(WORLD_RADIUS as i32),
                -(WORLD_RADIUS as i32),
            )),
        },
        GridToChunkTestCase {
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
            ),
            expected_chunk_position: None,
        },
    ];

    for test_case in test_cases {
        let chunk_position = grid::grid_to_chunk(test_case.grid_position);

        assert_eq!(
            chunk_position, test_case.expected_chunk_position,
            "Expected {:?} to be {:?}",
            chunk_position, test_case.expected_chunk_position
        );
    }
}

struct GridToBlockTestCase {
    grid_position: IVec3,
    expected_block_position: Option<IVec3>,
}

#[test]
fn grid_to_block() {
    let test_cases = vec![
        GridToBlockTestCase {
            grid_position: IVec3::new(0, 0, 0),
            expected_block_position: Some(IVec3::new(0, 0, 0)),
        },
        GridToBlockTestCase {
            grid_position: IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            ),
            expected_block_position: Some(IVec3::new(
                CHUNK_RADIUS as i32,
                CHUNK_RADIUS as i32,
                CHUNK_RADIUS as i32,
            )),
        },
        GridToBlockTestCase {
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_block_position: None,
        },
        GridToBlockTestCase {
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            ),
            expected_block_position: Some(IVec3::new(
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
            )),
        },
        GridToBlockTestCase {
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
            ),
            expected_block_position: None,
        },
    ];

    for test_case in test_cases {
        let block_position = grid::grid_to_block(test_case.grid_position);

        assert_eq!(
            block_position, test_case.expected_block_position,
            "Expected {:?} to be {:?}",
            block_position, test_case.expected_block_position
        );
    }
}

struct ChunkToGridTestCase {
    chunk_position: IVec3,
    expected_grid_position: Option<IVec3>,
}

#[test]
fn chunk_to_grid() {
    let test_cases = vec![
        ChunkToGridTestCase {
            chunk_position: IVec3::new(0, 0, 0),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkToGridTestCase {
            chunk_position: IVec3::new(
                WORLD_RADIUS as i32,
                WORLD_RADIUS as i32,
                WORLD_RADIUS as i32,
            ),
            expected_grid_position: Some(IVec3::new(
                WORLD_RADIUS as i32 * CHUNK_SIZE as i32,
                WORLD_RADIUS as i32 * CHUNK_SIZE as i32,
                WORLD_RADIUS as i32 * CHUNK_SIZE as i32,
            )),
        },
        ChunkToGridTestCase {
            chunk_position: IVec3::new(
                (WORLD_RADIUS as i32) + 1,
                (WORLD_RADIUS as i32) + 1,
                (WORLD_RADIUS as i32) + 1,
            ),
            expected_grid_position: None,
        },
        ChunkToGridTestCase {
            chunk_position: IVec3::new(
                -(WORLD_RADIUS as i32),
                -(WORLD_RADIUS as i32),
                -(WORLD_RADIUS as i32),
            ),
            expected_grid_position: Some(IVec3::new(
                -(WORLD_RADIUS as i32) * CHUNK_SIZE as i32,
                -(WORLD_RADIUS as i32) * CHUNK_SIZE as i32,
                -(WORLD_RADIUS as i32) * CHUNK_SIZE as i32,
            )),
        },
        ChunkToGridTestCase {
            chunk_position: IVec3::new(
                -(WORLD_RADIUS as i32) - 1,
                -(WORLD_RADIUS as i32) - 1,
                -(WORLD_RADIUS as i32) - 1,
            ),
            expected_grid_position: None,
        },
    ];

    for test_case in test_cases {
        let grid_position = grid::chunk_to_grid(test_case.chunk_position);

        assert_eq!(
            grid_position, test_case.expected_grid_position,
            "Failed at {:?}",
            test_case.chunk_position
        );
    }
}

struct GridToChunkIDTestCase {
    grid_position: IVec3,
    expected_chunk_id: Option<chunk::ID>,
}

#[test]
fn grid_to_chunk_id() {
    let test_cases = vec![
        GridToChunkIDTestCase {
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_id: Some(chunk::ID((WORLD_VOLUME - 1) / 2)),
        },
        GridToChunkIDTestCase {
            grid_position: IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            ),
            expected_chunk_id: Some(chunk::ID(WORLD_VOLUME - 1)),
        },
        GridToChunkIDTestCase {
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_chunk_id: None,
        },
        GridToChunkIDTestCase {
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            ),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        GridToChunkIDTestCase {
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
            ),
            expected_chunk_id: None,
        },
    ];

    for test_case in test_cases {
        let chunk_id = grid::grid_to_chunk_id(test_case.grid_position);

        assert_eq!(
            chunk_id, test_case.expected_chunk_id,
            "Failed at {:?}",
            test_case.grid_position
        );
    }
}

struct WorldToChunkIDTestCase {
    world_position: Vec3,
    expected_chunk_id: Option<chunk::ID>,
}

#[test]
fn world_to_chunk_id() {
    let test_cases = vec![
        WorldToChunkIDTestCase {
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_chunk_id: Some(chunk::ID((WORLD_VOLUME - 1) / 2)),
        },
        WorldToChunkIDTestCase {
            world_position: Vec3::new(
                GRID_BOUNDARY as f32,
                GRID_BOUNDARY as f32,
                GRID_BOUNDARY as f32,
            ),
            expected_chunk_id: Some(chunk::ID(WORLD_VOLUME - 1)),
        },
        WorldToChunkIDTestCase {
            world_position: Vec3::new(
                (GRID_BOUNDARY as f32) + 1.0,
                (GRID_BOUNDARY as f32) + 1.0,
                (GRID_BOUNDARY as f32) + 1.0,
            ),
            expected_chunk_id: None,
        },
        WorldToChunkIDTestCase {
            world_position: Vec3::new(
                -(GRID_BOUNDARY as f32),
                -(GRID_BOUNDARY as f32),
                -(GRID_BOUNDARY as f32),
            ),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        WorldToChunkIDTestCase {
            world_position: Vec3::new(
                -(GRID_BOUNDARY as f32) - 1.0,
                -(GRID_BOUNDARY as f32) - 1.0,
                -(GRID_BOUNDARY as f32) - 1.0,
            ),
            expected_chunk_id: None,
        },
    ];

    for test_case in test_cases {
        let chunk_id = grid::world_to_chunk_id(test_case.world_position);

        assert_eq!(
            chunk_id, test_case.expected_chunk_id,
            "Failed at {:?}",
            test_case.world_position
        );
    }
}

struct ChunkIDToPositionTestCase {
    chunk_id: chunk::ID,
    expected_chunk_position: Option<IVec3>,
}

#[test]
fn chunk_id_to_position() {
    let test_cases = vec![
        ChunkIDToPositionTestCase {
            chunk_id: chunk::ID(0),
            expected_chunk_position: Some(IVec3::new(
                -(WORLD_RADIUS as i32),
                -(WORLD_RADIUS as i32),
                -(WORLD_RADIUS as i32),
            )),
        },
        ChunkIDToPositionTestCase {
            chunk_id: chunk::ID(CHUNK_ID_MAX / 2),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkIDToPositionTestCase {
            chunk_id: chunk::ID::MAX,
            expected_chunk_position: Some(IVec3::new(
                WORLD_RADIUS as i32,
                WORLD_RADIUS as i32,
                WORLD_RADIUS as i32,
            )),
        },
        ChunkIDToPositionTestCase {
            chunk_id: chunk::ID(WORLD_VOLUME),
            expected_chunk_position: None,
        },
    ];

    for test_case in test_cases {
        let chunk_position = grid::chunk_id_to_position(test_case.chunk_id);

        assert_eq!(
            chunk_position, test_case.expected_chunk_position,
            "Failed at {:?}",
            test_case.chunk_id
        );
    }
}

struct GridToBlockIDTestCase {
    grid_position: IVec3,
    expected_block_id: Option<block::ID>,
}

#[test]
fn grid_to_block_id() {
    let test_cases = vec![
        GridToBlockIDTestCase {
            grid_position: IVec3::new(0, 0, 0),
            expected_block_id: Some(block::ID(BLOCK_ID_MAX / 2)),
        },
        GridToBlockIDTestCase {
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
            ),
            expected_block_id: None,
        },
        GridToBlockIDTestCase {
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_block_id: None,
        },
        GridToBlockIDTestCase {
            grid_position: IVec3::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32, CHUNK_SIZE as i32),
            expected_block_id: Some(block::ID(BLOCK_ID_MAX / 2)),
        },
        GridToBlockIDTestCase {
            grid_position: IVec3::new(
                -(CHUNK_SIZE as i32),
                -(CHUNK_SIZE as i32),
                -(CHUNK_SIZE as i32),
            ),
            expected_block_id: Some(block::ID(BLOCK_ID_MAX / 2)),
        },
        GridToBlockIDTestCase {
            grid_position: IVec3::new(
                CHUNK_RADIUS as i32,
                CHUNK_RADIUS as i32,
                CHUNK_RADIUS as i32,
            ),
            expected_block_id: Some(block::ID::MAX),
        },
        GridToBlockIDTestCase {
            grid_position: IVec3::new(
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
            ),
            expected_block_id: Some(block::ID(0)),
        },
        GridToBlockIDTestCase {
            grid_position: IVec3::new(
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
            ),
            expected_block_id: Some(block::ID::MAX),
        },
        GridToBlockIDTestCase {
            grid_position: IVec3::new(
                -(CHUNK_SIZE as i32) - (CHUNK_RADIUS as i32),
                -(CHUNK_SIZE as i32) - (CHUNK_RADIUS as i32),
                -(CHUNK_SIZE as i32) - (CHUNK_RADIUS as i32),
            ),
            expected_block_id: Some(block::ID(0)),
        },
    ];

    for test_case in test_cases {
        let block_id = grid::grid_to_block_id(test_case.grid_position);

        assert_eq!(
            block_id, test_case.expected_block_id,
            "Failed at {:?}",
            test_case.grid_position
        );
    }
}

struct BlockIDToPositionTestCase {
    block_id: block::ID,
    expected_block_position: Option<IVec3>,
}

#[test]
fn block_id_to_position() {
    let test_cases = vec![
        BlockIDToPositionTestCase {
            block_id: block::ID(BLOCK_ID_MAX / 2),
            expected_block_position: Some(IVec3::new(0, 0, 0)),
        },
        BlockIDToPositionTestCase {
            block_id: block::ID(BLOCK_ID_MAX + 1),
            expected_block_position: None,
        },
        BlockIDToPositionTestCase {
            block_id: block::ID(0),
            expected_block_position: Some(IVec3::new(
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
            )),
        },
        BlockIDToPositionTestCase {
            block_id: block::ID::MAX,
            expected_block_position: Some(IVec3::new(
                CHUNK_RADIUS as i32,
                CHUNK_RADIUS as i32,
                CHUNK_RADIUS as i32,
            )),
        },
    ];

    for test_case in test_cases {
        let block_position = grid::block_id_to_position(test_case.block_id);

        assert_eq!(
            block_position, test_case.expected_block_position,
            "Failed at {:?}",
            test_case.block_id
        );
    }
}

struct GridToIDsTestCase {
    grid_position: IVec3,
    expected_chunk_id: chunk::ID,
    expected_block_id: block::ID,
}

#[test]
fn grid_to_ids() {
    let test_cases = vec![
        GridToIDsTestCase {
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_id: chunk::ID(CHUNK_ID_MAX / 2),
            expected_block_id: block::ID(BLOCK_ID_MAX / 2),
        },
        GridToIDsTestCase {
            grid_position: IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            ),
            expected_chunk_id: chunk::ID(WORLD_VOLUME - 1),
            expected_block_id: block::ID::MAX,
        },
        GridToIDsTestCase {
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            ),
            expected_chunk_id: chunk::ID(0),
            expected_block_id: block::ID(0),
        },
        GridToIDsTestCase {
            grid_position: IVec3::new(
                -(CHUNK_SIZE as i32) - CHUNK_RADIUS as i32,
                -(CHUNK_SIZE as i32) - CHUNK_RADIUS as i32,
                -(CHUNK_SIZE as i32) - CHUNK_RADIUS as i32,
            ),
            expected_chunk_id: {
                let chunk_position = IVec3::new(-1, -1, -1);

                let chunk_index = (chunk_position.x + WORLD_RADIUS as i32)
                    + (chunk_position.y + WORLD_RADIUS as i32) * WORLD_SIZE as i32
                    + (chunk_position.z + WORLD_RADIUS as i32)
                        * WORLD_SIZE as i32
                        * WORLD_SIZE as i32;

                chunk::ID(chunk_index as usize)
            },
            expected_block_id: block::ID(0),
        },
        GridToIDsTestCase {
            grid_position: IVec3::new(
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
            ),
            expected_chunk_id: {
                let chunk_position = IVec3::new(1, 1, 1);

                let chunk_index = (chunk_position.x + WORLD_RADIUS as i32)
                    + (chunk_position.y + WORLD_RADIUS as i32) * WORLD_SIZE as i32
                    + (chunk_position.z + WORLD_RADIUS as i32)
                        * WORLD_SIZE as i32
                        * WORLD_SIZE as i32;

                chunk::ID(chunk_index as usize)
            },
            expected_block_id: block::ID(BLOCK_ID_MAX),
        },
    ];

    for test_case in test_cases {
        let id_result = grid::grid_to_ids(test_case.grid_position);

        assert!(
            id_result.is_some(),
            "Failed at {:?}",
            test_case.grid_position
        );

        let (chunk_id, block_id) = id_result.unwrap();

        assert_eq!(
            chunk_id, test_case.expected_chunk_id,
            "Failed at chunk_id for {:?}",
            test_case.grid_position
        );

        assert_eq!(
            block_id, test_case.expected_block_id,
            "Failed at block_id for {:?}",
            test_case.grid_position
        );
    }
}

struct IDsToGridTestCase {
    chunk_id: chunk::ID,
    block_id: block::ID,
    expected_grid_position: Option<IVec3>,
}

#[test]
fn ids_to_grid() {
    let test_cases = vec![
        IDsToGridTestCase {
            chunk_id: chunk::ID(CHUNK_ID_MAX / 2),
            block_id: block::ID(BLOCK_ID_MAX / 2),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        IDsToGridTestCase {
            chunk_id: chunk::ID(WORLD_VOLUME - 1),
            block_id: block::ID::MAX,
            expected_grid_position: Some(IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            )),
        },
        IDsToGridTestCase {
            chunk_id: chunk::ID(0),
            block_id: block::ID(0),
            expected_grid_position: Some(IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            )),
        },
        IDsToGridTestCase {
            chunk_id: {
                let chunk_position = IVec3::new(-1, -1, -1);
                let chunk_index = (chunk_position.x + WORLD_RADIUS as i32)
                    + (chunk_position.y + WORLD_RADIUS as i32) * WORLD_SIZE as i32
                    + (chunk_position.z + WORLD_RADIUS as i32)
                        * WORLD_SIZE as i32
                        * WORLD_SIZE as i32;
                chunk::ID(chunk_index as usize)
            },
            block_id: block::ID(0),
            expected_grid_position: Some(IVec3::new(
                -(CHUNK_SIZE as i32) - CHUNK_RADIUS as i32,
                -(CHUNK_SIZE as i32) - CHUNK_RADIUS as i32,
                -(CHUNK_SIZE as i32) - CHUNK_RADIUS as i32,
            )),
        },
        IDsToGridTestCase {
            chunk_id: {
                let chunk_position = IVec3::new(1, 1, 1);
                let chunk_index = (chunk_position.x + WORLD_RADIUS as i32)
                    + (chunk_position.y + WORLD_RADIUS as i32) * WORLD_SIZE as i32
                    + (chunk_position.z + WORLD_RADIUS as i32)
                        * WORLD_SIZE as i32
                        * WORLD_SIZE as i32;
                chunk::ID(chunk_index as usize)
            },
            block_id: block::ID(BLOCK_ID_MAX),
            expected_grid_position: Some(IVec3::new(
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
            )),
        },
    ];

    for test_case in test_cases {
        let grid_position = grid::ids_to_grid(test_case.chunk_id, test_case.block_id);

        assert_eq!(
            grid_position, test_case.expected_grid_position,
            "Failed at chunk_id: {:?}, block_id: {:?}",
            test_case.chunk_id, test_case.block_id
        );
    }
}

#[test]
fn grid_to_block_fuzz() {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    for _ in 0..10_000 {
        let x = rng.gen_range(-100..100);
        let y = rng.gen_range(-100..100);
        let z = rng.gen_range(-100..100);

        let grid_position = IVec3::new(x, y, z);

        let _ = grid::grid_to_block_id(grid_position);
    }
}
