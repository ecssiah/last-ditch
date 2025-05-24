use glam::{IVec3, Vec3};
use last_ditch::simulation::{
    consts::*,
    world::{block, chunk, grid},
};

struct WorldToGridTestCase {
    description: String,
    world_position: Vec3,
    expected_grid_position: Option<IVec3>,
}

#[test]
fn world_to_grid() {
    let test_cases = vec![
        WorldToGridTestCase {
            description: String::from(""),
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        WorldToGridTestCase {
            description: String::from(""),
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
            description: String::from(""),
            world_position: Vec3::new(
                (GRID_BOUNDARY as f32) + 1.0,
                (GRID_BOUNDARY as f32) + 1.0,
                (GRID_BOUNDARY as f32) + 1.0,
            ),
            expected_grid_position: None,
        },
        WorldToGridTestCase {
            description: String::from(""),
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
            description: String::from(""),
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
            "{:?}",
            test_case.description,
        );
    }
}

struct GridToChunkTestCase {
    description: String,
    grid_position: IVec3,
    expected_chunk_position: Option<IVec3>,
}

#[test]
fn grid_to_chunk() {
    let test_cases = vec![
        GridToChunkTestCase {
            description: String::from(""),
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        GridToChunkTestCase {
            description: String::from(""),
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
            description: String::from(""),
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_chunk_position: None,
        },
        GridToChunkTestCase {
            description: String::from(""),
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
            description: String::from(""),
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
            "{:?}",
            test_case.description
        );
    }
}

struct GridToBlockTestCase {
    description: String,
    grid_position: IVec3,
    expected_block_position: Option<IVec3>,
}

#[test]
fn grid_to_block() {
    let test_cases = vec![
        GridToBlockTestCase {
            description: String::from(""),
            grid_position: IVec3::new(0, 0, 0),
            expected_block_position: Some(IVec3::new(0, 0, 0)),
        },
        GridToBlockTestCase {
            description: String::from(""),
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
            description: String::from(""),
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_block_position: None,
        },
        GridToBlockTestCase {
            description: String::from(""),
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
            description: String::from(""),
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
            "{:?}",
            test_case.description
        );
    }
}

struct ChunkToGridTestCase {
    description: String,
    chunk_position: IVec3,
    expected_grid_position: Option<IVec3>,
}

#[test]
fn chunk_to_grid() {
    let test_cases = vec![
        ChunkToGridTestCase {
            description: String::from(""),
            chunk_position: IVec3::new(0, 0, 0),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkToGridTestCase {
            description: String::from(""),
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
            description: String::from(""),
            chunk_position: IVec3::new(
                (WORLD_RADIUS as i32) + 1,
                (WORLD_RADIUS as i32) + 1,
                (WORLD_RADIUS as i32) + 1,
            ),
            expected_grid_position: None,
        },
        ChunkToGridTestCase {
            description: String::from(""),
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
            description: String::from(""),
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
            "{:?}",
            test_case.description
        );
    }
}

struct GridToChunkIDTestCase {
    description: String,
    grid_position: IVec3,
    expected_chunk_id: Option<chunk::ID>,
}

#[test]
fn grid_to_chunk_id() {
    let test_cases = vec![
        GridToChunkIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_id: Some(chunk::ID((WORLD_VOLUME - 1) / 2)),
        },
        GridToChunkIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            ),
            expected_chunk_id: Some(chunk::ID(WORLD_VOLUME - 1)),
        },
        GridToChunkIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_chunk_id: None,
        },
        GridToChunkIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            ),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        GridToChunkIDTestCase {
            description: String::from(""),
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
            "{:?}",
            test_case.description
        );
    }
}

struct WorldToChunkIDTestCase {
    description: String,
    world_position: Vec3,
    expected_chunk_id: Option<chunk::ID>,
}

#[test]
fn world_to_chunk_id() {
    let test_cases = vec![
        WorldToChunkIDTestCase {
            description: String::from(""),
            world_position: Vec3::new(0.0, 0.0, 0.0),
            expected_chunk_id: Some(chunk::ID((WORLD_VOLUME - 1) / 2)),
        },
        WorldToChunkIDTestCase {
            description: String::from(""),
            world_position: Vec3::new(
                GRID_BOUNDARY as f32,
                GRID_BOUNDARY as f32,
                GRID_BOUNDARY as f32,
            ),
            expected_chunk_id: Some(chunk::ID(WORLD_VOLUME - 1)),
        },
        WorldToChunkIDTestCase {
            description: String::from(""),
            world_position: Vec3::new(
                (GRID_BOUNDARY as f32) + 1.0,
                (GRID_BOUNDARY as f32) + 1.0,
                (GRID_BOUNDARY as f32) + 1.0,
            ),
            expected_chunk_id: None,
        },
        WorldToChunkIDTestCase {
            description: String::from(""),
            world_position: Vec3::new(
                -(GRID_BOUNDARY as f32),
                -(GRID_BOUNDARY as f32),
                -(GRID_BOUNDARY as f32),
            ),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        WorldToChunkIDTestCase {
            description: String::from(""),
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
            "{:?}",
            test_case.description
        );
    }
}

struct ChunkIDToPositionTestCase {
    description: String,
    chunk_id: chunk::ID,
    expected_chunk_position: Option<IVec3>,
}

#[test]
fn chunk_id_to_position() {
    let test_cases = vec![
        ChunkIDToPositionTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(0),
            expected_chunk_position: Some(IVec3::new(
                -(WORLD_RADIUS as i32),
                -(WORLD_RADIUS as i32),
                -(WORLD_RADIUS as i32),
            )),
        },
        ChunkIDToPositionTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(CHUNK_ID_MAX / 2),
            expected_chunk_position: Some(IVec3::new(0, 0, 0)),
        },
        ChunkIDToPositionTestCase {
            description: String::from(""),
            chunk_id: chunk::ID::MAX,
            expected_chunk_position: Some(IVec3::new(
                WORLD_RADIUS as i32,
                WORLD_RADIUS as i32,
                WORLD_RADIUS as i32,
            )),
        },
        ChunkIDToPositionTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(WORLD_VOLUME),
            expected_chunk_position: None,
        },
    ];

    for test_case in test_cases {
        let chunk_position = grid::chunk_id_to_position(test_case.chunk_id);

        assert_eq!(
            chunk_position, test_case.expected_chunk_position,
            "{:?}",
            test_case.description
        );
    }
}

