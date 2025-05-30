use glam::IVec3;
use last_ditch::simulation::{world::World, TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS};

#[test]
fn graph_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);
    test_world.setup_test_world();

    let chunk_center_grid_position = test_world.grid.chunk_to_grid(IVec3::new(0, 0, 0)).unwrap();
    let chunk_center = test_world.get_chunk_at(chunk_center_grid_position).unwrap();

    assert_eq!(
        chunk_center.graph.node_list.len(),
        27,
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
        0,
        "East Chunk incorrect node count"
    );
}
