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

        assert_eq!(
            chunk.graph.node_list.len(),
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

#[test]
fn edge_count_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let chunk_east_grid_position = test_world.grid.chunk_to_grid(IVec3::new(1, 0, 0)).unwrap();
    let chunk_east = test_world.get_chunk_at(chunk_east_grid_position).unwrap();

    let test_node_index1 = chunk_east
        .graph
        .get_node_index(chunk_east_grid_position + IVec3::new(-1, -3, 0))
        .unwrap();

    let test_node1 = &chunk_east.graph.node_list[test_node_index1];

    assert_eq!(test_node1.edge_list.len(), 6, "Edge count at (-1, -3, 0)");

    let test_node_index2 = chunk_east
        .graph
        .get_node_index(chunk_east_grid_position + IVec3::new(1, -3, -1))
        .unwrap();

    let test_node2 = &chunk_east.graph.node_list[test_node_index2];

    assert_eq!(test_node2.edge_list.len(), 2, "Edge count at (1, -3, -1)");

    let test_node_index3 = chunk_east
        .graph
        .get_node_index(chunk_east_grid_position + IVec3::new(1, -3, 1))
        .unwrap();

    let test_node3 = &chunk_east.graph.node_list[test_node_index3];

    assert_eq!(test_node3.edge_list.len(), 4, "Edge count at (1, -3, 1)");
}

#[test]
fn edge_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let chunk_east_grid_position = test_world.grid.chunk_to_grid(IVec3::new(1, 0, 0)).unwrap();
    let chunk_east = test_world.get_chunk_at(chunk_east_grid_position).unwrap();

    let test_node1_grid_position = chunk_east_grid_position + IVec3::new(0, -2, 0);
    let test_node2_grid_position = chunk_east_grid_position + IVec3::new(-1, -3, 0);
    let test_node3_grid_position = chunk_east_grid_position + IVec3::new(1, -3, -1);
    let test_node4_grid_position = chunk_east_grid_position + IVec3::new(1, -3, 1);
    let test_node5_grid_position = chunk_east_grid_position + IVec3::new(2, -2, 2);

    let test_node_index1 = chunk_east
        .graph
        .get_node_index(test_node1_grid_position)
        .unwrap();

    let test_node_index2 = chunk_east
        .graph
        .get_node_index(test_node2_grid_position)
        .unwrap();

    let edge12 = chunk_east.graph.node_list[test_node_index1]
        .edge_list
        .iter()
        .find(|edge| edge.target == test_node_index2);

    assert!(edge12.is_some(), "Edge between (0, -2, 0) and (-1, -3, 0)");

    let edge21 = chunk_east.graph.node_list[test_node_index2]
        .edge_list
        .iter()
        .find(|edge| edge.target == test_node_index1);

    assert!(edge21.is_some(), "Edge between (-1, -3, 0) and (0, -2, 0)");

    let test_node_index3 = chunk_east
        .graph
        .get_node_index(test_node3_grid_position)
        .unwrap();

    let edge13 = chunk_east.graph.node_list[test_node_index1]
        .edge_list
        .iter()
        .find(|edge| edge.target == test_node_index3);

    assert!(edge13.is_none(), "Edge between (0, -2, 0) and (1, -3, -1)");

    let test_node_index4 = chunk_east
        .graph
        .get_node_index(test_node4_grid_position)
        .unwrap();

    let test_node_index5 = chunk_east
        .graph
        .get_node_index(test_node5_grid_position)
        .unwrap();

    let edge45 = chunk_east.graph.node_list[test_node_index4]
        .edge_list
        .iter()
        .find(|edge| edge.target == test_node_index5);

    assert!(edge45.is_some(), "Edge between (1, -3, 1) and (2, -2, 2)");

    let edge54 = chunk_east.graph.node_list[test_node_index5]
        .edge_list
        .iter()
        .find(|edge| edge.target == test_node_index4);

    assert!(edge54.is_some(), "Edge between (2, -2, 2) and (1, -3, 1)");
}
