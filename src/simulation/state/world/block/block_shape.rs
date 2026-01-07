use ultraviolet::Vec3;
use crate::{simulation::constants::*, utils::ldmath::FloatBox};

pub const EMPTY_SHAPE_ARRAY: [FloatBox; 0] = [];

pub const BLOCK_SHAPE_ARRAY: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 0.0, 0.0),
    radius: Vec3::new(CELL_UNIT_01, CELL_UNIT_01, CELL_UNIT_01),
}];

pub const PLATFORM_SHAPE_ARRAY: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 0.0, CELL_UNIT_04),
    radius: Vec3::new(CELL_UNIT_02, CELL_UNIT_02, CELL_UNIT_02),
}];

pub const DOOR_CLOSED_X_SHAPE_ARRAY: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 0.0, 0.0),
    radius: Vec3::new(CELL_UNIT_01, CELL_UNIT_08, CELL_UNIT_01),
}];

pub const DOOR_OPEN_X_SHAPE_ARRAY: [FloatBox; 2] = [
    FloatBox {
        center_position: Vec3::new(-CELL_UNIT_04, 0.0, 0.0),
        radius: Vec3::new(CELL_UNIT_16, CELL_UNIT_08, CELL_UNIT_01),
    },
    FloatBox {
        center_position: Vec3::new(CELL_UNIT_04, 0.0, 0.0),
        radius: Vec3::new(CELL_UNIT_16, CELL_UNIT_08, CELL_UNIT_01),
    },
];

pub const DOOR_CLOSED_Y_SHAPE_ARRAY: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 0.0, 0.0),
    radius: Vec3::new(CELL_UNIT_01, CELL_UNIT_08, CELL_UNIT_01),
}];

pub const DOOR_OPEN_Y_SHAPE_ARRAY: [FloatBox; 2] = [
    FloatBox {
        center_position: Vec3::new(-CELL_UNIT_04, 0.0, 0.0),
        radius: Vec3::new(CELL_UNIT_16, CELL_UNIT_08, CELL_UNIT_01),
    },
    FloatBox {
        center_position: Vec3::new(CELL_UNIT_04, 0.0, 0.0),
        radius: Vec3::new(CELL_UNIT_16, CELL_UNIT_08, CELL_UNIT_01),
    },
];

pub const LADDER_NORTH_SHAPE_ARRAY: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, 3.0 * CELL_UNIT_08, 0.0),
    radius: Vec3::new(CELL_UNIT_01, CELL_UNIT_04, CELL_UNIT_01),
}];

pub const LADDER_WEST_SHAPE_ARRAY: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(-3.0 * CELL_UNIT_08, 0.0, 0.0),
    radius: Vec3::new(CELL_UNIT_04, CELL_UNIT_01, CELL_UNIT_01),
}];

pub const LADDER_SOUTH_SHAPE_ARRAY: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(0.0, -3.0 * CELL_UNIT_08, 0.0),
    radius: Vec3::new(CELL_UNIT_01, CELL_UNIT_08, CELL_UNIT_01),
}];

pub const LADDER_EAST_SHAPE_ARRAY: [FloatBox; 1] = [FloatBox {
    center_position: Vec3::new(3.0 * CELL_UNIT_08, 0.0, 0.0),
    radius: Vec3::new(CELL_UNIT_04, CELL_UNIT_01, CELL_UNIT_01),
}];

pub const STAIRS_NORTH_SHAPE_ARRAY: [FloatBox; 2] = [
    FloatBox {
        center_position: Vec3::new(0.0, -CELL_UNIT_04, -CELL_UNIT_04),
        radius: Vec3::new(CELL_UNIT_02, CELL_UNIT_04, CELL_UNIT_04),
    },
    FloatBox {
        center_position: Vec3::new(0.0, CELL_UNIT_04, CELL_UNIT_04),
        radius: Vec3::new(CELL_UNIT_02, CELL_UNIT_04, CELL_UNIT_04),
    },
];

pub const STAIRS_WEST_SHAPE_ARRAY: [FloatBox; 2] = [
    FloatBox {
        center_position: Vec3::new(0.0, -CELL_UNIT_04, -CELL_UNIT_04),
        radius: Vec3::new(CELL_UNIT_02, CELL_UNIT_04, CELL_UNIT_04),
    },
    FloatBox {
        center_position: Vec3::new(0.0, CELL_UNIT_04, CELL_UNIT_04),
        radius: Vec3::new(CELL_UNIT_02, CELL_UNIT_04, CELL_UNIT_04),
    },
];

pub const STAIRS_SOUTH_SHAPE_ARRAY: [FloatBox; 2] = [
    FloatBox {
        center_position: Vec3::new(0.0, -CELL_UNIT_04, -CELL_UNIT_04),
        radius: Vec3::new(CELL_UNIT_02, CELL_UNIT_04, CELL_UNIT_04),
    },
    FloatBox {
        center_position: Vec3::new(0.0, CELL_UNIT_04, CELL_UNIT_04),
        radius: Vec3::new(CELL_UNIT_02, CELL_UNIT_04, CELL_UNIT_04),
    },
];

pub const STAIRS_EAST_SHAPE_ARRAY: [FloatBox; 2] = [
    FloatBox {
        center_position: Vec3::new(0.0, -CELL_UNIT_04, -CELL_UNIT_04),
        radius: Vec3::new(CELL_UNIT_02, CELL_UNIT_04, CELL_UNIT_04),
    },
    FloatBox {
        center_position: Vec3::new(0.0, CELL_UNIT_04, CELL_UNIT_04),
        radius: Vec3::new(CELL_UNIT_02, CELL_UNIT_04, CELL_UNIT_04),
    },
];