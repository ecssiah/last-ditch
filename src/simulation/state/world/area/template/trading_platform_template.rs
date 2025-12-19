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
        template::set_block_cube(
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

        template::set_block_cube(
            IVec3::new(1, 2, 1),
            IVec3::new(1, 1, 2),
            block::Kind::Server1,
            area,
            world,
        );

        template::set_block_cube(
            IVec3::new(8, 4, 1),
            IVec3::new(2, 2, 2),
            block::Kind::Server2,
            area,
            world,
        );

        template::set_block_cube(
            IVec3::new(1, 4, 1),
            IVec3::new(1, 2, 1),
            block::Kind::Server3,
            area,
            world,
        );
    }
}
