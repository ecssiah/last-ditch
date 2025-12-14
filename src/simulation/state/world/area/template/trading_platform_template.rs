use crate::simulation::{
    constants::{TRADING_PLATFORM_RADIUS_X, TRADING_PLATFORM_RADIUS_Y},
    state::world::{
        area::template::{self, Template},
        block,
    },
};
use ultraviolet::IVec3;

pub struct TradingPlatformTemplate {}

impl Template for TradingPlatformTemplate {
    fn construct(
        area: &crate::simulation::state::world::Area,
        world: &mut crate::simulation::state::World,
    ) {
        template::set_cube(
            IVec3::new(0, 0, 0),
            IVec3::new(
                2 * TRADING_PLATFORM_RADIUS_X as i32 + 1,
                2 * TRADING_PLATFORM_RADIUS_Y as i32 + 1,
                1,
            ),
            block::Kind::Metal2,
            area,
            world,
        );

        template::set_cube(
            IVec3::new(2, 2, 2),
            IVec3::new(3, 1, 1),
            block::Kind::Ornate1,
            area,
            world,
        );

        // let (min, max) = Area::local_to_world_bounds(IVec3::zero(), IVec3::zero(), area);

        // World::set_box(min, max, block::Kind::Metal2, &mut world.sector_vec);

        // let (min, max) =
        //     Area::local_to_world_bounds(IVec3::new(2, 0, 1), IVec3::new(-2, 0, -3), area);

        // World::set_cube(min, max, block::Kind::None, &mut world.sector_vec);

        // let (min, max) =
        //     Area::local_to_world_bounds(IVec3::new(0, 2, 1), IVec3::new(0, -2, -3), area);

        // World::set_cube(min, max, block::Kind::None, &mut world.sector_vec);

        // let (min, max) =
        //     Area::local_to_world_bounds(IVec3::new(2, 2, 0), IVec3::new(-2, -2, 0), area);

        // World::set_cube(min, max, block::Kind::None, &mut world.sector_vec);

        // let (min, max) = Area::local_to_world_bounds(
        //     IVec3::new(1, 1, area.max.z + 1),
        //     IVec3::new(-1, -1, 1),
        //     area,
        // );

        // World::set_wireframe(min, max, block::Kind::Metal2, &mut world.sector_vec);

        // let (min, max) = Area::local_to_world_bounds(
        //     IVec3::new(2, 2, area.max.z + 2),
        //     IVec3::new(-2, -2, 2),
        //     area,
        // );

        // World::set_wireframe(min, max, block::Kind::Metal2, &mut world.sector_vec);
    }
}
