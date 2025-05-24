use glam::{IVec3, Vec3};
use last_ditch::simulation::{world::grid, CHUNK_RADIUS, CHUNK_SIZE, GRID_BOUNDARY, WORLD_RADIUS};

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
