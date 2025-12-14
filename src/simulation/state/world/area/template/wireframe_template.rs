use crate::simulation::state::world::{
    area::template::{self, Template},
    block,
};

pub struct WireframeTemplate {}

impl Template for WireframeTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        template::set_wireframe(
            area.grid_position,
            area.size,
            block::Kind::Metal1,
            area,
            world,
        );
    }
}
