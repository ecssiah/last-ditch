use glam::IVec3;
use last_ditch::simulation::{
    consts::*,
    world::{block, chunk, grid},
};

#[test]
fn test_get_chunk_id() {
    let test_cases = vec![
        (
            IVec3::new(
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            Some(chunk::ID(0)),
        ),
        (IVec3::new(0, 0, 0), Some(chunk::ID((WORLD_VOLUME - 1) / 2))),
        (
            IVec3::new(
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
            ),
            Some(chunk::ID(WORLD_VOLUME - 1)),
        ),
        (
            IVec3::new(
                WORLD_BOUNDARY as i32,
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            Some(chunk::ID(WORLD_SIZE - 1)),
        ),
    ];

    for (grid_position, expected_chunk_id) in test_cases {
        let chunk_id = grid::get_chunk_id(grid_position);

        assert_eq!(
            chunk_id, expected_chunk_id,
            "Failed at position {:?}",
            grid_position
        );
    }
}

#[test]
fn test_get_block_id() {
    let test_cases = vec![
        (
            IVec3::new(
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            Some(block::ID(0)),
        ),
        (IVec3::new(0, 0, 0), Some(block::ID((CHUNK_VOLUME - 1) / 2))),
        (
            IVec3::new(
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
            ),
            Some(block::ID(CHUNK_VOLUME - 1)),
        ),
        (
            IVec3::new(
                WORLD_BOUNDARY as i32,
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            Some(block::ID(CHUNK_SIZE - 1)),
        ),
    ];

    for (grid_position, expected_block_id) in test_cases {
        let block_id = grid::get_block_id(grid_position);

        assert_eq!(
            block_id, expected_block_id,
            "Failed at position {:?}",
            grid_position
        );
    }
}

#[test]
fn test_get_grid_position() {
    let test_cases = vec![
        (
            (0, 0),
            IVec3::new(
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
        ),
        (
            ((WORLD_VOLUME - 1) / 2, (WORLD_VOLUME - 1) / 2),
            IVec3::new(0, 0, 0),
        ),
        (
            (WORLD_VOLUME - 1, WORLD_VOLUME - 1),
            IVec3::new(
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
            ),
        ),
        (
            (WORLD_SIZE - 1, CHUNK_SIZE - 1),
            IVec3::new(
                WORLD_BOUNDARY as i32,
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
        ),
    ];

    for ((chunk_id, block_id), expected_grid_position) in test_cases {
        let chunk_id = chunk::ID(chunk_id);
        let block_id = block::ID(block_id);

        let grid_position = grid::get_grid_position(chunk_id, block_id).unwrap();

        assert_eq!(
            grid_position, expected_grid_position,
            "Failed at chunk_id: {:?}, block_id: {:?}",
            chunk_id, block_id,
        );
    }
}
