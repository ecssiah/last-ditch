use crate::simulation::state::world::{block, chunk::Chunk, World};
use glam::IVec3;

pub struct TestWorld {}

impl TestWorld {
    pub fn build(world: &mut World) {
        Self::build_rooms(world);
        // Self::build_ground(world);
        // Self::build_central_room(world);
        // Self::build_empty_room(world);
        // Self::build_clearance_test(world);
        // Self::build_chunk_graph_test(world);
        // Self::build_world_graph_test(world);

        world.update_chunks();
    }

    fn build_rooms(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let world_radius = world.grid.world_radius as i32;

        for x in -world_radius..=world_radius {
            for y in -world_radius..=world_radius {
                for z in -world_radius..=world_radius {
                    let chunk_coordinates = IVec3::new(x, y, z);

                    let chunk_position = world
                        .grid
                        .chunk_coordinates_to_position(chunk_coordinates)
                        .unwrap();

                    let component_sum =
                        chunk_coordinates.x + chunk_coordinates.y + chunk_coordinates.z;

                    let kind = if component_sum % 2 == 0 {
                        block::Kind::Polished2
                    } else {
                        block::Kind::Polished1
                    };

                    world.set_box(
                        chunk_position - chunk_radius,
                        chunk_position + chunk_radius,
                        kind,
                    );
                }
            }
        }

        let world_boundary = world.grid.world_boundary as i32;

        world.set_cube(
            IVec3::new(0, -3, -world_boundary),
            IVec3::new(0, -1, world_boundary),
            block::Kind::Empty,
        );

        world.set_cube(
            IVec3::new(-world_boundary, -3, 0),
            IVec3::new(world_boundary, -1, 0),
            block::Kind::Empty,
        );
    }

    pub fn build_central_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_center_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(0, 0, 0))
            .unwrap();
        let chunk_north_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(0, 0, 1))
            .unwrap();
        let chunk_south_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(0, 0, -1))
            .unwrap();
        let chunk_east_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(1, 0, 0))
            .unwrap();
        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        world.set_cube(
            chunk_center_position + IVec3::new(-chunk_radius, -chunk_radius + 1, -chunk_radius),
            chunk_center_position + IVec3::new(chunk_radius, chunk_radius - 1, chunk_radius),
            block::Kind::Empty,
        );

        let compass_radius = 2;

        world.set_block_kind(IVec3::new(0, -3, compass_radius), block::Kind::North);
        world.set_block_kind(IVec3::new(-compass_radius, -3, 0), block::Kind::West);
        world.set_block_kind(IVec3::new(0, -3, -compass_radius), block::Kind::South);
        world.set_block_kind(IVec3::new(compass_radius, -3, 0), block::Kind::East);

        world.set_cube(
            chunk_center_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_north_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_center_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_west_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_center_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_south_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_center_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_east_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );
    }

    pub fn build_clearance_test(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_north_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(0, 0, 1))
            .unwrap();

        world.set_cube(
            chunk_north_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_north_position + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );

        world.set_block_kind(
            chunk_north_position + IVec3::new(-2, -2, 2),
            block::Kind::PurpleStone,
        );

        world.set_block_kind(
            chunk_north_position + IVec3::new(-1, -1, 2),
            block::Kind::PurpleStone,
        );

        world.set_block_kind(
            chunk_north_position + IVec3::new(0, 0, 2),
            block::Kind::PurpleStone,
        );

        world.set_block_kind(
            chunk_north_position + IVec3::new(1, 1, 2),
            block::Kind::PurpleStone,
        );

        world.set_block_kind(
            chunk_north_position + IVec3::new(2, 2, 2),
            block::Kind::PurpleStone,
        );
    }

    fn build_empty_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_west_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(-1, 0, 0))
            .unwrap();

        world.set_cube(
            chunk_west_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_west_position + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );
    }

    fn build_chunk_graph_test(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_east_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(1, 0, 0))
            .unwrap();

        world.set_cube(
            chunk_east_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_east_position + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_east_position + IVec3::new(1, -2, 1),
            chunk_east_position + IVec3::new(2, -2, 2),
            block::Kind::TealStone,
        );

        world.set_block_kind(
            chunk_east_position + IVec3::new(2, -1, 2),
            block::Kind::TealStone,
        );

        world.set_cube(
            chunk_east_position + IVec3::new(2, -2, -2),
            chunk_east_position + IVec3::new(2, -1, -2),
            block::Kind::TealStone,
        );
    }

    fn build_world_graph_test(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_south_1_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(0, 0, -1))
            .unwrap();

        let chunk_south_2_position = world
            .grid
            .chunk_coordinates_to_position(IVec3::new(0, 0, -2))
            .unwrap();

        world.set_cube(
            chunk_south_1_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius),
            chunk_south_1_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_south_2_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_south_2_position + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_south_2_position + IVec3::new(1, -2, 3),
            chunk_south_2_position + IVec3::new(2, -2, 4),
            block::Kind::CrimsonStone,
        );

        world.set_cube(
            chunk_south_2_position + IVec3::new(-2, -2, 3),
            chunk_south_2_position + IVec3::new(-2, -1, 3),
            block::Kind::CrimsonStone,
        );

        world.set_cube(
            chunk_south_2_position + IVec3::new(2, -2, 3),
            chunk_south_2_position + IVec3::new(2, -1, 3),
            block::Kind::CrimsonStone,
        );

        world.set_cube(
            chunk_south_2_position + IVec3::new(-1, -3, 3),
            chunk_south_2_position + IVec3::new(-1, -4, 4),
            block::Kind::Empty,
        );

        world.set_block_kind(
            chunk_south_2_position + IVec3::new(-1, -4, 4),
            block::Kind::CrimsonStone,
        );

        world.set_cube(
            chunk_south_2_position + IVec3::new(0, -2, 0),
            chunk_south_2_position + IVec3::new(2, -1, 0),
            block::Kind::CrimsonStone,
        );

        world.set_cube(
            chunk_south_2_position + IVec3::new(0, -2, 0),
            chunk_south_2_position + IVec3::new(0, -1, -2),
            block::Kind::CrimsonStone,
        );

        world.set_block_kind(
            chunk_south_2_position + IVec3::new(-2, 0, -2),
            block::Kind::CrimsonStone,
        );

        world.set_block_kind(
            chunk_south_2_position + IVec3::new(-2, -1, -1),
            block::Kind::CrimsonStone,
        );
    }
}
