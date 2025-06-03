use last_ditch::simulation::{
    world::{builder, World},
    TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS,
};

struct NodeCountTestCase {
    description: String,
    expected_node_count: usize,
}

impl NodeCountTestCase {
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

    let test_cases = vec![NodeCountTestCase {
        description: "Completed World Count".to_string(),
        expected_node_count: test_world.grid.volume as usize,
    }];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
