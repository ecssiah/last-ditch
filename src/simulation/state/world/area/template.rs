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
    world::{block, grid::Direction, object, Area},
    World,
};
use ultraviolet::IVec3;

pub trait Template {
    fn construct(area: &Area, world: &mut World);
}

pub fn set_block(min_offset: IVec3, block_kind: block::Kind, area: &Area, world: &mut World) {
    let local_int_box = Area::set_local(min_offset, IVec3::new(1, 1, 1), area);

    World::set_block(local_int_box.min, block_kind, world);
}

pub fn set_block_cube(
    min_offset: IVec3,
    size: IVec3,
    block_kind: block::Kind,
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
    block_kind: block::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, size, area);

    World::set_block_box(local_int_box.min, local_int_box.max, block_kind, world);
}

pub fn set_block_wireframe(
    min_offset: IVec3,
    size: IVec3,
    block_kind: block::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, size, area);

    World::set_block_wireframe(local_int_box.min, local_int_box.max, block_kind, world);
}

pub fn set_object(
    min_offset: IVec3,
    direction: Direction,
    object_kind: object::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, IVec3::new(1, 1, 1), area);

    World::set_object(local_int_box.min, object_kind, direction, world);
}

pub fn set_object_cube(
    min_offset: IVec3,
    size: IVec3,
    direction: Direction,
    object_kind: object::Kind,
    area: &Area,
    world: &mut World,
) {
    let local_int_box = Area::set_local(min_offset, size, area);

    World::set_object_cube(
        local_int_box.min,
        local_int_box.max,
        direction,
        object_kind,
        world,
    );
}
