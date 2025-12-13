use crate::simulation::state::world::{
    area::template::{self, Template},
    block,
};
use ultraviolet::IVec3;

pub struct ElevatorCapTemplate {}

impl Template for ElevatorCapTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        template::set_box(
            IVec3::zero(),
            IVec3::zero(),
            block::Kind::Metal2,
            area,
            world,
        );
        template::set_cube(
            IVec3::new(2, 0, 1),
            IVec3::new(-2, 0, -3),
            block::Kind::None,
            area,
            world,
        );
        template::set_cube(
            IVec3::new(0, 2, 1),
            IVec3::new(0, -2, -3),
            block::Kind::None,
            area,
            world,
        );
        template::set_cube(
            IVec3::new(2, 2, 0),
            IVec3::new(-2, -2, 0),
            block::Kind::None,
            area,
            world,
        );
        template::set_wireframe(
            IVec3::new(1, 1, area.max.z + 1),
            IVec3::new(-1, -1, 1),
            block::Kind::Metal2,
            area,
            world,
        );
        template::set_wireframe(
            IVec3::new(2, 2, area.max.z + 2),
            IVec3::new(-2, -2, 2),
            block::Kind::Metal2,
            area,
            world,
        );
    }
}
