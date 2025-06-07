use crate::simulation::{
    consts::*,
    world::{builder, World},
};
use glam::IVec3;
use std::f32::EPSILON;

struct NodeCountCase {
    description: String,
    chunk_position: IVec3,
    expected_node_count: usize,
}

impl NodeCountCase {
    pub fn check(&self, world: &World) {
        if let Some(chunk_graph) = world.graph.get_chunk_graph(self.chunk_position) {
            let node_count = chunk_graph.node_map.len();

            assert_eq!(
                node_count, self.expected_node_count,
                "{:?}",
                self.description
            );
        } else {
            panic!("{:?}", self.description);
        }
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
        let chunk_position = world.grid.chunk_to_grid(self.chunk_position).unwrap();
        let position = chunk_position + self.block_position;

        if let Some(chunk_graph) = world.graph.get_chunk_graph(self.chunk_position) {
            let node = chunk_graph.get_node(position).unwrap();
            let edge_count = node.edge_list.len();

            assert_eq!(
                edge_count, self.expected_edge_count,
                "{:?}",
                self.description
            );
        } else {
            panic!("{:?}", self.description);
        }
    }
}

#[test]
fn edge_count_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![EdgeCountValidationCase {
        description: "Edge Count: (0, -1, 0)".to_string(),
        chunk_position: IVec3::new(1, 0, 0),
        block_position: IVec3::new(0, -1, 0),
        expected_edge_count: 4,
    }];

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
        let position = world.grid.chunk_to_grid(self.chunk_position).unwrap();

        let node1_position = position + self.block_position1;
        let node2_position = position + self.block_position2;

        let chunk_graph = world
            .graph
            .chunk_graph_map
            .get(&self.chunk_position)
            .unwrap();

        let node1 = chunk_graph.get_node(node1_position).unwrap();
        let node2 = chunk_graph.get_node(node2_position).unwrap();

        let edge12 = node1
            .edge_list
            .iter()
            .find(|edge| edge.to_position == node2_position);

        let edge21 = node2
            .edge_list
            .iter()
            .find(|edge| edge.to_position == node1_position);

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
