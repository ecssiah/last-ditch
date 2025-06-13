use crate::simulation::{
    consts::*,
    world::{builder, World},
};
use glam::IVec3;
use std::f32::EPSILON;

struct NodeCountCase {
    description: String,
    chunk_coordinates: IVec3,
    expected_node_count: usize,
}

impl NodeCountCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world
            .grid
            .chunk_coordinates_to_chunk_id(self.chunk_coordinates)
            .expect("invalid chunk coordinates");

        let chunk_graph = world
            .graph
            .get_chunk_graph(chunk_id)
            .expect("invalid chunk_id");

        let node_count = chunk_graph.node_map.len();

        assert_eq!(
            node_count, self.expected_node_count,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn node_count() {
    let mut world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        NodeCountCase {
            description: "(0, 0, 0)".to_string(),
            chunk_coordinates: IVec3::new(0, 0, 0),
            expected_node_count: 49,
        },
        NodeCountCase {
            description: "(1, 0, 0)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            expected_node_count: 26,
        },
        NodeCountCase {
            description: "(-1, 0, 0)".to_string(),
            chunk_coordinates: IVec3::new(-1, 0, 0),
            expected_node_count: 26,
        },
        NodeCountCase {
            description: "(0, 1, 0)".to_string(),
            chunk_coordinates: IVec3::new(0, 1, 0),
            expected_node_count: 0,
        },
        NodeCountCase {
            description: "(0, -1, 0)".to_string(),
            chunk_coordinates: IVec3::new(0, -1, 0),
            expected_node_count: 0,
        },
        NodeCountCase {
            description: "(0, 0, 1)".to_string(),
            chunk_coordinates: IVec3::new(0, 0, 1),
            expected_node_count: 29,
        },
        NodeCountCase {
            description: "(0, 0, -1)".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -1),
            expected_node_count: 31,
        },
        NodeCountCase {
            description: "(0, 0, -2)".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -2),
            expected_node_count: 31,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct EdgeCountCase {
    description: String,
    chunk_coordinates: IVec3,
    block_coordinates: IVec3,
    expected_edge_count: usize,
}

impl EdgeCountCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world
            .grid
            .chunk_coordinates_to_chunk_id(self.chunk_coordinates)
            .expect("invalid chunk coordinates");

        let block_id = world
            .grid
            .block_coordinates_to_block_id(self.block_coordinates)
            .expect("invalid block coordinates");

        let chunk_graph = world
            .graph
            .get_chunk_graph(chunk_id)
            .expect("invalid chunk id");

        let edge_count = chunk_graph.get_edge_iter(block_id).count();

        assert_eq!(
            edge_count, self.expected_edge_count,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn edge_count() {
    let mut world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        EdgeCountCase {
            description: "Edge Count: (0, -2, 0)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates: IVec3::new(0, -2, 0),
            expected_edge_count: 4,
        },
        EdgeCountCase {
            description: "Edge Count: (0, -1, 0)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates: IVec3::new(0, -1, 0),
            expected_edge_count: 0,
        },
        EdgeCountCase {
            description: "Edge Count: (1, -1, 1)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates: IVec3::new(1, -1, 1),
            expected_edge_count: 4,
        },
        EdgeCountCase {
            description: "Edge Count: (1, -1, 2)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates: IVec3::new(1, -1, 2),
            expected_edge_count: 3,
        },
        EdgeCountCase {
            description: "Edge Count: (2, 1, -2)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates: IVec3::new(2, 0, -2),
            expected_edge_count: 0,
        },
        EdgeCountCase {
            description: "Edge Count: (0, -3, 0)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates: IVec3::new(0, -3, 0),
            expected_edge_count: 0,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct EdgeValidationCase {
    description: String,
    chunk_coordinates: IVec3,
    block_coordinates1: IVec3,
    block_coordinates2: IVec3,
    expected_cost: Option<f32>,
}

impl EdgeValidationCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world
            .grid
            .chunk_coordinates_to_chunk_id(self.chunk_coordinates)
            .expect("invalid chunk coordinates");

        let block_id1 = world
            .grid
            .block_coordinates_to_block_id(self.block_coordinates1)
            .expect("invalid block coordinates");

        let block_id2 = world
            .grid
            .block_coordinates_to_block_id(self.block_coordinates2)
            .expect("invalid block coordinates");

        let chunk_graph = world
            .graph
            .get_chunk_graph(chunk_id)
            .expect("invalid chunk id");

        if self.expected_cost.is_some() {
            assert_eq!(
                chunk_graph.has_edge(block_id1, block_id2),
                true,
                "{:?}",
                self.description
            );

            let expected_cost = self.expected_cost.unwrap();

            let edge = chunk_graph
                .get_edge(block_id1, block_id2)
                .expect("edge does not exist");

            assert!(
                (edge.cost - expected_cost).abs() < EPSILON,
                "{:?}",
                self.description
            );
        } else {
            assert_eq!(
                chunk_graph.has_edge(block_id1, block_id2),
                false,
                "{:?}",
                self.description
            );
        }
    }
}

#[test]
fn edge_validation() {
    let mut world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        EdgeValidationCase {
            description: "Edge: (0, -2, 0) - (1, -2, 0)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates1: IVec3::new(0, -2, 0),
            block_coordinates2: IVec3::new(1, -2, 0),
            expected_cost: Some(WORLD_FACE_COST),
        },
        EdgeValidationCase {
            description: "Edge: (1, -2, 0) - (0, -2, 0)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates1: IVec3::new(1, -2, 0),
            block_coordinates2: IVec3::new(0, -2, 0),
            expected_cost: Some(WORLD_FACE_COST),
        },
        EdgeValidationCase {
            description: "Edge: (1, -1, 1) - (2, 0, 2)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates1: IVec3::new(2, 0, 2),
            block_coordinates2: IVec3::new(1, -1, 1),
            expected_cost: None,
        },
        EdgeValidationCase {
            description: "Edge: (1, -1, 2) - (2, 0, 2)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates1: IVec3::new(2, 0, 2),
            block_coordinates2: IVec3::new(1, -1, 2),
            expected_cost: Some(WORLD_EDGE_COST),
        },
        EdgeValidationCase {
            description: "Edge: (1, 0, 1) - (2, -1, 2)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            block_coordinates1: IVec3::new(1, 0, 0),
            block_coordinates2: IVec3::new(2, -1, 2),
            expected_cost: None,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
