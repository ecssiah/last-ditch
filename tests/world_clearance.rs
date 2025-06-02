use glam::IVec3;
use last_ditch::simulation::{
    consts::*,
    world::{builder, World},
};

struct HasClearanceTestCase {
    description: String,
    chunk_position: IVec3,
    block_position: IVec3,
    height: i32,
    expected_has_clearance: bool,
}

impl HasClearanceTestCase {
    pub fn check(&self, world: &World) {
        let chunk_grid_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let grid_position = chunk_grid_position + self.block_position;

        let has_clearance = world.has_clearance(grid_position, self.height);

        assert_eq!(
            has_clearance, self.expected_has_clearance,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn has_clearance() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![
        HasClearanceTestCase {
            description: String::from("clearance max"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(0, -3, 0),
            height: MAXIMUM_CLEARANCE as i32,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("empty block has no clearance"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(0, 0, 0),
            height: 1,
            expected_has_clearance: false,
        },
        HasClearanceTestCase {
            description: String::from("clearance 0"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(-2, -3, 2),
            height: 0,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("not clearance 1"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(-2, -3, 2),
            height: 1,
            expected_has_clearance: false,
        },
        HasClearanceTestCase {
            description: String::from("clearance 1"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(-1, -3, 2),
            height: 1,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("clearance 2"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(0, -3, 2),
            height: 2,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("clearance 3"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(1, -3, 2),
            height: 3,
            expected_has_clearance: true,
        },
        HasClearanceTestCase {
            description: String::from("clearance max"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(2, -3, 2),
            height: 4,
            expected_has_clearance: true,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GetClearanceTestCase {
    description: String,
    chunk_position: IVec3,
    block_position: IVec3,
    expected_clearance: i32,
}

impl GetClearanceTestCase {
    pub fn check(&self, world: &World) {
        let chunk_grid_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let grid_position = chunk_grid_position + self.block_position;

        let clearance = world.get_clearance(grid_position) as i32;

        assert_eq!(clearance, self.expected_clearance, "{:?}", self.description);
    }
}

#[test]
fn get_clearance() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![
        GetClearanceTestCase {
            description: String::from("clearance 1"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(-2, -3, 2),
            expected_clearance: 0,
        },
        GetClearanceTestCase {
            description: String::from("clearance 2"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(-1, -3, 2),
            expected_clearance: 1,
        },
        GetClearanceTestCase {
            description: String::from("clearance 3"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(0, -3, 2),
            expected_clearance: 2,
        },
        GetClearanceTestCase {
            description: String::from("clearance 4"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(1, -3, 2),
            expected_clearance: 3,
        },
        GetClearanceTestCase {
            description: String::from("clearance max"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(2, -3, 2),
            expected_clearance: 4,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
