pub mod elevator_cap_template;
pub mod elevator_template;
pub mod generic_room_template;
pub mod trading_platform_template;
pub mod wireframe_template;

pub use elevator_cap_template::ElevatorCapTemplate;
pub use elevator_template::ElevatorTemplate;
pub use generic_room_template::GenericRoomTemplate;
pub use trading_platform_template::TradingPlatformTemplate;
pub use wireframe_template::WireframeTemplate;

use crate::simulation::state::{
    world::{block, Area},
    World,
};
use ultraviolet::IVec3;

pub trait Template {
    fn construct(area: &Area, world: &mut World);
}

pub fn set_block(min_offset: IVec3, block_kind: block::Kind, area: &Area, world: &mut World) {
    let local_ibox = Area::set_local(min_offset, IVec3::new(1, 1, 1), area);

    World::set_block(local_ibox.min, block_kind, &mut world.sector_vec);
}

pub fn set_cube(
    min_offset: IVec3,
    size: IVec3,
    block_kind: block::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_ibox = Area::set_local(min_offset, size, area);

    World::set_cube(
        local_ibox.min,
        local_ibox.max,
        block_kind,
        &mut world.sector_vec,
    );
}

pub fn set_box(
    min_offset: IVec3,
    size: IVec3,
    block_kind: block::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_ibox = Area::set_local(min_offset, size, area);

    World::set_box(
        local_ibox.min,
        local_ibox.max,
        block_kind,
        &mut world.sector_vec,
    );
}

pub fn set_wireframe(
    min_offset: IVec3,
    size: IVec3,
    block_kind: block::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_ibox = Area::set_local(min_offset, size, area);

    World::set_wireframe(
        local_ibox.min,
        local_ibox.max,
        block_kind,
        &mut world.sector_vec,
    );
}
