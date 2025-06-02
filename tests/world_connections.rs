use last_ditch::simulation::{
    world::{builder, World},
    TEST_CHUNK_RADIUS, TEST_WORLD_RADIUS,
};

#[test]
fn connection_count_validation() {
    let mut test_world = World::new(TEST_WORLD_RADIUS as u32, TEST_CHUNK_RADIUS as u32);

    builder::TestWorld::build(&mut test_world);
}
