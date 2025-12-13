use crate::simulation::state::{
    world::{
        area::template::TemplateConstructor,
        block,
        grid::{Axis, Direction},
        object, Area,
    },
    World,
};
use ultraviolet::IVec3;

pub struct GenericRoomTemplate {}

impl TemplateConstructor for GenericRoomTemplate {
    fn construct(area: &Area, world: &mut World) {
        World::set_wireframe(
            area.min,
            area.max,
            block::Kind::Metal1,
            &mut world.sector_vec,
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
    }
}
