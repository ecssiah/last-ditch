use crate::simulation::state::{
    world::{area::template::Template, block, grid},
    World,
};
use ultraviolet::IVec3;

pub struct ElevatorTemplate {}

impl Template for ElevatorTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        let (area_min, area_max) = grid::get_bounds(area.grid_position, area.size);

        World::set_box(
            area_min,
            area_max,
            block::Kind::Metal2,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area_min.x + 2, area_min.y + 0, area_min.z + 1),
            IVec3::new(area_max.x - 2, area_max.y + 0, area_max.z - 3),
            block::Kind::None,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area_min.x + 0, area_min.y + 2, area_min.z + 1),
            IVec3::new(area_max.x + 0, area_max.y - 2, area_max.z - 3),
            block::Kind::None,
            &mut world.sector_vec,
        );

        World::set_cube(
            IVec3::new(area_min.x + 2, area_min.y + 2, area_min.z + 0),
            IVec3::new(area_max.x - 2, area_max.y - 2, area_max.z + 0),
            block::Kind::None,
            &mut world.sector_vec,
        );
    }
}
