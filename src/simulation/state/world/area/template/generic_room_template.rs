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
    utils::ldmath::rand_chacha_ext::{gen_bool, gen_i32},
};
use ultraviolet::IVec3;

pub struct GenericRoomTemplate {}

impl GenericRoomTemplate {
    pub fn resource_map(floor_number: i32) -> Vec<block::Kind> {
        if floor_number >= -1 {
            vec![block::Kind::Vent1, block::Kind::Vent2, block::Kind::Vent3]
        } else if floor_number >= -2 {
            vec![block::Kind::Vent2, block::Kind::Vent3, block::Kind::Vent4]
        } else if floor_number >= -3 {
            vec![
                block::Kind::Vent2,
                block::Kind::Vent3,
                block::Kind::Vent4,
                block::Kind::Server1,
                block::Kind::Server2,
            ]
        } else if floor_number >= -4 {
            vec![
                block::Kind::Server2,
                block::Kind::Server3,
                block::Kind::Server4,
                block::Kind::Ornate3,
            ]
        } else {
            vec![]
        }
    }
}

impl Template for GenericRoomTemplate {
    fn construct(area: &Area, world: &mut World) {
        template::set_block_box(
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

            World::remove_block_cube(
                connection.entrance_vec[0] + 0 * IVec3::unit_z(),
                connection.entrance_vec[0] + 1 * IVec3::unit_z(),
                world,
            );

            World::set_object(
                connection.entrance_vec[0] + 0 * IVec3::unit_z(),
                if gen_bool(&mut world.rng) {
                    object::Kind::DoorOpen
                } else {
                    object::Kind::DoorClosed
                },
                direction,
                world,
            );
        }

        let resource_count = gen_i32(8, 16, &mut world.rng);

        let area_ibox = grid::get_grid_ibox(area.grid_position, area.size);

        for _ in 0..resource_count {
            let x = gen_i32(area_ibox.min.x + 1, area_ibox.max.x - 1, &mut world.rng);
            let y = gen_i32(area_ibox.min.y + 1, area_ibox.max.y - 1, &mut world.rng);
            let z = gen_i32(area_ibox.min.z + 2, area_ibox.max.z - 2, &mut world.rng);

            let resource_block_kind_vec = GenericRoomTemplate::resource_map(area.floor_number);

            let resource_block_kind_index = gen_i32(
                0,
                (resource_block_kind_vec.len() - 1) as i32,
                &mut world.rng,
            ) as usize;

            let resource_block_kind = resource_block_kind_vec[resource_block_kind_index];

            World::set_block_cube(
                IVec3::new(x, y, area_ibox.min.z + 1),
                IVec3::new(x, y, z),
                resource_block_kind,
                world,
            );
        }
    }
}
