use crate::simulation::{self, consts::*, state::world::World};
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
    let mode = simulation::Kind::WorldTest;

    let mut world = World::new(mode);
    world.setup();

    let test_cases = vec![
        GetClearanceCase {
            description: String::from("clearance 0"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(-4, -4, 4),
            expected_clearance: Some(0),
        },
        GetClearanceCase {
            description: String::from("clearance 1"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(-3, -4, 4),
            expected_clearance: Some(1),
        },
        GetClearanceCase {
            description: String::from("clearance 2"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(-2, -4, 4),
            expected_clearance: Some(2),
        },
        GetClearanceCase {
            description: String::from("clearance 3"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(-1, -4, 4),
            expected_clearance: Some(3),
        },
        GetClearanceCase {
            description: String::from("clearance 4"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(0, -4, 4),
            expected_clearance: Some(4),
        },
        GetClearanceCase {
            description: String::from("clearance 5"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(1, -4, 4),
            expected_clearance: Some(5),
        },
        GetClearanceCase {
            description: String::from("clearance 6"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(2, -4, 4),
            expected_clearance: Some(MAXIMUM_CLEARANCE),
        },
        GetClearanceCase {
            description: String::from("clearance 7"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(3, -4, 4),
            expected_clearance: Some(MAXIMUM_CLEARANCE),
        },
        GetClearanceCase {
            description: String::from("clearance 8"),
            chunk_coordinates: IVec3::new(0, 0, 1),
            block_coordinates: IVec3::new(4, -4, 4),
            expected_clearance: Some(MAXIMUM_CLEARANCE),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
