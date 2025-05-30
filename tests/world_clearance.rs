use glam::IVec3;
use last_ditch::simulation::{
    world::World, MAXIMUM_CLEARANCE_CHECK, TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS,
};

struct HasClearanceTestCase {
    description: String,
    grid_position: IVec3,
    height: i32,
    expected_has_clearance: bool,
}

#[test]
fn has_clearance() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);
    test_world.setup_test_world();

    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_north_grid_position = test_world.grid.chunk_to_grid(IVec3::new(0, 0, 1)).unwrap();

    let test_cases = vec![
        HasClearanceTestCase {
            description: String::from("clearance max"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z,
            ),
            height: MAXIMUM_CLEARANCE_CHECK,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("empty block has no clearance"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x,
                chunk_north_grid_position.y,
                chunk_north_grid_position.z,
            ),
            height: 1,
            expected_has_clearance: false,
        },
        HasClearanceTestCase {
            description: String::from("clearance 0"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x - 2,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            height: 0,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("not clearance 1"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x - 2,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            height: 1,
            expected_has_clearance: false,
        },
        HasClearanceTestCase {
            description: String::from("clearance 1"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x - 1,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            height: 1,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("clearance 2"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            height: 2,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("clearance 3"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x + 1,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            height: 3,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("clearance max"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x + 2,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            height: 4,
            expected_has_clearance: true,
        },
    ];

    for test_case in test_cases {
        let has_clearance = test_world.has_clearance(test_case.grid_position, test_case.height);

        assert_eq!(
            has_clearance, test_case.expected_has_clearance,
            "{:?}",
            test_case.description
        );
    }
}

struct GetClearanceTestCase {
    description: String,
    grid_position: IVec3,
    expected_clearance: i32,
}

#[test]
fn get_clearance() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);
    test_world.setup_test_world();

    let chunk_radius = test_world.grid.chunk_radius as i32;
    let chunk_north_grid_position = test_world.grid.chunk_to_grid(IVec3::new(0, 0, 1)).unwrap();

    let test_cases = vec![
        GetClearanceTestCase {
            description: String::from("clearance 1"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x - 2,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            expected_clearance: 0,
        },
        GetClearanceTestCase {
            description: String::from("clearance 2"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x - 1,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            expected_clearance: 1,
        },
        GetClearanceTestCase {
            description: String::from("clearance 3"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            expected_clearance: 2,
        },
        GetClearanceTestCase {
            description: String::from("clearance 4"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x + 1,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            expected_clearance: 3,
        },
        GetClearanceTestCase {
            description: String::from("clearance max"),
            grid_position: IVec3::new(
                chunk_north_grid_position.x + 2,
                chunk_north_grid_position.y - chunk_radius,
                chunk_north_grid_position.z + 2,
            ),
            expected_clearance: 4,
        },
    ];

    for test_case in test_cases {
        let clearance = test_world.get_clearance(test_case.grid_position) as i32;

        assert_eq!(
            clearance, test_case.expected_clearance,
            "{:?}",
            test_case.description
        );
    }
}
