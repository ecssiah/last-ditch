use glam::IVec3;
use last_ditch::simulation::{
    consts::*,
    world::{builder, World},
};
use std::f32::EPSILON;

struct NodeCountCase {
    description: String,
    chunk_position: IVec3,
    expected_node_count: usize,
}

impl NodeCountCase {
    pub fn check(&self, world: &World) {
        let grid_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let chunk = world.get_chunk_at(grid_position).unwrap();

        let node_count = chunk.graph.node_map.len();

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
        NodeCountCase {
            description: "(0, 0, 0)".to_string(),
            chunk_position: IVec3::new(0, 0, 0),
            expected_node_count: 49,
        },
        NodeCountCase {
            description: "(1, 0, 0)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            expected_node_count: 26,
        },
        NodeCountCase {
            description: "(-1, 0, 0)".to_string(),
            chunk_position: IVec3::new(-1, 0, 0),
            expected_node_count: 26,
        },
        NodeCountCase {
            description: "(0, 1, 0)".to_string(),
            chunk_position: IVec3::new(0, 1, 0),
            expected_node_count: 0,
        },
        NodeCountCase {
            description: "(0, -1, 0)".to_string(),
            chunk_position: IVec3::new(0, -1, 0),
            expected_node_count: 0,
        },
        NodeCountCase {
            description: "(0, 0, 1)".to_string(),
            chunk_position: IVec3::new(0, 0, 1),
            expected_node_count: 27,
        },
        NodeCountCase {
            description: "(0, 0, -1)".to_string(),
            chunk_position: IVec3::new(0, 0, -1),
            expected_node_count: 31,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct EdgeCountValidationCase {
    description: String,
    chunk_position: IVec3,
    block_position: IVec3,
    expected_edge_count: usize,
}

impl EdgeCountValidationCase {
    pub fn check(&self, world: &World) {
        let chunk_grid_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let chunk = world.get_chunk_at(chunk_grid_position).unwrap();

        let grid_position = chunk_grid_position + self.block_position;

        let node = chunk.graph.get_node(grid_position).unwrap();
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
        EdgeCountValidationCase {
            description: "Edge Count: (-1, -3, 0)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position: IVec3::new(-1, -3, 0),
            expected_edge_count: 4,
        },
        EdgeCountValidationCase {
            description: "Edge Count: (1, -3, -1)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position: IVec3::new(1, -3, -1),
            expected_edge_count: 2,
        },
        EdgeCountValidationCase {
            description: "Edge Count: (1, -3, 1)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position: IVec3::new(1, -3, 1),
            expected_edge_count: 3,
        },
        EdgeCountValidationCase {
            description: "Edge Count: (2, 0, 0)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position: IVec3::new(2, 0, 0),
            expected_edge_count: 2,
        },
        EdgeCountValidationCase {
            description: "Edge Count: (-2, -3, 2)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position: IVec3::new(-2, -3, 2),
            expected_edge_count: 2,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct EdgeValidationCase {
    description: String,
    chunk_position: IVec3,
    block_position1: IVec3,
    block_position2: IVec3,
    expected_cost: Option<f32>,
}

impl EdgeValidationCase {
    pub fn check(&self, world: &World) {
        let grid_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let chunk = world.get_chunk_at(grid_position).unwrap();

        let node1_grid_position = grid_position + self.block_position1;
        let node2_grid_position = grid_position + self.block_position2;

        let edge12 = chunk
            .graph
            .node_map
            .get(&node1_grid_position)
            .unwrap()
            .edge_list
            .iter()
            .find(|edge| edge.target == node2_grid_position);

        let edge21 = chunk
            .graph
            .node_map
            .get(&node2_grid_position)
            .unwrap()
            .edge_list
            .iter()
            .find(|edge| edge.target == node1_grid_position);

        if self.expected_cost.is_some() {
            assert!(edge12.is_some(), "{:?}", self.description);
            assert!(edge21.is_some(), "{:?}", self.description);

            let expected_cost = self.expected_cost.unwrap();

            let edge12_cost = edge12.unwrap().cost;
            let edge21_cost = edge21.unwrap().cost;

            assert!(
                (edge12_cost - expected_cost).abs() < EPSILON,
                "{:?}",
                self.description
            );

            assert!(
                (edge21_cost - expected_cost).abs() < EPSILON,
                "{:?}",
                self.description
            );
        } else {
            assert!(edge12.is_none(), "{:?}", self.description);
            assert!(edge21.is_none(), "{:?}", self.description);
        }
    }
}

#[test]
fn edge_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![
        EdgeValidationCase {
            description: "Edge: (0, -2, 0) - (-1, -3, 0)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position1: IVec3::new(0, -2, 0),
            block_position2: IVec3::new(-1, -3, 0),
            expected_cost: Some(WORLD_EDGE_COST),
        },
        EdgeValidationCase {
            description: "Edge: (-1, -3, 0) - (-2, -3, 0)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position1: IVec3::new(-1, -3, 0),
            block_position2: IVec3::new(-2, -3, 0),
            expected_cost: Some(WORLD_FACE_COST),
        },
        EdgeValidationCase {
            description: "Edge: (0, -2, 0) - (1, -3, -1)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position1: IVec3::new(0, -2, 0),
            block_position2: IVec3::new(1, -3, -1),
            expected_cost: None,
        },
        EdgeValidationCase {
            description: "Edge: (1, -3, 1) - (2, -2, 2)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position1: IVec3::new(1, -3, 1),
            block_position2: IVec3::new(2, -2, 2),
            expected_cost: None,
        },
        EdgeValidationCase {
            description: "Edge: (-2, -3, 0) - (-1, -3, 0)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            block_position1: IVec3::new(-2, -3, 0),
            block_position2: IVec3::new(-1, -3, 0),
            expected_cost: Some(WORLD_FACE_COST),
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
