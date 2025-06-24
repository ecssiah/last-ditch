use crate::simulation::{
    consts::*,
    state::world::{builder, World},
};
use glam::IVec3;

struct GetClearanceCase {
    description: String,
    chunk_coordinates: IVec3,
    block_coordinates: IVec3,
    expected_clearance: Option<u32>,
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
            chunk_coordinates: IVec3::new(0, 0, 3),
            block_coordinates: IVec3::new(-3, -3, 3),
            expected_clearance: Some(0),
        },
        GetClearanceCase {
            description: String::from("clearance 1"),
            chunk_coordinates: IVec3::new(0, 0, 3),
            block_coordinates: IVec3::new(-2, -3, 3),
            expected_clearance: Some(1),
        },
        GetClearanceCase {
            description: String::from("clearance 2"),
            chunk_coordinates: IVec3::new(0, 0, 3),
            block_coordinates: IVec3::new(-1, -3, 3),
            expected_clearance: Some(2),
        },
        GetClearanceCase {
            description: String::from("clearance 3"),
            chunk_coordinates: IVec3::new(0, 0, 3),
            block_coordinates: IVec3::new(0, -3, 3),
            expected_clearance: Some(3),
        },
        GetClearanceCase {
            description: String::from("clearance 4"),
            chunk_coordinates: IVec3::new(0, 0, 3),
            block_coordinates: IVec3::new(1, -3, 3),
            expected_clearance: Some(4),
        },
        GetClearanceCase {
            description: String::from("clearance max"),
            chunk_coordinates: IVec3::new(0, 0, 3),
            block_coordinates: IVec3::new(2, -3, 3),
            expected_clearance: Some(5),
        },
        GetClearanceCase {
            description: String::from("clearance max"),
            chunk_coordinates: IVec3::new(0, 0, 3),
            block_coordinates: IVec3::new(3, -3, 3),
            expected_clearance: Some(MAXIMUM_CLEARANCE),
        },
        GetClearanceCase {
            description: String::from("clearance none"),
            chunk_coordinates: IVec3::new(0, 0, 3),
            block_coordinates: IVec3::new(-3, 0, 3),
            expected_clearance: None,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
