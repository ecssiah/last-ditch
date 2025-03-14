use glam::IVec3;
use last_ditch::simulation::*;

#[test]
fn test_grid_position_to_chunk_id() {
    let test_cases = vec![
        (
            IVec3::new(
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            Some(0),
        ),
        (IVec3::new(0, 0, 0), Some((WORLD_VOLUME - 1) / 2)),
        (
            IVec3::new(
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
            ),
            Some(WORLD_VOLUME - 1),
        ),
        (
            IVec3::new(
                WORLD_BOUNDARY as i32,
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            Some(WORLD_SIZE - 1),
        ),
    ];

    for (grid_position, expected_chunk_id) in test_cases {
        let chunk_id = Simulation::grid_position_to_chunk_id(grid_position);

        assert_eq!(
            chunk_id, expected_chunk_id,
            "Failed at position {:?}",
            grid_position
        );
    }
}

#[test]
fn test_grid_position_to_block_id() {
    let test_cases = vec![
        (
            IVec3::new(
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            Some(0),
        ),
        (IVec3::new(0, 0, 0), Some((CHUNK_VOLUME - 1) / 2)),
        (
            IVec3::new(
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
                WORLD_BOUNDARY as i32,
            ),
            Some(CHUNK_VOLUME - 1),
        ),
        (
            IVec3::new(
                WORLD_BOUNDARY as i32,
                -(WORLD_BOUNDARY as i32),
                -(WORLD_BOUNDARY as i32),
            ),
            Some(CHUNK_SIZE - 1),
        ),
    ];

    for (grid_position, expected_block_id) in test_cases {
        let block_id = Simulation::grid_position_to_block_id(grid_position);

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
        let grid_position = Simulation::get_grid_position(chunk_id, block_id);

        assert_eq!(
            grid_position, expected_grid_position,
            "Failed at chunk_id: {:?}, block_id: {:?}",
            chunk_id, block_id,
        );
    }
}
