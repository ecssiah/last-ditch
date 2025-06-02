use glam::IVec3;
use last_ditch::simulation::{
    world::{builder, World},
    TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS,
};

struct NodeCountTestCase {
    description: String,
    chunk_position: IVec3,
    expected_node_count: usize,
}

impl NodeCountTestCase {
    pub fn check(&self, world: &World) {
        let grid_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let chunk = world.get_chunk_at(grid_position).unwrap();

        let node_count = chunk.graph.node_list.len();

        assert_eq!(
            node_count, self.expected_node_count,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn node_count_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![
        NodeCountTestCase {
            description: "(0, 0, 0)".to_string(),
            chunk_position: IVec3::new(0, 0, 0),
            expected_node_count: 49,
        },
        NodeCountTestCase {
            description: "(1, 0, 0)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            expected_node_count: 26,
        },
        NodeCountTestCase {
            description: "(-1, 0, 0)".to_string(),
            chunk_position: IVec3::new(-1, 0, 0),
            expected_node_count: 26,
        },
        NodeCountTestCase {
            description: "(0, 1, 0)".to_string(),
            chunk_position: IVec3::new(0, 1, 0),
            expected_node_count: 0,
        },
        NodeCountTestCase {
            description: "(0, -1, 0)".to_string(),
            chunk_position: IVec3::new(0, -1, 0),
            expected_node_count: 0,
        },
        NodeCountTestCase {
            description: "(0, 0, 1)".to_string(),
            chunk_position: IVec3::new(0, 0, 1),
            expected_node_count: 27,
        },
        NodeCountTestCase {
            description: "(0, 0, -1)".to_string(),
            chunk_position: IVec3::new(0, 0, -1),
            expected_node_count: 31,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct EdgeCountValidationTestCase {
    description: String,
    chunk_position: IVec3,
    block_position: IVec3,
    expected_edge_count: usize,
}

impl EdgeCountValidationTestCase {
    pub fn check(&self, world: &World) {
        let grid_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let chunk = world.get_chunk_at(grid_position).unwrap();

        let node_index = chunk
            .graph
            .get_node_index(grid_position + self.block_position)
            .unwrap();

        let node = &chunk.graph.node_list[node_index];
        let edge_count = node.edge_list.len();

        assert_eq!(
            edge_count, self.expected_edge_count,
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
        EdgeCountValidationTestCase {
            description: "".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position: IVec3::new(-1, -3, 0),
            expected_edge_count: 6,
        },
        EdgeCountValidationTestCase {
            description: "".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position: IVec3::new(1, -3, -1),
            expected_edge_count: 2,
        },
        EdgeCountValidationTestCase {
            description: "".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position: IVec3::new(1, -3, 1),
            expected_edge_count: 4,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct EdgeValidationTestCase {
    description: String,
    chunk_position: IVec3,
    block_position1: IVec3,
    block_position2: IVec3,
    expected_is_some_edge: bool,
}

impl EdgeValidationTestCase {
    pub fn check(&self, world: &World) {
        let grid_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let chunk = world.get_chunk_at(grid_position).unwrap();

        let node1_grid_position = grid_position + self.block_position1;
        let node2_grid_position = grid_position + self.block_position2;

        let node_index1 = chunk.graph.get_node_index(node1_grid_position).unwrap();
        let node_index2 = chunk.graph.get_node_index(node2_grid_position).unwrap();

        let edge12 = chunk.graph.node_list[node_index1]
            .edge_list
            .iter()
            .find(|edge| edge.target == node_index2);

        assert_eq!(
            edge12.is_some(),
            self.expected_is_some_edge,
            "{:?}",
            self.description
        );

        let edge21 = chunk.graph.node_list[node_index2]
            .edge_list
            .iter()
            .find(|edge| edge.target == node_index1);

        assert_eq!(
            edge21.is_some(),
            self.expected_is_some_edge,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn edge_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![
        EdgeValidationTestCase {
            description: "Edge: (0, -2, 0) - (-1, -3, 0)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position1: IVec3::new(0, -2, 0),
            block_position2: IVec3::new(-1, -3, 0),
            expected_is_some_edge: true,
        },
        EdgeValidationTestCase {
            description: "Edge: (0, -2, 0) - (1, -3, -1)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position1: IVec3::new(0, -2, 0),
            block_position2: IVec3::new(1, -3, -1),
            expected_is_some_edge: false,
        },
        EdgeValidationTestCase {
            description: "Edge: (1, -3, 1) - (2, -2, 2)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position1: IVec3::new(1, -3, 1),
            block_position2: IVec3::new(2, -2, 2),
            expected_is_some_edge: true,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
