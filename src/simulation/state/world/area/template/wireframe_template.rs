use crate::simulation::state::{World, world::{area::template::Template, block}};

pub struct WireframeTemplate {}

impl Template for WireframeTemplate {
    fn construct(area: &crate::simulation::state::world::Area, world: &mut crate::simulation::state::World) {
        World::set_wireframe_box(
            area.min,
            area.max,
            block::Kind::Metal1,
            &mut world.sector_vec,
        );
    }
}