struct GridToBlockIDTestCase {
    description: String,
    grid_position: IVec3,
    expected_block_id: Option<block::ID>,
}

#[test]
fn grid_to_block_id() {
    let test_cases = vec![
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(0, 0, 0),
            expected_block_id: Some(block::ID(BLOCK_ID_MAX / 2)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
            ),
            expected_block_id: None,
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_block_id: None,
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(CHUNK_SIZE as i32, CHUNK_SIZE as i32, CHUNK_SIZE as i32),
            expected_block_id: Some(block::ID(BLOCK_ID_MAX / 2)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                -(CHUNK_SIZE as i32),
                -(CHUNK_SIZE as i32),
                -(CHUNK_SIZE as i32),
            ),
            expected_block_id: Some(block::ID(BLOCK_ID_MAX / 2)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                CHUNK_RADIUS as i32,
                CHUNK_RADIUS as i32,
                CHUNK_RADIUS as i32,
            ),
            expected_block_id: Some(block::ID::MAX),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
            ),
            expected_block_id: Some(block::ID(0)),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
                CHUNK_SIZE as i32 + CHUNK_RADIUS as i32,
            ),
            expected_block_id: Some(block::ID::MAX),
        },
        GridToBlockIDTestCase {
            description: String::from(""),
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
            "{:?}",
            test_case.description
        );
    }
}

struct BlockIDToPositionTestCase {
    description: String,
    block_id: block::ID,
    expected_block_position: Option<IVec3>,
}

#[test]
fn block_id_to_position() {
    let test_cases = vec![
        BlockIDToPositionTestCase {
            description: String::from(""),
            block_id: block::ID(BLOCK_ID_MAX / 2),
            expected_block_position: Some(IVec3::new(0, 0, 0)),
        },
        BlockIDToPositionTestCase {
            description: String::from(""),
            block_id: block::ID(BLOCK_ID_MAX + 1),
            expected_block_position: None,
        },
        BlockIDToPositionTestCase {
            description: String::from(""),
            block_id: block::ID(0),
            expected_block_position: Some(IVec3::new(
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
                -(CHUNK_RADIUS as i32),
            )),
        },
        BlockIDToPositionTestCase {
            description: String::from(""),
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
            "{:?}",
            test_case.description
        );
    }
}

struct GridToIDsTestCase {
    description: String,
    grid_position: IVec3,
    expected_chunk_id: chunk::ID,
    expected_block_id: block::ID,
}

#[test]
fn grid_to_ids() {
    let test_cases = vec![
        GridToIDsTestCase {
            description: String::from(""),
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_id: chunk::ID(CHUNK_ID_MAX / 2),
            expected_block_id: block::ID(BLOCK_ID_MAX / 2),
        },
        GridToIDsTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            ),
            expected_chunk_id: chunk::ID(WORLD_VOLUME - 1),
            expected_block_id: block::ID::MAX,
        },
        GridToIDsTestCase {
            description: String::from(""),
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            ),
            expected_chunk_id: chunk::ID(0),
            expected_block_id: block::ID(0),
        },
        GridToIDsTestCase {
            description: String::from(""),
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
            description: String::from(""),
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

        assert!(id_result.is_some(), "{:?}", test_case.description);

        let (chunk_id, block_id) = id_result.unwrap();

        assert_eq!(
            chunk_id, test_case.expected_chunk_id,
            "{:?}",
            test_case.description
        );

        assert_eq!(
            block_id, test_case.expected_block_id,
            "{:?}",
            test_case.description
        );
    }
}

struct IDsToGridTestCase {
    description: String,
    chunk_id: chunk::ID,
    block_id: block::ID,
    expected_grid_position: Option<IVec3>,
}

#[test]
fn ids_to_grid() {
    let test_cases = vec![
        IDsToGridTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(CHUNK_ID_MAX / 2),
            block_id: block::ID(BLOCK_ID_MAX / 2),
            expected_grid_position: Some(IVec3::new(0, 0, 0)),
        },
        IDsToGridTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(WORLD_VOLUME - 1),
            block_id: block::ID::MAX,
            expected_grid_position: Some(IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            )),
        },
        IDsToGridTestCase {
            description: String::from(""),
            chunk_id: chunk::ID(0),
            block_id: block::ID(0),
            expected_grid_position: Some(IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            )),
        },
        IDsToGridTestCase {
            description: String::from(""),
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
            description: String::from(""),
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
            "{:?}",
            test_case.description
        );
    }
}
