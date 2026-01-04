use crate::{
    simulation::{
        constants::{
            CELL_RADIUS_IN_METERS, CELL_SIZE_IN_METERS, CELL_UNIT_EIGHTH, CELL_UNIT_QUARTER,
        },
        state::world::block::BlockKind,
    },
    utils::ldmath::FloatBox,
};
use strum::EnumCount;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub block_kind: BlockKind,
    pub solid: bool,
    pub float_box_array: &'static [FloatBox],
}

const BLOCK_SHAPE: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 0.0, 0.0),
    radius: Vec3::new(
        CELL_RADIUS_IN_METERS,
        CELL_RADIUS_IN_METERS,
        CELL_RADIUS_IN_METERS,
    ),
}];

const DOOR_CLOSED_SHAPE: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 0.0, 0.0),
    radius: Vec3::new(CELL_SIZE_IN_METERS, CELL_UNIT_EIGHTH, CELL_SIZE_IN_METERS),
}];

const LADDER_SHAPE: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 3.0 * CELL_UNIT_EIGHTH, 0.0),
    radius: Vec3::new(CELL_SIZE_IN_METERS, CELL_UNIT_EIGHTH, CELL_SIZE_IN_METERS),
}];

const STAIRS_SHAPE: [FloatBox; 2] = [
    FloatBox {
        center_position: Vec3::new(0.0, -CELL_UNIT_QUARTER, -CELL_UNIT_QUARTER),
        radius: Vec3::new(CELL_RADIUS_IN_METERS, CELL_UNIT_QUARTER, CELL_UNIT_QUARTER),
    },
    FloatBox {
        center_position: Vec3::new(0.0, CELL_UNIT_QUARTER, CELL_UNIT_QUARTER),
        radius: Vec3::new(CELL_RADIUS_IN_METERS, CELL_UNIT_QUARTER, CELL_UNIT_QUARTER),
    },
];

pub static BLOCK_INFO_ARRAY: [BlockInfo; BlockKind::COUNT] = [
    BlockInfo {
        block_kind: BlockKind::Engraved1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Engraved2,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Engraved3,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Engraved4,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Ornate1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Ornate2,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Ornate3,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Ornate4,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Carved1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Carved2,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Carved3,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Carved4,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Stone1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Stone2,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Stone3,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Stone4,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::LionSymbol,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::EagleSymbol,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::HorseSymbol,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::WolfSymbol,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::LionStone,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::EagleStone,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::HorseStone,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::WolfStone,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::NorthBlock,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::WestBlock,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::SouthBlock,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::EastBlock,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Server1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Server2,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Server3,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Server4,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Metal1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Metal2,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Metal3,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Metal4,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Panel1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Panel2,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Panel3,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Panel4,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Vent1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Vent2,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Vent3,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Vent4,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Caution1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Caution2,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Caution3,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Caution4,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Platform1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Stairs1,
        solid: true,
        float_box_array: &STAIRS_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Ladder1,
        solid: true,
        float_box_array: &BLOCK_SHAPE,
    },
    BlockInfo {
        block_kind: BlockKind::Door1,
        solid: true,
        float_box_array: &DOOR_CLOSED_SHAPE,
    },
];
