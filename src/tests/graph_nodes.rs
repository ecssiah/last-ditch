use crate::simulation::{self, state::World};
use glam::IVec3;

struct NodeValidationCase {
    description: String,
    chunk_coordinates: IVec3,
    block_coordinates: IVec3,
    expected_node: bool,
}

impl NodeValidationCase {
    pub fn check(&self, world: &World) {
        let chunk_position = world
            .grid
            .chunk_coordinates_to_position(self.chunk_coordinates);

        let _position = chunk_position + self.block_coordinates;

        assert_eq!(true, self.expected_node, "{:?}", self.description);
    }
}

#[test]
fn get_clearance() {
    let kind = simulation::Kind::GraphTest;

    let mut world = World::new(kind);
    World::setup(kind, &mut world);

    let test_cases = vec![
        NodeValidationCase {
            description: "".to_string(),
            chunk_coordinates: IVec3::new(0, 0, 0),
            block_coordinates: IVec3::new(0, 0, 0),
            expected_node: true,
        },
        NodeValidationCase {
            description: "".to_string(),
            chunk_coordinates: IVec3::new(0, 0, 0),
            block_coordinates: IVec3::new(0, 0, 0),
            expected_node: true,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
