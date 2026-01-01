use crate::simulation::state::world::{
    area::template::{self, Template},
    block,
};
use ultraviolet::IVec3;

pub struct WireframeTemplate {}

impl Template for WireframeTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        template::set_block_wireframe(
            IVec3::new(0, 0, 0),
            area.size,
            &BlockKind::Caution,
            area,
            world,
        );
    }
}
