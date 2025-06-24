use crate::simulation::state::world::{
    block::{self},
    World,
};
use glam::IVec3;

pub struct TestWorld {}

impl TestWorld {
    pub fn build(world: &mut World) {
        Self::build_rooms(world);
        Self::build_north_test_room(world);
        Self::build_central_room(world);

        world.update_chunks();
    }

    fn build_rooms(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let world_radius = world.grid.world_radius as i32;

        for x in -world_radius..=world_radius {
            for y in -1..=1 {
                for z in -world_radius..=world_radius {
                    let chunk_coordinates = IVec3::new(x, y, z);

                    let chunk_position = world
                        .grid
                        .chunk_coordinates_to_position(chunk_coordinates)
                        .unwrap();

                    let component_sum =
                        chunk_coordinates.x + chunk_coordinates.y + chunk_coordinates.z;

                    let chunk_kind = if component_sum % 2 == 0 {
                        block::Kind::Polished2
                    } else {
                        block::Kind::Polished1
                    };

                    world.set_cube(
                        chunk_position - chunk_radius,
                        chunk_position + chunk_radius,
                        chunk_kind,
                    );
                }
            }
        }
    }

    fn build_central_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let chunk_position = IVec3::new(0, 0, 0);

        let chunk_coordinates = world
            .grid
            .position_to_chunk_coordinates(chunk_position)
            .unwrap();

        let chunk_position = world
            .grid
            .chunk_coordinates_to_position(chunk_coordinates)
            .unwrap();

        world.set_cube(
            chunk_position - chunk_radius + 1,
            chunk_position + chunk_radius - 1,
            block::Kind::Empty,
        );

        let center_position = IVec3::new(0, -4, 0);

        world.set_block_kind(center_position + IVec3::Z * 2, block::Kind::North);
        world.set_block_kind(center_position - IVec3::Z * 2, block::Kind::South);
        world.set_block_kind(center_position + IVec3::X * 2, block::Kind::East);
        world.set_block_kind(center_position - IVec3::X * 2, block::Kind::West);

        world.set_cube(
            chunk_position + IVec3::new(-1, -3, 0),
            chunk_position + IVec3::new(1, 0, 12),
            block::Kind::Empty,
        );
    }

    fn build_north_test_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let room_center_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(0, 0, 2))
            .unwrap();

        world.set_cube(
            room_center_position
                + IVec3::new(-3 * chunk_radius, -chunk_radius + 1, -3 * chunk_radius),
            room_center_position + IVec3::new(3 * chunk_radius, chunk_radius - 1, 3 * chunk_radius),
            block::Kind::Empty,
        );

        let clearance_test_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(0, 0, 3))
            .unwrap();

        world.set_block_kind(
            clearance_test_position + IVec3::new(-3, -3, 3),
            block::Kind::MagentaStone,
        );

        world.set_block_kind(
            clearance_test_position + IVec3::new(-2, -2, 3),
            block::Kind::MagentaStone,
        );

        world.set_block_kind(
            clearance_test_position + IVec3::new(-1, -1, 3),
            block::Kind::MagentaStone,
        );

        world.set_block_kind(
            clearance_test_position + IVec3::new(0, 0, 3),
            block::Kind::MagentaStone,
        );

        world.set_block_kind(
            clearance_test_position + IVec3::new(1, 1, 3),
            block::Kind::MagentaStone,
        );

        world.set_block_kind(
            clearance_test_position + IVec3::new(2, 2, 3),
            block::Kind::MagentaStone,
        );

        world.set_block_kind(
            clearance_test_position + IVec3::new(3, 3, 3),
            block::Kind::MagentaStone,
        );
    }
}
