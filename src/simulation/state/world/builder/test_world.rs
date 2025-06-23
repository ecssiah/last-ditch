use crate::simulation::state::world::{
    block::{self},
    World,
};
use glam::IVec3;

pub struct TestWorld {}

impl TestWorld {
    pub fn build(world: &mut World) {
        Self::build_rooms(world);
        Self::build_entrances(world);
        Self::build_compass(world);

        world.update_chunks();
    }

    fn build_rooms(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let world_radius = world.grid.world_radius as i32;

        for x in -world_radius..=world_radius {
            for z in -world_radius..=world_radius {
                let chunk_coordinates = IVec3::new(x, 0, z);

                let chunk_position = world
                    .grid
                    .chunk_coordinates_to_position(chunk_coordinates)
                    .unwrap();

                let component_sum = chunk_coordinates.x + chunk_coordinates.y + chunk_coordinates.z;

                let chunk_kind = if component_sum % 2 == 0 {
                    block::Kind::Polished2
                } else {
                    block::Kind::Polished1
                };

                world.set_box(
                    chunk_position - chunk_radius,
                    chunk_position + chunk_radius,
                    chunk_kind,
                );
            }
        }
    }

    fn build_entrances(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let world_radius = world.grid.world_radius as i32;
        let entrance_boundary = (world.grid.world_boundary as i32) - chunk_radius;

        for offset in -world_radius..=world_radius {
            {
                let chunk_coordinates = IVec3::new(offset, 0, 0);

                let chunk_position = world
                    .grid
                    .chunk_coordinates_to_position(chunk_coordinates)
                    .unwrap();

                world.set_cube(
                    chunk_position + IVec3::new(-1, -(chunk_radius - 1), -entrance_boundary),
                    chunk_position + IVec3::new(1, 0, entrance_boundary),
                    block::Kind::Empty,
                );
            }

            {
                let chunk_coordinates = IVec3::new(0, 0, offset);

                let chunk_position = world
                    .grid
                    .chunk_coordinates_to_position(chunk_coordinates)
                    .unwrap();

                world.set_cube(
                    chunk_position + IVec3::new(-entrance_boundary, -(chunk_radius - 1), -1),
                    chunk_position + IVec3::new(entrance_boundary, 0, 1),
                    block::Kind::Empty,
                );
            }
        }
    }

    fn build_compass(world: &mut World) {
        let center_position = IVec3::new(0, -4, 0);

        world.set_block_kind(center_position + IVec3::Z * 2, block::Kind::North);
        world.set_block_kind(center_position - IVec3::Z * 2, block::Kind::South);
        world.set_block_kind(center_position + IVec3::X * 2, block::Kind::East);
        world.set_block_kind(center_position - IVec3::X * 2, block::Kind::West);
    }
}
