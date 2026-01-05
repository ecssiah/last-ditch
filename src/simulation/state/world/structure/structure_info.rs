use crate::{
    simulation::{constants::*, state::world::structure::structure_kind::StructureKind},
    utils::ldmath::FloatBox,
};
use strum::EnumCount;
use ultraviolet::Vec3;

#[derive(Clone, Debug)]
pub struct StructureInfo {
    pub structure_kind: StructureKind,
    pub solid: bool,
    pub float_box_array: &'static [FloatBox],
}

const PLATFORM_SHAPE: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 0.0, CELL_UNIT_04),
    radius: Vec3::new(CELL_RADIUS_IN_METERS, CELL_RADIUS_IN_METERS, CELL_UNIT_02),
}];

const DOOR_CLOSED_SHAPE: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 0.0, 0.0),
    radius: Vec3::new(CELL_SIZE_IN_METERS, CELL_UNIT_08, CELL_SIZE_IN_METERS),
}];

const LADDER_SHAPE: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 3.0 * CELL_UNIT_08, 0.0),
    radius: Vec3::new(CELL_SIZE_IN_METERS, CELL_UNIT_08, CELL_SIZE_IN_METERS),
}];

const STAIRS_SHAPE: [FloatBox; 2] = [
    FloatBox {
        center_position: Vec3::new(0.0, -CELL_UNIT_04, -CELL_UNIT_04),
        radius: Vec3::new(CELL_RADIUS_IN_METERS, CELL_UNIT_04, CELL_UNIT_04),
    },
    FloatBox {
        center_position: Vec3::new(0.0, CELL_UNIT_04, CELL_UNIT_04),
        radius: Vec3::new(CELL_RADIUS_IN_METERS, CELL_UNIT_04, CELL_UNIT_04),
    },
];

pub static STRUCTURE_INFO_ARRAY: [StructureInfo; StructureKind::COUNT] = [
    StructureInfo {
        structure_kind: StructureKind::Door1,
        solid: true,
        float_box_array: &DOOR_CLOSED_SHAPE,
    },
    StructureInfo {
        structure_kind: StructureKind::Ladder1,
        solid: true,
        float_box_array: &LADDER_SHAPE,
    },
    StructureInfo {
        structure_kind: StructureKind::Stairs1,
        solid: true,
        float_box_array: &STAIRS_SHAPE,
    },
];
