use glam::IVec3;
use last_ditch::simulation::{world::grid, GRID_BOUNDARY};

struct IsValidTestCase {
    grid_position: IVec3,
    expected_is_valid: bool,
}

#[test]
fn origin() {
    let test_cases = vec![IsValidTestCase {
        grid_position: IVec3::new(0, 0, 0),
        expected_is_valid: true,
    }];

    for test_case in test_cases {
        let is_valid = grid::is_valid(test_case.grid_position);

        assert_eq!(
            is_valid, test_case.expected_is_valid,
            "Expected {:?} to be {:?}",
            test_case.grid_position, test_case.expected_is_valid
        );
    }
}

#[test]
fn boundaries() {
    struct IsValidTestCase {
        grid_position: IVec3,
        expected_is_valid: bool,
    }

    let test_cases = vec![
        IsValidTestCase {
            grid_position: IVec3::new(
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
                GRID_BOUNDARY as i32,
            ),
            expected_is_valid: true,
        },
        IsValidTestCase {
            grid_position: IVec3::new(
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
                -(GRID_BOUNDARY as i32),
            ),
            expected_is_valid: true,
        },
        IsValidTestCase {
            grid_position: IVec3::new(
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
                (GRID_BOUNDARY as i32) + 1,
            ),
            expected_is_valid: false,
        },
        IsValidTestCase {
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
            "Expected {:?} to be {:?}",
            test_case.grid_position, test_case.expected_is_valid
        );
    }
}
