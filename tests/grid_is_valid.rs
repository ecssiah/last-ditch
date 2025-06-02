use glam::IVec3;
use last_ditch::simulation::{world::World, TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS};

struct IsValidTestCase {
    description: String,
    grid_position: IVec3,
    expected_is_valid: bool,
}

#[test]
fn origin() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let test_cases = vec![IsValidTestCase {
        description: String::from("Grid Position: (0, 0, 0)"),
        grid_position: IVec3::new(0, 0, 0),
        expected_is_valid: true,
    }];

    for test_case in test_cases {
        let is_valid_grid_position = test_world.grid.is_valid_grid_position(test_case.grid_position);

        assert_eq!(
            is_valid_grid_position, test_case.expected_is_valid,
            "{:?}",
            test_case.description
        );
    }
}

#[test]
fn boundaries() {
    let test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    let boundary = test_world.grid.boundary as i32;

    let test_cases = vec![
        IsValidTestCase {
            description: String::from(
                "Grid Position: (GRID_BOUNDARY, GRID_BOUNDARY, GRID_BOUNDARY)",
            ),
            grid_position: IVec3::new(boundary, boundary, boundary),
            expected_is_valid: true,
        },
        IsValidTestCase {
            description: String::from(
                "Grid Position: (-GRID_BOUNDARY, -GRID_BOUNDARY, -GRID_BOUNDARY)",
            ),
            grid_position: IVec3::new(-boundary, -boundary, -boundary),
            expected_is_valid: true,
        },
        IsValidTestCase {
            description: String::from(
                "Grid Position: (GRID_BOUNDARY + 1, GRID_BOUNDARY + 1, GRID_BOUNDARY + 1)",
            ),
            grid_position: IVec3::new(boundary + 1, boundary + 1, boundary + 1),
            expected_is_valid: false,
        },
        IsValidTestCase {
            description: String::from(
                "Grid Position: (-GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1)",
            ),
            grid_position: IVec3::new(-boundary - 1, -boundary - 1, -boundary - 1),
            expected_is_valid: false,
        },
    ];

    for test_case in test_cases {
        let is_valid_grid_position = test_world.grid.is_valid_grid_position(test_case.grid_position);

        assert_eq!(
            is_valid_grid_position, test_case.expected_is_valid,
            "{:?}",
            test_case.description
        );
    }
}
