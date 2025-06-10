use crate::simulation::{
    consts::*,
    world::{builder, World},
};
use glam::IVec3;

struct HasClearanceCase {
    description: String,
    chunk_coordinates: IVec3,
    block_coordinates: IVec3,
    height: u32,
    expected_has_clearance: bool,
}

impl HasClearanceCase {
    pub fn check(&self, world: &World) {
        let chunk_position = world
            .grid
            .chunk_coordinates_to_position(self.chunk_coordinates)
            .expect("invalid chunk coordinates");

        let position = chunk_position + self.block_coordinates;

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
    let mut world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        HasClearanceCase {
            description: String::from("clearance 0 at (-2, -2, 2)"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(-2, -2, 2),
            height: 0,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: String::from("clearance 1 at (-1, -2, 2)"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(-1, -2, 2),
            height: 1,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: String::from("not clearance 2 at (-1, -2, 2)"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(-1, -2, 2),
            height: 2,
            expected_has_clearance: false,
        },
        HasClearanceCase {
            description: String::from("clearance 2 at (0, -2, 2)"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(0, -2, 2),
            height: 2,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: String::from("clearance 3 at (1, -2, 2)"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(1, -2, 2),
            height: 3,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: String::from("clearance 4 at (2, -2, 2)"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(2, -2, 2),
            height: 4,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: String::from("not clearance max at (2, -2, 2)"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(2, -2, 2),
            height: MAXIMUM_CLEARANCE,
            expected_has_clearance: false,
        },
        HasClearanceCase {
            description: "clearance max at (0, -2, 0)".to_string(),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(0, -2, 0),
            height: MAXIMUM_CLEARANCE,
            expected_has_clearance: true,
        },
        HasClearanceCase {
            description: "clearance min at (-2, -2, 0)".to_string(),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(0, -2, 0),
            height: MINIMUM_CLEARANCE,
            expected_has_clearance: true,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct GetClearanceCase {
    description: String,
    chunk_coordinates: IVec3,
    block_coordinates: IVec3,
    expected_clearance: u32,
}

impl GetClearanceCase {
    pub fn check(&self, world: &World) {
        let chunk_position = world
            .grid
            .chunk_coordinates_to_position(self.chunk_coordinates)
            .expect("invalid chunk coordinates");

        let position = chunk_position + self.block_coordinates;

        let clearance = world.get_clearance(position);

        assert_eq!(clearance, self.expected_clearance, "{:?}", self.description);
    }
}

#[test]
fn get_clearance() {
    let mut world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        GetClearanceCase {
            description: String::from("clearance 0"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(-2, -2, 2),
            expected_clearance: 0,
        },
        GetClearanceCase {
            description: String::from("clearance 1"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(-1, -2, 2),
            expected_clearance: 1,
        },
        GetClearanceCase {
            description: String::from("clearance 2"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(0, -2, 2),
            expected_clearance: 2,
        },
        GetClearanceCase {
            description: String::from("clearance 3"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(1, -2, 2),
            expected_clearance: 3,
        },
        GetClearanceCase {
            description: String::from("clearance 4"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(2, -2, 2),
            expected_clearance: 4,
        },
        GetClearanceCase {
            description: String::from("clearance max"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(0, -2, 0),
            expected_clearance: MAXIMUM_CLEARANCE,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
