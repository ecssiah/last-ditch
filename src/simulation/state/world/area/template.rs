pub mod elevator_cap_template;
pub mod elevator_template;
pub mod generic_room_template;
pub mod temple_template;
pub mod trading_platform_template;
pub mod wireframe_template;

pub use elevator_cap_template::ElevatorCapTemplate;
pub use elevator_template::ElevatorTemplate;
pub use generic_room_template::GenericRoomTemplate;
pub use temple_template::TempleTemplate;
pub use trading_platform_template::TradingPlatformTemplate;
pub use wireframe_template::WireframeTemplate;

use crate::simulation::state::{
    world::{
        block,
        grid::Direction,
        object::{ladder, stairs, ObjectManager},
        Area,
    },
    World,
};
use ultraviolet::IVec3;

pub trait Template {
    fn construct(area: &Area, world: &mut World);
}

pub fn set_block(min_offset: IVec3, block_kind: &block::Kind, area: &Area, world: &mut World) {
    let local_int_box = Area::set_local(min_offset, IVec3::new(1, 1, 1), area);

    World::set_block(local_int_box.min, block_kind, world);
}

pub fn set_block_cube(
    min_offset: IVec3,
    size: IVec3,
    block_kind: &block::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, size, area);

    World::set_block_cube(local_int_box.min, local_int_box.max, block_kind, world);
}

pub fn remove_block_cube(min_offset: IVec3, size: IVec3, area: &Area, world: &mut World) {
    let local_int_box = Area::set_local(min_offset, size, area);

    World::remove_block_cube(local_int_box.min, local_int_box.max, world);
}

pub fn set_block_box(
    min_offset: IVec3,
    size: IVec3,
    block_kind: &block::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, size, area);

    World::set_block_box(local_int_box.min, local_int_box.max, block_kind, world);
}

pub fn set_block_wireframe(
    min_offset: IVec3,
    size: IVec3,
    block_kind: &block::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, size, area);

    World::set_block_wireframe(local_int_box.min, local_int_box.max, block_kind, world);
}

pub fn set_stairs(
    min_offset: IVec3,
    stairs_kind: &stairs::Kind,
    direction: &Direction,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, IVec3::new(1, 1, 1), area);

    ObjectManager::set_stairs(local_int_box.min, stairs_kind, direction, world);
}

pub fn set_stairs_cube(
    min_offset: IVec3,
    size: IVec3,
    stairs_kind: &stairs::Kind,
    direction: &Direction,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, size, area);

    ObjectManager::set_stairs_cube(
        local_int_box.min,
        local_int_box.max,
        stairs_kind,
        direction,
        world,
    );
}

pub fn set_ladder(
    min_offset: IVec3,
    ladder_kind: &ladder::Kind,
    direction: &Direction,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, IVec3::new(1, 1, 1), area);

    ObjectManager::set_ladder(local_int_box.min, ladder_kind, direction, world);
}

pub fn set_ladder_cube(
    min_offset: IVec3,
    size: IVec3,
    ladder_kind: &ladder::Kind,
    direction: &Direction,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, size, area);

    ObjectManager::set_ladder_cube(
        local_int_box.min,
        local_int_box.max,
        ladder_kind,
        direction,
        world,
    );
}
