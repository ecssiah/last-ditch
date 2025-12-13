use crate::simulation::state::{
    world::{area::template::Template, block},
    World,
};

pub struct WireframeTemplate {}

impl Template for WireframeTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        World::set_wireframe(
            area.min,
            area.max,
            block::Kind::Metal1,
            &mut world.sector_vec,
        );
    }
}
