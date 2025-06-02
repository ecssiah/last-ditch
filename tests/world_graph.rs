use glam::IVec3;
use last_ditch::simulation::{
    world::{builder, World},
    TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS,
};

#[test]
fn node_count_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);

    let chunk_center_grid_position = test_world.grid.chunk_to_grid(IVec3::new(0, 0, 0)).unwrap();
    let chunk_center = test_world.get_chunk_at(chunk_center_grid_position).unwrap();

    assert_eq!(
        chunk_center.graph.node_list.len(),
        33,
        "Center chunk incorrect node count"
    );

    let chunk_west_grid_position = test_world.grid.chunk_to_grid(IVec3::new(-1, 0, 0)).unwrap();
    let chunk_west = test_world.get_chunk_at(chunk_west_grid_position).unwrap();

    assert_eq!(
        chunk_west.graph.node_list.len(),
        26,
        "West chunk incorrect node count"
    );

    let chunk_east_grid_position = test_world.grid.chunk_to_grid(IVec3::new(1, 0, 0)).unwrap();
    let chunk_east = test_world.get_chunk_at(chunk_east_grid_position).unwrap();

    assert_eq!(
        chunk_east.graph.node_list.len(),
        26,
        "East Chunk incorrect node count"
    );

    let chunk_up_grid_position = test_world.grid.chunk_to_grid(IVec3::new(0, 1, 0)).unwrap();
    let chunk_up = test_world.get_chunk_at(chunk_up_grid_position).unwrap();

    assert_eq!(
        chunk_up.graph.node_list.len(),
        0,
        "Vertical Chunk incorrect node count"
    );
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
