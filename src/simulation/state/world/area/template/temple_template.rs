use crate::simulation::state::{population::nation::{self, Nation}, world::{
    area::{self, template::{self, Template}},
    block,
}};
use ultraviolet::IVec3;

pub struct TempleTemplate {}

impl Template for TempleTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        if let area::Style::Temple { nation_kind } = area.style {
            template::set_block(
                IVec3::new(area.size.x / 2, area.size.y / 2, 4),
                Nation::get_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_wireframe(IVec3::zero(), area.size, block::Kind::Ornate4, area, world);
        }
    }
}
