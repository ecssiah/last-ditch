use crate::simulation::state::{
    world::{area::template::TemplateConstructor, block, Area},
    World,
};
use ultraviolet::IVec3;

pub struct TradingPlatformTemplate {}

impl TemplateConstructor for TradingPlatformTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        let (min, max) = Area::local_to_world_bounds(IVec3::zero(), IVec3::zero(), area);

        World::set_box(min, max, block::Kind::Metal2, &mut world.sector_vec);

        let (min, max) = Area::local_to_world_bounds(
            IVec3::new(2, 0, 1),
            IVec3::new(-2, 0, -3),
            area,
        );

        World::set_cube(min, max, block::Kind::None, &mut world.sector_vec);

        let (min, max) = Area::local_to_world_bounds(
            IVec3::new(0, 2, 1), 
            IVec3::new(0, -2, -3),
            area,
        );

        World::set_cube(
            min,
            max,
            block::Kind::None,
            &mut world.sector_vec,
        );

        let (min, max) = Area::local_to_world_bounds(
            IVec3::new(2, 2, 0), 
            IVec3::new(-2, -2, 0),
            area,
        );

        World::set_cube(
            min,
            max,
            block::Kind::None,
            &mut world.sector_vec,
        );

        let (min, max) = Area::local_to_world_bounds(
            IVec3::new(1, 1, area.max.z + 1), 
            IVec3::new(-1, -1, 1),
            area,
        );

        World::set_wireframe(
            min,
            max,
            block::Kind::Metal2,
            &mut world.sector_vec,
        );

        let (min, max) = Area::local_to_world_bounds(
            IVec3::new(2, 2, area.max.z + 2), 
            IVec3::new(-2, -2, 2),
            area,
        );

        World::set_wireframe(
            min,
            max,
            block::Kind::Metal2,
            &mut world.sector_vec,
        );
    }
}
