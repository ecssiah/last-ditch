use glam::IVec3;
use last_ditch::simulation::{
    consts::*,
    world::{block, chunk, grid},
};

#[test]
fn chunk_ids() {
    struct ChunkIDTestCase {
        grid_position: IVec3,
        expected_chunk_id: Option<chunk::ID>,
    }

    let test_cases = vec![
        ChunkIDTestCase {
            grid_position: IVec3::new(
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            expected_chunk_id: Some(chunk::ID(0)),
        },
        ChunkIDTestCase {
            grid_position: IVec3::new(0, 0, 0),
            expected_chunk_id: Some(chunk::ID((WORLD_VOLUME - 1) / 2)),
        },
        ChunkIDTestCase {
            grid_position: IVec3::new(
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
            ),
            expected_chunk_id: Some(chunk::ID(WORLD_VOLUME - 1)),
        },
        ChunkIDTestCase {
            grid_position: IVec3::new(
                WORLD_BOUNDARY as i32,
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            expected_chunk_id: Some(chunk::ID(WORLD_SIZE - 1)),
        },
    ];

    for test_case in test_cases {
        let chunk_id = grid::get_chunk_id(test_case.grid_position);

        assert_eq!(
            chunk_id, test_case.expected_chunk_id,
            "Failed at position {:?}",
            test_case.grid_position
        );
    }
}

#[test]
fn block_ids() {
    struct BlockIDTestCase {
        grid_position: IVec3,
        expected_block_id: Option<block::ID>,
    }

    let test_cases = vec![
        BlockIDTestCase {
            grid_position: IVec3::new(
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            expected_block_id: Some(block::ID(0)),
        },
        BlockIDTestCase {
            grid_position: IVec3::new(0, 0, 0),
            expected_block_id: Some(block::ID((CHUNK_VOLUME - 1) / 2)),
        },
        BlockIDTestCase {
            grid_position: IVec3::new(
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
            ),
            expected_block_id: Some(block::ID(CHUNK_VOLUME - 1)),
        },
        BlockIDTestCase {
            grid_position: IVec3::new(
                WORLD_BOUNDARY as i32,
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            expected_block_id: Some(block::ID(CHUNK_SIZE - 1)),
        },
    ];

    for test_case in test_cases {
        let block_id = grid::get_block_id(test_case.grid_position);

        assert_eq!(
            block_id, test_case.expected_block_id,
            "Failed at grid position {:?}",
            test_case.grid_position
        );
    }
}

#[test]
fn grid_positions() {
    struct GridPositionTestCase {
        chunk_id: chunk::ID,
        block_id: block::ID,
        expected_grid_position: IVec3,
    }

    let test_cases = vec![
        GridPositionTestCase {
            chunk_id: chunk::ID(0),
            block_id: block::ID(0),
            expected_grid_position: IVec3::new(
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
        },
        GridPositionTestCase {
            chunk_id: chunk::ID((WORLD_VOLUME - 1) / 2),
            block_id: block::ID((WORLD_VOLUME - 1) / 2),
            expected_grid_position: IVec3::new(0, 0, 0),
        },
        GridPositionTestCase {
            chunk_id: chunk::ID(WORLD_VOLUME - 1),
            block_id: block::ID(WORLD_VOLUME - 1),
            expected_grid_position: IVec3::new(
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
            ),
        },
        GridPositionTestCase {
            chunk_id: chunk::ID(WORLD_SIZE - 1),
            block_id: block::ID(CHUNK_SIZE - 1),
            expected_grid_position: IVec3::new(
                WORLD_BOUNDARY as i32,
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
        },
    ];

    for test_case in test_cases {
        let grid_position =
            grid::get_grid_position(test_case.chunk_id, test_case.block_id).unwrap();

        assert_eq!(
            grid_position, test_case.expected_grid_position,
            "Failed at chunk_id: {:?}, block_id: {:?}",
            test_case.chunk_id, test_case.block_id,
        );
    }
}
