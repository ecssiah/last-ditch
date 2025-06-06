use crate::simulation::{
    consts::*,
    world::{builder, World},
};
use glam::IVec3;

struct HasClearanceCase {
    description: String,
    chunk_position: IVec3,
    block_position: IVec3,
    height: i32,
    expected_has_clearance: bool,
}

impl HasClearanceCase {
    pub fn check(&self, world: &World) {
        let chunk_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let position = chunk_position + self.block_position;

        let has_clearance = world.has_clearance(position, self.height);

        assert_eq!(
            has_clearance, self.expected_has_clearance,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn has_clearance() {
    let mut test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![
        HasClearanceCase {
            description: String::from("clearance 0"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(-2, -2, 2),
            height: 0,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: String::from("clearance 1"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(-1, -2, 2),
            height: 1,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: String::from("clearance 2"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(0, -2, 2),
            height: 2,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: String::from("clearance 3"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(1, -2, 2),
            height: 3,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: String::from("clearance max"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(2, -2, 2),
            height: 4,
            expected_has_clearance: true,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct GetClearanceCase {
    description: String,
    chunk_position: IVec3,
    block_position: IVec3,
    expected_clearance: u32,
}

impl GetClearanceCase {
    pub fn check(&self, world: &World) {
        let chunk_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let position = chunk_position + self.block_position;

        let clearance = world.get_clearance(position);

        assert_eq!(clearance, self.expected_clearance, "{:?}", self.description);
    }
}

#[test]
fn get_clearance() {
    let mut test_world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![
        GetClearanceCase {
            description: String::from("clearance 0"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(-2, -2, 2),
            expected_clearance: 0,
        },
        GetClearanceCase {
            description: String::from("clearance 1"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(-1, -2, 2),
            expected_clearance: 1,
        },
        GetClearanceCase {
            description: String::from("clearance 2"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(0, -2, 2),
            expected_clearance: 2,
        },
        GetClearanceCase {
            description: String::from("clearance 3"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(1, -2, 2),
            expected_clearance: 3,
        },
        GetClearanceCase {
            description: String::from("clearance 4"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(2, -2, 2),
            expected_clearance: 4,
        },
        GetClearanceCase {
            description: String::from("clearance max"),
            chunk_position: IVec3::new(0, 0, 1),
            block_position: IVec3::new(0, -2, 0),
            expected_clearance: MAXIMUM_CLEARANCE,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
