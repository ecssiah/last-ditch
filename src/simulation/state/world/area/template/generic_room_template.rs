use crate::{
    simulation::state::{
        world::{
            area::template::{self, Template},
            block,
            grid::{self, Axis, Direction},
            object, Area,
        },
        World,
    },
    utils::ldmath::rand_chacha_ext::gen_i32,
};
use ultraviolet::IVec3;

pub struct GenericRoomTemplate {}

impl Template for GenericRoomTemplate {
    fn construct(area: &Area, world: &mut World) {
        template::set_box(
            IVec3::new(0, 0, 0),
            area.size,
            block::Kind::Metal1,
            area,
            world,
        );

        for connection in &area.connection_vec {
            let direction = match connection.line.axis {
                Axis::X => Direction::North,
                Axis::Y => Direction::East,
                Axis::Z => Direction::Up,
            };

            World::set_cube(
                connection.entrance_vec[0] + 1 * IVec3::unit_z(),
                connection.entrance_vec[0] + 2 * IVec3::unit_z(),
                block::Kind::None,
                &mut world.sector_vec,
            );

            World::set_object(
                connection.entrance_vec[0] + 1 * IVec3::unit_z(),
                direction,
                object::Kind::DoorOpen,
                world,
            );
        }

        let server_count = gen_i32(8, 16, &mut world.rng);

        let (area_min, area_max) = grid::get_bounds(area.grid_position, area.size);

        for _ in 0..server_count {
            let x = gen_i32(area_min.x + 1, area_max.x - 1, &mut world.rng);
            let y = gen_i32(area_min.y + 1, area_max.y - 1, &mut world.rng);
            let z = gen_i32(area_min.z + 2, area_max.z - 2, &mut world.rng);

            let server_kind_vec = vec![
                block::Kind::Server1,
                block::Kind::Server2,
                block::Kind::Server3,
                block::Kind::Server4,
            ];

            let server_block_kind = server_kind_vec[gen_i32(0, 3, &mut world.rng) as usize];

            World::set_cube(
                IVec3::new(x, y, area_min.z + 1),
                IVec3::new(x, y, z),
                server_block_kind,
                &mut world.sector_vec,
            );
        }
    }
}
