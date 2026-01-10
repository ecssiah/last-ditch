use crate::simulation::state::world::{
    area::template::{self, Template},
    block::block_kind::BlockKind,
    grid::Direction,
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
            &Direction::North,
            &BlockKind::Caution1,
            area,
            world,
        );
    }
}
