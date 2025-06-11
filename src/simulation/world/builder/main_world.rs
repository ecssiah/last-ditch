use crate::simulation::{
    population::agent,
    world::{block, World},
};
use glam::IVec3;

pub struct MainWorld {}

impl MainWorld {
    pub fn build(world: &mut World) {
        log::info!("Setup Ground");

        Self::build_ground(world);

        log::info!("Setup Structures");

        Self::build_compass(world);

        Self::build_temple(world, 0, 0, 34, agent::Kind::Eagle);
        Self::build_temple(world, -34, 0, 0, agent::Kind::Lion);
        Self::build_temple(world, 0, 0, -34, agent::Kind::Horse);
        Self::build_temple(world, 34, 0, 0, agent::Kind::Wolf);

        Self::build_observation_deck(world);

        world.update_chunks();
    }

    fn build_ground(world: &mut World) {
        let ground_boundary = (world.grid.world_boundary - world.grid.chunk_size) as isize;

        for x in -ground_boundary..=ground_boundary {
            for y in -1..=0 {
                for z in -ground_boundary..=ground_boundary {
                    let position = IVec3::new(x as i32, y as i32, z as i32);
                    let chunk_coordinates =
                        world.grid.position_to_chunk_coordinates(position).unwrap();

                    let component_sum =
                        chunk_coordinates.x + chunk_coordinates.y + chunk_coordinates.z;

                    let kind = if component_sum % 2 == 0 {
                        block::Kind::Polished1
                    } else {
                        block::Kind::Polished2
                    };

                    world.set_block_kind(position, kind);
                }
            }
        }
    }

    fn build_compass(world: &mut World) {
        world.set_block_kind(IVec3::new(0, 0, 0), block::Kind::TealStone);
        world.set_block_kind(IVec3::new(0, 0, 4), block::Kind::North);
        world.set_block_kind(IVec3::new(-4, 0, 0), block::Kind::West);
        world.set_block_kind(IVec3::new(0, 0, -4), block::Kind::South);
        world.set_block_kind(IVec3::new(4, 0, 0), block::Kind::East);
    }

    fn build_temple(world: &mut World, x: i32, y: i32, z: i32, kind: agent::Kind) {
        world.flags.insert(kind, IVec3::new(x, y + 3, z));

        world.set_block_kind(IVec3::new(x, y + 6, z), kind.icon());

        world.set_cube(
            IVec3::new(x - 8, y + 1, z - 8),
            IVec3::new(x + 8, y + 1, z + 8),
            block::Kind::Stone1,
        );

        world.set_cube(
            IVec3::new(x - 7, y + 2, z - 7),
            IVec3::new(x + 7, y + 2, z + 7),
            block::Kind::Stone1,
        );

        world.set_cube(
            IVec3::new(x - 6, y + 8, z - 6),
            IVec3::new(x + 6, y + 8, z + 6),
            block::Kind::Stone1,
        );

        world.set_cube(
            IVec3::new(x - 5, y + 9, z - 5),
            IVec3::new(x + 5, y + 9, z + 5),
            block::Kind::Stone1,
        );

        world.set_cube(
            IVec3::new(x - 5, y + 8, z - 5),
            IVec3::new(x + 5, y + 8, z + 5),
            block::Kind::Empty,
        );

        world.set_cube(
            IVec3::new(x + 5, y + 1, z + 5),
            IVec3::new(x + 5, y + 8, z + 5),
            block::Kind::Engraved1,
        );

        world.set_cube(
            IVec3::new(x - 5, y + 1, z + 5),
            IVec3::new(x - 5, y + 8, z + 5),
            block::Kind::Engraved1,
        );

        world.set_cube(
            IVec3::new(x + 5, y + 1, z - 5),
            IVec3::new(x + 5, y + 8, z - 5),
            block::Kind::Engraved1,
        );

        world.set_cube(
            IVec3::new(x - 5, y + 1, z - 5),
            IVec3::new(x - 5, y + 8, z - 5),
            block::Kind::Engraved1,
        );
    }

    fn build_observation_deck(world: &mut World) {
        let chunk_radius = world.grid.chunk_radius as i32;
        let chunk_size = world.grid.chunk_size as i32;

        let height = 16;
        let center = 3 * chunk_size;

        world.set_cube(
            IVec3::new(-center + 1, height, -center + 1),
            IVec3::new(-center - 1, 0, -center - 1),
            block::Kind::Polished2,
        );

        world.set_cube(
            IVec3::new(center + 1, height, -center + 1),
            IVec3::new(center - 1, 0, -center - 1),
            block::Kind::Polished2,
        );

        world.set_cube(
            IVec3::new(-center + 1, height, center + 1),
            IVec3::new(-center - 1, 0, center - 1),
            block::Kind::Polished2,
        );

        world.set_cube(
            IVec3::new(center + 1, height, center + 1),
            IVec3::new(center - 1, 0, center - 1),
            block::Kind::Polished2,
        );

        world.set_cube(
            IVec3::new(-center - chunk_radius, height, -center - chunk_radius),
            IVec3::new(center + chunk_radius, height, center + chunk_radius),
            block::Kind::Polished1,
        );

        world.set_cube(
            IVec3::new(
                -center + chunk_radius + 1,
                height,
                -center + chunk_radius + 1,
            ),
            IVec3::new(center - chunk_radius - 1, height, center - chunk_radius - 1),
            block::Kind::Empty,
        );
    }
}
