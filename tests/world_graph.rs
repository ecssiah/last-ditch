use glam::IVec3;
use last_ditch::simulation::{
    world::{builder, World},
    TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS,
};

struct NodeCountCase {
    description: String,
    expected_node_count: usize,
}

impl NodeCountCase {
    pub fn check(&self, world: &World) {
        assert_eq!(
            world.graph.node_map.len(),
            self.expected_node_count,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn node_count_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![NodeCountCase {
        description: "Completed World Count".to_string(),
        expected_node_count: test_world.grid.volume as usize,
    }];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct EdgeCountValidationCase {
    description: String,
    chunk_position: IVec3,
    expected_edge_count: usize,
}

impl EdgeCountValidationCase {
    pub fn check(&self, world: &World) {
        let node = world.graph.get_node(self.chunk_position).unwrap();

        assert_eq!(
            node.edge_list.len(),
            self.expected_edge_count,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn edge_count_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![
        EdgeCountValidationCase {
            description: "".to_string(),
            chunk_position: IVec3::new(0, 0, -1),
            expected_edge_count: 5,
        },
        EdgeCountValidationCase {
            description: "".to_string(),
            chunk_position: IVec3::new(0, 0, -2),
            expected_edge_count: 4,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
