use crate::simulation::{
    consts::*,
    world::{builder, World},
};
use glam::IVec3;

struct NodeCountCase {
    description: String,
    world: World,
    expected_node_count: usize,
}

impl NodeCountCase {
    pub fn check(&self) {
        assert_eq!(
            self.world.graph.node_map.len(),
            self.expected_node_count,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn node_count_validation() {
    let test_world1 = World::new(1, 1);
    let expected_count1 = test_world1.grid.volume as usize;

    let test_world2 = World::new(2, 1);
    let expected_count2 = test_world2.grid.volume as usize;

    let test_world3 = World::new(1, 2);
    let expected_count3 = test_world3.grid.volume as usize;

    let test_world4 = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);
    let expected_count4 = test_world4.grid.volume as usize;

    let test_cases = vec![
        NodeCountCase {
            description: "test world 1".to_string(),
            world: test_world1,
            expected_node_count: expected_count1,
        },
        NodeCountCase {
            description: "test world 2".to_string(),
            world: test_world2,
            expected_node_count: expected_count2,
        },
        NodeCountCase {
            description: "test world 3".to_string(),
            world: test_world3,
            expected_node_count: expected_count3,
        },
        NodeCountCase {
            description: "test world 4".to_string(),
            world: test_world4,
            expected_node_count: expected_count4,
        },
    ];

    for test_case in test_cases {
        test_case.check();
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
            node.edge_map.len(),
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
            description: "Chunk: (0, 0, 0)".to_string(),
            chunk_position: IVec3::new(0, 0, 0),
            expected_edge_count: 4,
        },
        EdgeCountValidationCase {
            description: "Chunk: (0, 0, -1)".to_string(),
            chunk_position: IVec3::new(0, 0, -1),
            expected_edge_count: 5,
        },
        EdgeCountValidationCase {
            description: "Chunk: (0, 0, -2)".to_string(),
            chunk_position: IVec3::new(0, 0, -2),
            expected_edge_count: 4,
        },
        EdgeCountValidationCase {
            description: "Chunk: (1, 0, 0)".to_string(),
            chunk_position: IVec3::new(1, 0, 0),
            expected_edge_count: 1,
        },
        EdgeCountValidationCase {
            description: "Chunk: (-1, 0, 0)".to_string(),
            chunk_position: IVec3::new(-1, 0, 0),
            expected_edge_count: 1,
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}

struct EdgeValidationCase {
    pub description: String,
    pub from_chunk_position: IVec3,
    pub to_chunk_position: IVec3,
    pub from_position: IVec3,
    pub to_position: IVec3,
    pub expected_cost: Option<f32>,
}

impl EdgeValidationCase {
    pub fn check(&self, world: &World) {
        let node = world.graph.get_node(self.from_chunk_position).unwrap();

        let edge = node.get_edge(self.from_position, self.to_position);

        if self.expected_cost.is_some() {
            assert!(edge.is_some(), "{:?}", self.description);

            let edge = edge.unwrap();
            let expected_cost = self.expected_cost.unwrap();

            assert_eq!(
                edge.to_chunk_position, self.to_chunk_position,
                "{:?}",
                self.description
            );
            assert_eq!(edge.cost, expected_cost, "{:?}", self.description);
        } else {
            assert!(edge.is_none(), "{:?}", self.description);
        }
    }
}

#[test]
fn edge_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let test_cases = vec![
        EdgeValidationCase {
            description: "(0, -3, -10) to (0, -3, -11)".to_string(),
            from_chunk_position: IVec3::new(0, 0, -1),
            to_chunk_position: IVec3::new(0, 0, -2),
            from_position: IVec3::new(0, -3, -10),
            to_position: IVec3::new(0, -3, -11),
            expected_cost: Some(WORLD_FACE_COST),
        },
        EdgeValidationCase {
            description: "(0, -3, -11) to (0, -3, -10)".to_string(),
            from_chunk_position: IVec3::new(0, 0, -2),
            to_chunk_position: IVec3::new(0, 0, -1),
            from_position: IVec3::new(0, -3, -11),
            to_position: IVec3::new(0, -3, -10),
            expected_cost: Some(WORLD_FACE_COST),
        },
        EdgeValidationCase {
            description: "(0, -3, -10) to (1, -3, -11)".to_string(),
            from_chunk_position: IVec3::new(0, 0, -1),
            to_chunk_position: IVec3::new(0, 0, -2),
            from_position: IVec3::new(0, -3, -10),
            to_position: IVec3::new(1, -3, -11),
            expected_cost: None,
        },
        EdgeValidationCase {
            description: "(1, -3, -11) to (0, -3, -10)".to_string(),
            from_chunk_position: IVec3::new(0, 0, -2),
            to_chunk_position: IVec3::new(0, 0, -1),
            from_position: IVec3::new(1, -3, -11),
            to_position: IVec3::new(0, -3, -10),
            expected_cost: None,
        },
        EdgeValidationCase {
            description: "(2, -2, -10) to (2, -2, -11)".to_string(),
            from_chunk_position: IVec3::new(0, 0, -1),
            to_chunk_position: IVec3::new(0, 0, -2),
            from_position: IVec3::new(2, -2, -10),
            to_position: IVec3::new(2, -2, -11),
            expected_cost: Some(WORLD_FACE_COST),
        },
        EdgeValidationCase {
            description: "(2, -2, -11) to (2, -2, -10)".to_string(),
            from_chunk_position: IVec3::new(0, 0, -2),
            to_chunk_position: IVec3::new(0, 0, -1),
            from_position: IVec3::new(2, -2, -11),
            to_position: IVec3::new(2, -2, -10),
            expected_cost: Some(WORLD_FACE_COST),
        },
        EdgeValidationCase {
            description: "(-1, -3, -10) to (-1, -2, -11)".to_string(),
            from_chunk_position: IVec3::new(0, 0, -1),
            to_chunk_position: IVec3::new(0, 0, -2),
            from_position: IVec3::new(-1, -3, -10),
            to_position: IVec3::new(-1, -2, -11),
            expected_cost: Some(WORLD_EDGE_COST),
        },
        EdgeValidationCase {
            description: "(-1, -2, -11) to (-1, -3, -10)".to_string(),
            from_chunk_position: IVec3::new(0, 0, -2),
            to_chunk_position: IVec3::new(0, 0, -1),
            from_position: IVec3::new(-1, -2, -11),
            to_position: IVec3::new(-1, -3, -10),
            expected_cost: Some(WORLD_EDGE_COST),
        },
    ];

    for test_case in test_cases {
        test_case.check(&test_world);
    }
}
