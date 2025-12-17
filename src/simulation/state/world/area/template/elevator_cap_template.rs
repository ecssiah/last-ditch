use crate::simulation::state::{
    world::{
        area::template::{self, Template},
        block,
        grid::{self, Direction},
        object,
    },
    World,
};
use ultraviolet::IVec3;

pub struct ElevatorCapTemplate {}

impl Template for ElevatorCapTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        template::set_box(IVec3::zero(), area.size, block::Kind::Metal2, area, world);

        template::set_cube(
            IVec3::new(2, 0, 1),
            IVec3::new(3, area.size.y, 4),
            block::Kind::None,
            area,
            world,
        );

        template::set_cube(
            IVec3::new(0, 2, 1),
            IVec3::new(area.size.x, 3, 4),
            block::Kind::None,
            area,
            world,
        );

        template::set_cube(
            IVec3::new(2, 2, 0),
            IVec3::new(3, 3, area.size.z),
            block::Kind::None,
            area,
            world,
        );

        template::set_wireframe(
            IVec3::new(1, 1, area.size.z),
            IVec3::new(area.size.x - 2, area.size.y - 2, 1),
            block::Kind::Metal2,
            area,
            world,
        );

        template::set_wireframe(
            IVec3::new(2, 2, area.size.z + 1),
            IVec3::new(area.size.x - 4, area.size.y - 4, 1),
            block::Kind::Metal2,
            area,
            world,
        );

        let area_ibox = grid::get_grid_ibox(area.grid_position, area.size);

        World::set_object(
            IVec3::new(area_ibox.min.x + 2, area_ibox.min.y + 2, area_ibox.min.z),
            Direction::South,
            object::Kind::Platform,
            world,
        );
    }
}
