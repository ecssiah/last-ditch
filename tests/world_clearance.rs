use glam::IVec3;
use last_ditch::simulation::{
    world::{block, World},
    MAXIMUM_CLEARANCE_CHECK,
};

struct HasClearanceTestCase {
    description: String,
    grid_position: IVec3,
    height: i32,
    expected_has_clearance: bool,
}

#[test]
fn has_clearance() {
    let test_world = setup_test_world();

    let test_cases = vec![
        HasClearanceTestCase {
            description: String::from("height 1 at (0, 0, 0)"),
            grid_position: IVec3::new(0, 0, 0),
            height: 1,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("height 2 at (0, 0, 0)"),
            grid_position: IVec3::new(0, 0, 0),
            height: 2,
            expected_has_clearance: false,
        },
        HasClearanceTestCase {
            description: String::from("height 2 at (2, 0, 0)"),
            grid_position: IVec3::new(2, 0, 0),
            height: 2,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("height 3 at (2, 0, 0)"),
            grid_position: IVec3::new(2, 0, 0),
            height: 3,
            expected_has_clearance: false,
        },
        HasClearanceTestCase {
            description: String::from("height 3 at (4, 0, 0)"),
            grid_position: IVec3::new(4, 0, 0),
            height: 3,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("height 4 at (4, 0, 0)"),
            grid_position: IVec3::new(4, 0, 0),
            height: 4,
            expected_has_clearance: false,
        },
        HasClearanceTestCase {
            description: String::from("no clearance at vertical boundary"),
            grid_position: IVec3::new(0, test_world.grid.boundary as i32, 0),
            height: 4,
            expected_has_clearance: false,
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
    let test_world = setup_test_world();

    let test_cases = vec![
        GetClearanceTestCase {
            description: String::from("clearance 1 at (0, 0, 0)"),
            grid_position: IVec3::new(0, 0, 0),
            expected_clearance: 1,
        },
        GetClearanceTestCase {
            description: String::from("clearance 2 at (2, 0, 0)"),
            grid_position: IVec3::new(2, 0, 0),
            expected_clearance: 2,
        },
        GetClearanceTestCase {
            description: String::from("clearance 3 at (4, 0, 0)"),
            grid_position: IVec3::new(4, 0, 0),
            expected_clearance: 3,
        },
        GetClearanceTestCase {
            description: String::from("maximum clearance check at (6, 0, 0)"),
            grid_position: IVec3::new(6, 0, 0),
            expected_clearance: MAXIMUM_CLEARANCE_CHECK,
        },
        GetClearanceTestCase {
            description: String::from("clearance check that passes boundary"),
            grid_position: IVec3::new(2, test_world.grid.boundary as i32 - 1, 0),
            expected_clearance: 1,
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

fn setup_test_world() -> World {
    let mut test_world = World::new(1, 2);

    test_world.set_block_kind(0, 0, 0, block::Kind::Polished1);
    test_world.set_block_kind(0, 2, 0, block::Kind::Polished1);

    test_world.set_block_kind(2, 0, 0, block::Kind::Polished1);
    test_world.set_block_kind(2, 3, 0, block::Kind::Polished1);

    test_world.set_block_kind(4, 0, 0, block::Kind::Polished1);
    test_world.set_block_kind(4, 4, 0, block::Kind::Polished1);

    test_world.set_block_kind(6, 0, 0, block::Kind::Polished1);

    test_world.set_block_kind(
        0,
        test_world.grid.boundary as i32,
        0,
        block::Kind::Polished1,
    );

    test_world.set_block_kind(
        2,
        test_world.grid.boundary as i32 - 1,
        0,
        block::Kind::Polished1,
    );

    test_world.update_chunks();

    test_world
}
