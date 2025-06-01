use crate::simulation::world::{block, World};
use glam::IVec3;

pub struct TestWorld {}

impl TestWorld {
    pub fn setup(world: &mut World) {
        Self::setup_ground(world);
        Self::setup_central_room(world);
        Self::setup_empty_room(world);
        Self::setup_clearance_test(world);
        Self::setup_graph_test(world);
        Self::setup_connection_test(world);

        world.update_chunks();
    }

    fn setup_ground(world: &mut World) {
        let boundary = world.grid.boundary as isize;

        for x in -boundary..=boundary {
            for y in -boundary..=boundary {
                for z in -boundary..=boundary {
                    let grid_position = IVec3::new(x as i32, y as i32, z as i32);
                    let chunk_position = world.grid.grid_to_chunk(grid_position).unwrap();

                    let kind = if (chunk_position.x + chunk_position.y + chunk_position.z) % 2 == 0
                    {
                        block::Kind::Polished2
                    } else {
                        block::Kind::Polished1
                    };

                    world.set_block_kind(grid_position.x, grid_position.y, grid_position.z, kind);
                }
            }
        }
    }

    pub fn setup_central_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_center_grid_position = world.grid.chunk_to_grid(IVec3::new(0, 0, 0)).unwrap();
        let chunk_south_grid_position = world.grid.chunk_to_grid(IVec3::new(0, 0, -1)).unwrap();
        let chunk_north_grid_position = world.grid.chunk_to_grid(IVec3::new(0, 0, 1)).unwrap();
        let chunk_west_grid_position = world.grid.chunk_to_grid(IVec3::new(-1, 0, 0)).unwrap();
        let chunk_east_grid_position = world.grid.chunk_to_grid(IVec3::new(1, 0, 0)).unwrap();

        world.set_cube(
            chunk_center_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius),
            chunk_center_grid_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );

        world.set_block_kind(0, -chunk_radius, 1, block::Kind::North);
        world.set_block_kind(-1, -chunk_radius, 0, block::Kind::West);
        world.set_block_kind(0, -chunk_radius, -1, block::Kind::South);
        world.set_block_kind(1, -chunk_radius, 0, block::Kind::East);

        world.set_cube(
            chunk_center_grid_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_north_grid_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_center_grid_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_west_grid_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_center_grid_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_south_grid_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_center_grid_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_east_grid_position + IVec3::new(0, -chunk_radius + 3, 0),
            block::Kind::Empty,
        );
    }

    pub fn setup_clearance_test(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_north_grid_position = world.grid.chunk_to_grid(IVec3::new(0, 0, 1)).unwrap();

        world.set_cube(
            chunk_north_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_north_grid_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );

        world.set_block_kind(
            chunk_north_grid_position.x - 2,
            chunk_north_grid_position.y - chunk_radius + 1,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );

        world.set_block_kind(
            chunk_north_grid_position.x - 1,
            chunk_north_grid_position.y - chunk_radius + 2,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );

        world.set_block_kind(
            chunk_north_grid_position.x,
            chunk_north_grid_position.y - chunk_radius + 3,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );

        world.set_block_kind(
            chunk_north_grid_position.x + 1,
            chunk_north_grid_position.y - chunk_radius + 4,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );

        world.set_block_kind(
            chunk_north_grid_position.x + 2,
            chunk_north_grid_position.y - chunk_radius + 5,
            chunk_north_grid_position.z + 2,
            block::Kind::Origin,
        );
    }

    fn setup_empty_room(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_west_grid_position = world.grid.chunk_to_grid(IVec3::new(-1, 0, 0)).unwrap();

        world.set_cube(
            chunk_west_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_west_grid_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );
    }

    fn setup_graph_test(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_east_grid_position = world.grid.chunk_to_grid(IVec3::new(1, 0, 0)).unwrap();

        world.set_cube(
            chunk_east_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_east_grid_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius - 1),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_east_grid_position + IVec3::new(1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_east_grid_position
                + IVec3::new(chunk_radius - 1, -chunk_radius + 1, chunk_radius - 1),
            block::Kind::Origin,
        );

        world.set_cube(
            chunk_east_grid_position + IVec3::new(1, -chunk_radius + 1, 1),
            chunk_east_grid_position + IVec3::new(1, -chunk_radius + 1, -1),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_east_grid_position + IVec3::new(0, -chunk_radius + 1, 0),
            chunk_east_grid_position + IVec3::new(0, -chunk_radius + 1, 0),
            block::Kind::Origin,
        );

        world.set_cube(
            chunk_east_grid_position + IVec3::new(1, -chunk_radius + 1, 0),
            chunk_east_grid_position + IVec3::new(1, -chunk_radius + 2, 0),
            block::Kind::Origin,
        );

        world.set_cube(
            chunk_east_grid_position + IVec3::new(2, -chunk_radius + 1, 0),
            chunk_east_grid_position + IVec3::new(2, -chunk_radius + 3, 0),
            block::Kind::Origin,
        );

        world.set_block_kind(
            chunk_east_grid_position.x + 2,
            chunk_east_grid_position.y - 1,
            chunk_east_grid_position.z - 1,
            block::Kind::Origin,
        );
    }

    fn setup_connection_test(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;

        let chunk_south_transition_grid_position =
            world.grid.chunk_to_grid(IVec3::new(0, 0, -1)).unwrap();
        let chunk_south_grid_position = world.grid.chunk_to_grid(IVec3::new(0, 0, -2)).unwrap();

        world.set_cube(
            chunk_south_transition_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius),
            chunk_south_transition_grid_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_south_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, -chunk_radius + 1),
            chunk_south_grid_position
                + IVec3::new(chunk_radius - 1, chunk_radius - 1, chunk_radius),
            block::Kind::Empty,
        );

        world.set_cube(
            chunk_south_grid_position
                + IVec3::new(-chunk_radius + 1, -chunk_radius + 1, chunk_radius),
            chunk_south_grid_position
                + IVec3::new(chunk_radius - 1, -chunk_radius + 1, chunk_radius),
            block::Kind::Origin,
        );

        world.set_block_kind(
            chunk_south_grid_position.x + 2,
            chunk_south_grid_position.y - chunk_radius + 1,
            chunk_south_grid_position.z + chunk_radius + 1,
            block::Kind::Origin,
        );

        world.set_block_kind(
            chunk_south_grid_position.x + 1,
            chunk_south_grid_position.y - chunk_radius + 2,
            chunk_south_grid_position.z + chunk_radius,
            block::Kind::Origin,
        );

        world.set_block_kind(
            chunk_south_grid_position.x - 2,
            chunk_south_grid_position.y - chunk_radius + 1,
            chunk_south_grid_position.z + chunk_radius + 1,
            block::Kind::Origin,
        );

        world.set_block_kind(
            chunk_south_grid_position.x,
            chunk_south_grid_position.y - chunk_radius + 1,
            chunk_south_grid_position.z + chunk_radius,
            block::Kind::Empty,
        );
    }
}
