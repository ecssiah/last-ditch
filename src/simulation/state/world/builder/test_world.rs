use crate::simulation::state::world::{
    block::{self},
    grid, World,
};
use glam::IVec3;

pub struct TestWorld {}

impl TestWorld {
    pub fn build(world: &mut World) {
        Self::build_rooms(world);
        Self::build_north_test_room(world);
        Self::build_graph_test_room(world);
        Self::build_central_room(world);

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

        Self::build_chunk_room(
            world,
            chunk_position,
            Vec::from([grid::Direction::XoYoZp, grid::Direction::XoYoZn]),
        );

        let center_position = IVec3::new(0, -4, 0);

        world.set_block_kind(center_position + IVec3::Z * 2, block::Kind::North);
        world.set_block_kind(center_position - IVec3::Z * 2, block::Kind::South);
        world.set_block_kind(center_position + IVec3::X * 2, block::Kind::East);
        world.set_block_kind(center_position - IVec3::X * 2, block::Kind::West);

        world.set_cube(
            chunk_position + IVec3::new(-1, -3, 0),
            chunk_position + IVec3::new(1, 0, chunk_radius + 1),
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

    fn build_graph_test_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let chunk_size = world.grid.chunk_size as i32;

        let hallway_position_start = IVec3::new(0, 0, -chunk_radius);
        let hallway_position_end = IVec3::new(0, 0, -chunk_radius - chunk_size - 1);

        world.set_cube(
            hallway_position_start + IVec3::new(-1, -3, 0),
            hallway_position_end + IVec3::new(1, 0, 0),
            block::Kind::Empty,
        );

        Self::build_graph_test_center_room(world);
        Self::build_graph_test_constricted_entrance_room(world);
        Self::build_graph_test_expanded_entrance_room(world);
        Self::build_graph_test_multiple_entrance_room(world);
    }

    fn build_graph_test_center_room(world: &mut World) {
        let chunk_coordinates = IVec3::new(0, 0, -2);

        let chunk_position = world
            .grid
            .chunk_coordinates_to_position(chunk_coordinates)
            .unwrap();

        let entrances = Vec::from([]);

        Self::build_chunk_room(world, chunk_position, entrances);
    }

    fn build_graph_test_constricted_entrance_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let chunk_coordinates = IVec3::new(-1, 0, -2);

        let chunk_position = world
            .grid
            .chunk_coordinates_to_position(chunk_coordinates)
            .unwrap();

        let entrances = Vec::from([
            grid::Direction::XnYoZo,
            grid::Direction::XoYoZp,
            grid::Direction::XoYoZn,
        ]);

        Self::build_chunk_room(world, chunk_position, entrances);

        world.set_cube(
            chunk_position + IVec3::new(chunk_radius, 0, 0),
            chunk_position + IVec3::new(chunk_radius, -2, 0),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_position + IVec3::new(chunk_radius + 1, 1, -1),
            chunk_position + IVec3::new(chunk_radius + 1, -3, 1),
            block::Kind::Empty,
        );
    }

    fn build_graph_test_expanded_entrance_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let chunk_coordinates = IVec3::new(1, 0, -2);

        let chunk_position = world
            .grid
            .chunk_coordinates_to_position(chunk_coordinates)
            .unwrap();

        let entrances = Vec::from([
            grid::Direction::XpYoZo,
            grid::Direction::XoYoZp,
            grid::Direction::XoYoZn,
        ]);

        Self::build_chunk_room(world, chunk_position, entrances);

        world.set_cube(
            chunk_position + IVec3::new(-chunk_radius, 1, -2),
            chunk_position + IVec3::new(-chunk_radius + 2, -4, 2),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_position + IVec3::new(-chunk_radius - 1, 0, -1),
            chunk_position + IVec3::new(-chunk_radius - 1, -3, 1),
            block::Kind::Empty,
        );
    }

    fn build_graph_test_multiple_entrance_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let chunk_coordinates = IVec3::new(0, 0, -3);

        let chunk_position = world
            .grid
            .chunk_coordinates_to_position(chunk_coordinates)
            .unwrap();

        let entrances = Vec::from([grid::Direction::XpYoZo, grid::Direction::XnYoZo]);

        Self::build_chunk_room(world, chunk_position, entrances);

        world.set_cube(
            chunk_position + IVec3::new(-chunk_radius + 1, 0, chunk_radius),
            chunk_position + IVec3::new(-chunk_radius + 2, -3, chunk_radius + 1),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_position + IVec3::new(chunk_radius - 2, 0, chunk_radius),
            chunk_position + IVec3::new(chunk_radius - 1, -3, chunk_radius + 1),
            block::Kind::Empty,
        );
    }

    fn build_chunk_room(world: &mut World, position: IVec3, entrances: Vec<grid::Direction>) {
        let chunk_radius = world.grid.chunk_radius as i32;

        world.set_cube(
            position - IVec3::splat(chunk_radius - 1),
            position + IVec3::splat(chunk_radius - 1),
            block::Kind::Empty,
        );

        if entrances.contains(&grid::Direction::XpYoZo) {
            world.set_cube(
                position + IVec3::new(chunk_radius, 0, 1),
                position + IVec3::new(chunk_radius, -3, -1),
                block::Kind::Empty,
            );
        }

        if entrances.contains(&grid::Direction::XnYoZo) {
            world.set_cube(
                position + IVec3::new(-chunk_radius, 0, 1),
                position + IVec3::new(-chunk_radius, -3, -1),
                block::Kind::Empty,
            );
        }

        if entrances.contains(&grid::Direction::XoYoZp) {
            world.set_cube(
                position + IVec3::new(1, 0, chunk_radius),
                position + IVec3::new(-1, -3, chunk_radius),
                block::Kind::Empty,
            );
        }

        if entrances.contains(&grid::Direction::XoYoZn) {
            world.set_cube(
                position + IVec3::new(1, 0, -chunk_radius),
                position + IVec3::new(-1, -3, -chunk_radius),
                block::Kind::Empty,
            );
        }

        if entrances.contains(&grid::Direction::XoYpZo) {
            world.set_cube(
                position + IVec3::new(-1, chunk_radius, -1),
                position + IVec3::new(1, chunk_radius, 1),
                block::Kind::Empty,
            );
        }

        if entrances.contains(&grid::Direction::XoYnZo) {
            world.set_cube(
                position + IVec3::new(-1, -chunk_radius, -1),
                position + IVec3::new(1, -chunk_radius, 1),
                block::Kind::Empty,
            );
        }
    }
}
