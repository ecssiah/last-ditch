use glam::IVec3;
use last_ditch::simulation::{world::grid, GRID_BOUNDARY};

struct IsValidTestCase {
    description: String,
    grid_position: IVec3,
    expected_is_valid: bool,
}

#[test]
fn origin() {
    let test_cases = vec![IsValidTestCase {
        description: String::from("Grid Position: (0, 0, 0)"),
        grid_position: IVec3::new(0, 0, 0),
        expected_is_valid: true,
    }];

    for test_case in test_cases {
        let is_valid = grid::is_valid(test_case.grid_position);

        assert_eq!(
            is_valid, test_case.expected_is_valid,
            "{:?}",
            test_case.description
        );
    }
}

#[test]
fn boundaries() {
    struct IsValidTestCase {
        description: String,
        grid_position: IVec3,
        expected_is_valid: bool,
    }

    let test_cases = vec![
        IsValidTestCase {
            description: String::from(
                "Grid Position: (GRID_BOUNDARY, GRID_BOUNDARY, GRID_BOUNDARY)",
            ),
            grid_position: IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            ),
            expected_is_valid: true,
        },
        IsValidTestCase {
            description: String::from(
                "Grid Position: (-GRID_BOUNDARY, -GRID_BOUNDARY, -GRID_BOUNDARY)",
            ),
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            ),
            expected_is_valid: true,
        },
        IsValidTestCase {
            description: String::from(
                "Grid Position: (GRID_BOUNDARY + 1, GRID_BOUNDARY + 1, GRID_BOUNDARY + 1)",
            ),
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_is_valid: false,
        },
        IsValidTestCase {
            description: String::from(
                "Grid Position: (-GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1, -GRID_BOUNDARY - 1)",
            ),
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
                -(GRID_BOUNDARY as i32) - 1,
            ),
            expected_is_valid: false,
        },
    ];

    for test_case in test_cases {
        let is_valid = grid::is_valid(test_case.grid_position);

        assert_eq!(
            is_valid, test_case.expected_is_valid,
            "{:?}",
            test_case.description
        );
    }
}
