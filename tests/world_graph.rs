use glam::IVec3;
use last_ditch::simulation::world::{block, World};

#[test]
fn graph_validation() {
    let test_world = setup_test_world();

    if let Some(chunk_origin) = test_world.get_chunk_at(IVec3::ZERO) {
        assert_eq!(
            chunk_origin.graph.node_list.len(),
            9,
            "Incorrect node count"
        );
    } else {
        panic!("No origin chunk");
    }
}

fn setup_test_world() -> World {
    let mut test_world = World::new(1, 2);

    let boundary = test_world.grid.boundary as i32;
    let chunk_radius = test_world.grid.chunk_radius as i32;

    test_world.set_cube(
        IVec3::new(-boundary, -boundary, -boundary),
        IVec3::new(boundary, boundary, boundary),
        block::Kind::Polished1,
    );

    test_world.update_chunks();

    test_world
}
