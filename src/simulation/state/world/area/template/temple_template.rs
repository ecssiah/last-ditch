use crate::simulation::state::{
    population::nation::Nation,
    world::{
        area::{
            self,
            template::{self, Template},
        },
        block,
        object::{ladder, stairs},
    },
};
use ultraviolet::IVec3;

pub struct TempleTemplate {}

impl Template for TempleTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        if let area::Style::Temple { nation_kind } = area.style {
            template::set_block_wireframe(
                IVec3::new(1, 1, 1),
                area.size - IVec3::new(2, 2, 2),
                &Nation::get_material_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_block(
                IVec3::new(1, 1, area.size.z - 3),
                &Nation::get_symbol_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_block(
                IVec3::new(area.size.x - 2, 1, area.size.z - 3),
                &Nation::get_symbol_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_block(
                IVec3::new(1, area.size.y - 2, area.size.z - 3),
                &Nation::get_symbol_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_block(
                IVec3::new(area.size.x - 2, area.size.y - 2, area.size.z - 3),
                &Nation::get_symbol_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_block(
                IVec3::new(1, 1, 2),
                &Nation::get_symbol_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_block(
                IVec3::new(area.size.x - 2, 1, 2),
                &Nation::get_symbol_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_block(
                IVec3::new(1, area.size.y - 2, 2),
                &Nation::get_symbol_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_block(
                IVec3::new(area.size.x - 2, area.size.y - 2, 2),
                &Nation::get_symbol_block_kind(&nation_kind),
                area,
                world,
            );

            template::set_block_cube(
                IVec3::new(0, 0, 0),
                IVec3::new(area.size.x, area.size.y, 1),
                &block::Kind::Stone4,
                area,
                world,
            );

            template::set_block_cube(
                IVec3::new(1, 1, 1),
                IVec3::new(area.size.x - 2, area.size.y - 2, 1),
                &block::Kind::Stone4,
                area,
                world,
            );

            template::set_block_cube(
                IVec3::new(0, 0, area.size.z - 1),
                IVec3::new(area.size.x, area.size.y, 1),
                &block::Kind::Stone4,
                area,
                world,
            );

            template::set_block_cube(
                IVec3::new(1, 1, area.size.z - 2),
                IVec3::new(area.size.x - 2, area.size.y - 2, 1),
                &block::Kind::Stone4,
                area,
                world,
            );

            template::remove_block_cube(
                IVec3::new(area.size.x / 2 - 1, area.size.y - 2, 1),
                IVec3::new(3, 1, 1),
                area,
                world,
            );

            template::remove_block_cube(
                IVec3::new(area.size.x / 2 - 1, area.size.y - 1, 0),
                IVec3::new(3, 1, 1),
                area,
                world,
            );

            template::set_stairs_cube(
                IVec3::new(area.size.x / 2 - 1, area.size.y - 2, 1),
                IVec3::new(3, 1, 1),
                &stairs::Kind::Stairs1,
                &area.direction,
                area,
                world,
            );

            template::set_stairs_cube(
                IVec3::new(area.size.x / 2 - 1, area.size.y - 1, 0),
                IVec3::new(3, 1, 1),
                &stairs::Kind::Stairs1,
                &area.direction,
                area,
                world,
            );

            template::set_ladder_cube(
                IVec3::new(1, -1, 0),
                IVec3::new(1, 1, area.size.z + 1),
                &ladder::Kind::Ladder1,
                &area.direction,
                area,
                world,
            );

            template::set_ladder_cube(
                IVec3::new(area.size.x - 2, -1, 0),
                IVec3::new(1, 1, area.size.z + 1),
                &ladder::Kind::Ladder1,
                &area.direction,
                area,
                world,
            );
        }
    }
}
