use crate::simulation::state::world::{block, World};
use glam::IVec3;

pub struct TestWorld {}

impl TestWorld {
    pub fn build(world: &mut World) {
        Self::build_rooms(world);

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
}
