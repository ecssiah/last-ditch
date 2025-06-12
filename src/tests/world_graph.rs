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
    let world1 = World::new(1, 1);
    let expected_count1 = world1.grid.world_volume as usize;

    let world2 = World::new(1, 2);
    let expected_count2 = world2.grid.world_volume as usize;

    let world3 = World::new(2, 1);
    let expected_count3 = world3.grid.world_volume as usize;

    let world4 = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);
    let expected_count4 = world4.grid.world_volume as usize;

    let test_cases = vec![
        NodeCountCase {
            description: "test world 1".to_string(),
            world: world1,
            expected_node_count: expected_count1,
        },
        NodeCountCase {
            description: "test world 2".to_string(),
            world: world2,
            expected_node_count: expected_count2,
        },
        NodeCountCase {
            description: "test world 3".to_string(),
            world: world3,
            expected_node_count: expected_count3,
        },
        NodeCountCase {
            description: "test world 4".to_string(),
            world: world4,
            expected_node_count: expected_count4,
        },
    ];

    for case in test_cases {
        case.check();
    }
}

struct EdgeCountValidationCase {
    description: String,
    chunk_coordinates: IVec3,
    expected_edge_count: usize,
}

impl EdgeCountValidationCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world
            .grid
            .chunk_coordinates_to_chunk_id(self.chunk_coordinates)
            .expect("invalid chunk coordinates");

        let edge_count = world.graph.get_edge_iter(chunk_id).count();

        assert_eq!(
            edge_count, self.expected_edge_count,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn edge_count_validation() {
    let mut world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        EdgeCountValidationCase {
            description: "Chunk: (0, 0, 0)".to_string(),
            chunk_coordinates: IVec3::new(0, 0, 0),
            expected_edge_count: 4,
        },
        EdgeCountValidationCase {
            description: "Chunk: (0, 0, -1)".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -1),
            expected_edge_count: 5,
        },
        EdgeCountValidationCase {
            description: "Chunk: (0, 0, -2)".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -2),
            expected_edge_count: 3,
        },
        EdgeCountValidationCase {
            description: "Chunk: (1, 0, 0)".to_string(),
            chunk_coordinates: IVec3::new(1, 0, 0),
            expected_edge_count: 1,
        },
        EdgeCountValidationCase {
            description: "Chunk: (-1, 0, 0)".to_string(),
            chunk_coordinates: IVec3::new(-1, 0, 0),
            expected_edge_count: 1,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct EdgeValidationCase {
    pub description: String,
    pub chunk_coordinates1: IVec3,
    pub block_coordinates1: IVec3,
    pub chunk_coordinates2: IVec3,
    pub block_coordinates2: IVec3,
    pub expected_clearance: Option<u32>,
    pub expected_cost: Option<f32>,
}

impl EdgeValidationCase {
    pub fn check(&self, world: &World) {
        let chunk_id1 = world
            .grid
            .chunk_coordinates_to_chunk_id(self.chunk_coordinates1)
            .expect("invalid chunk 1 coordinates");

        let block_id1 = world
            .grid
            .block_coordinates_to_block_id(self.block_coordinates1)
            .expect("invalid block 1 coordinates");

        let chunk_id2 = world
            .grid
            .chunk_coordinates_to_chunk_id(self.chunk_coordinates2)
            .expect("invalid chunk 2 coordinates");

        let block_id2 = world
            .grid
            .block_coordinates_to_block_id(self.block_coordinates2)
            .expect("invalid block 2 coordinates");

        let edge = world
            .graph
            .get_edge(chunk_id1, block_id1, chunk_id2, block_id2);

        if self.expected_cost.is_some() {
            assert!(edge.is_some(), "{:?}", self.description);

            let edge = edge.unwrap();

            let expected_clearance = self.expected_clearance.unwrap();
            let expected_cost = self.expected_cost.unwrap();

            assert_eq!(edge.clearance, expected_clearance, "{:?}", self.description);
            assert_eq!(edge.cost, expected_cost, "{:?}", self.description);
        } else {
            assert!(edge.is_none(), "{:?}", self.description);
        }
    }
}

#[test]
fn edge_validation() {
    let mut world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        EdgeValidationCase {
            description: "case 1".to_string(),
            chunk_coordinates1: IVec3::new(0, 0, -1),
            block_coordinates1: IVec3::new(0, -2, -3),
            chunk_coordinates2: IVec3::new(0, 0, -2),
            block_coordinates2: IVec3::new(0, -2, 3),
            expected_clearance: Some(MAXIMUM_CLEARANCE),
            expected_cost: Some(WORLD_FACE_COST),
        },
        EdgeValidationCase {
            description: "case 2".to_string(),
            chunk_coordinates2: IVec3::new(0, 0, -2),
            block_coordinates2: IVec3::new(0, -2, 3),
            chunk_coordinates1: IVec3::new(0, 0, -1),
            block_coordinates1: IVec3::new(0, -2, -3),
            expected_clearance: Some(MAXIMUM_CLEARANCE),
            expected_cost: Some(WORLD_FACE_COST),
        },
        EdgeValidationCase {
            description: "case 3".to_string(),
            chunk_coordinates1: IVec3::new(0, 0, -1),
            block_coordinates1: IVec3::new(2, -1, -3),
            chunk_coordinates2: IVec3::new(0, 0, -2),
            block_coordinates2: IVec3::new(2, 0, 3),
            expected_clearance: Some(3),
            expected_cost: Some(WORLD_EDGE_COST),
        },
        EdgeValidationCase {
            description: "case 4".to_string(),
            chunk_coordinates1: IVec3::new(0, 0, -1),
            block_coordinates1: IVec3::new(-2, -2, -3),
            chunk_coordinates2: IVec3::new(0, 0, -2),
            block_coordinates2: IVec3::new(-2, -2, 3),
            expected_clearance: None,
            expected_cost: None,
        },
        EdgeValidationCase {
            description: "case 5".to_string(),
            chunk_coordinates1: IVec3::new(0, 0, -1),
            block_coordinates1: IVec3::new(-1, -3, -3),
            chunk_coordinates2: IVec3::new(0, -1, -2),
            block_coordinates2: IVec3::new(-1, 3, 3),
            expected_clearance: Some(MAXIMUM_CLEARANCE),
            expected_cost: Some(WORLD_EDGE_COST),
        },
        EdgeValidationCase {
            description: "case 6".to_string(),
            chunk_coordinates1: IVec3::new(0, -1, -2),
            block_coordinates1: IVec3::new(-1, 3, 3),
            chunk_coordinates2: IVec3::new(0, 0, -1),
            block_coordinates2: IVec3::new(-1, -3, -3),
            expected_clearance: Some(MAXIMUM_CLEARANCE),
            expected_cost: Some(WORLD_EDGE_COST),
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}

struct GroupIDCase {
    description: String,
    chunk_coordinates: IVec3,
    block_coordinates1: IVec3,
    block_coordinates2: IVec3,
    expected_in_same_group: bool,
}

impl GroupIDCase {
    pub fn check(&self, world: &World) {
        let chunk_id = world
            .grid
            .chunk_coordinates_to_chunk_id(self.chunk_coordinates)
            .expect("invalid chunk 1 coordinates");

        let block_id1 = world
            .grid
            .block_coordinates_to_block_id(self.block_coordinates1)
            .expect("invalid block 1 coordinates");

        let block_id2 = world
            .grid
            .block_coordinates_to_block_id(self.block_coordinates2)
            .expect("invalid block 2 coordinates");

        let chunk_graph = world
            .graph
            .get_chunk_graph(chunk_id)
            .expect("invalid chunk id");

        let block_node1 = chunk_graph
            .get_block_node(block_id1)
            .expect("invalid block id 1");

        let block_node2 = chunk_graph
            .get_block_node(block_id2)
            .expect("invalid block id 2");

        println!("{:?} {:?}", block_node1.group_id, block_node2.group_id);

        assert_eq!(
            block_node1.group_id == block_node2.group_id,
            self.expected_in_same_group,
            "{:?}",
            self.description
        );
    }
}

#[test]
fn group_id_validation() {
    let mut world = World::new(TEST_CHUNK_RADIUS as u32, TEST_WORLD_RADIUS as u32);

    builder::TestWorld::build(&mut world);

    let test_cases = vec![
        GroupIDCase {
            description: "case 1".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -2),
            block_coordinates1: IVec3::new(0, 0, 0),
            block_coordinates2: IVec3::new(1, 0, 0),
            expected_in_same_group: true,
        },
        GroupIDCase {
            description: "case 2".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -2),
            block_coordinates1: IVec3::new(0, -2, 1),
            block_coordinates2: IVec3::new(-1, -2, 1),
            expected_in_same_group: true,
        },
        GroupIDCase {
            description: "case 3".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -2),
            block_coordinates1: IVec3::new(0, 0, 0),
            block_coordinates2: IVec3::new(-1, -2, 1),
            expected_in_same_group: false,
        },
        GroupIDCase {
            description: "case 4".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -2),
            block_coordinates1: IVec3::new(1, -2, -1),
            block_coordinates2: IVec3::new(2, -2, -2),
            expected_in_same_group: true,
        },
        GroupIDCase {
            description: "case 5".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -2),
            block_coordinates1: IVec3::new(-2, 1, -2),
            block_coordinates2: IVec3::new(-2, 0, -1),
            expected_in_same_group: true,
        },
        GroupIDCase {
            description: "case 6".to_string(),
            chunk_coordinates: IVec3::new(0, 0, -2),
            block_coordinates1: IVec3::new(-2, 0, -1),
            block_coordinates2: IVec3::new(-2, 1, -2),
            expected_in_same_group: true,
        },
    ];

    for case in test_cases {
        case.check(&world);
    }
}
