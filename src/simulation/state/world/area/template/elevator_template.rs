use crate::simulation::state::{
    world::{area::template::Template, block},
    World,
};
use ultraviolet::IVec3;

pub struct ElevatorTemplate {}

impl Template for ElevatorTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        World::set_box(
            area.min,
            area.max,
            block::Kind::Metal2,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area.min.x + 2, area.min.y + 0, area.min.z + 1),
            IVec3::new(area.max.x - 2, area.max.y + 0, area.max.z - 3),
            block::Kind::None,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area.min.x + 0, area.min.y + 2, area.min.z + 1),
            IVec3::new(area.max.x + 0, area.max.y - 2, area.max.z - 3),
            block::Kind::None,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area.min.x + 2, area.min.y + 2, area.min.z + 0),
            IVec3::new(area.max.x - 2, area.max.y - 2, area.max.z + 0),
            block::Kind::None,
            &mut world.sector_vec,
        );
    }
}
