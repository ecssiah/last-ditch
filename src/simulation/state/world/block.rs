pub mod block_kind;
pub mod block_shape;
pub mod block_state;

use ultraviolet::IVec3;

use crate::{
    simulation::state::world::{
        block::{
            block_kind::BlockKind,
            block_shape::BlockShape,
            block_state::{block_data::BlockData, door_data::DoorData, BlockState},
        },
        grid::Direction,
    },
    utils::ldmath::FloatBox,
};

#[derive(Clone)]
pub struct Block {
    pub block_kind: BlockKind,
    pub grid_position: IVec3,
    pub direction: Direction,
    pub block_shape: BlockShape,
    pub block_state: BlockState,
}

impl Block {
    pub fn new(block_kind: &BlockKind, grid_position: IVec3, direction: &Direction) -> Self {
        let block_shape = BlockKind::get_block_shape(block_kind);

        let block_state = match block_shape {
            BlockShape::Block => BlockState::Block(BlockData::new()),
            BlockShape::DoorLower => BlockState::Door(DoorData::new()),
            BlockShape::DoorUpper => BlockState::Door(DoorData::new()),
            BlockShape::Ladder => BlockState::Ladder,
            BlockShape::Stairs => BlockState::Stairs,
        };

        Self {
            block_kind: block_kind.clone(),
            grid_position,
            direction: direction.clone(),
            block_shape,
            block_state,
        }
    }

    pub fn get_float_box_array(block: &Self) -> &'static [FloatBox] {
        match block.block_shape {
            BlockShape::Block => &BlockShape::BLOCK_SHAPE_ARRAY,

            BlockShape::DoorLower => {
                let BlockState::Door(door_data) = &block.block_state else {
                    panic!("Door is missing its state");
                };

                match block.direction {
                    Direction::North | Direction::South => {
                        if door_data.is_open {
                            &BlockShape::DOOR_OPEN_Y_SHAPE_ARRAY
                        } else {
                            &BlockShape::DOOR_CLOSED_Y_SHAPE_ARRAY
                        }
                    }
                    Direction::West | Direction::East => {
                        if door_data.is_open {
                            &BlockShape::DOOR_OPEN_X_SHAPE_ARRAY
                        } else {
                            &BlockShape::DOOR_CLOSED_X_SHAPE_ARRAY
                        }
                    }
                    _ => unreachable!("Doors should never have Up or Down direction"),
                }
            }

            BlockShape::DoorUpper => {
                let BlockState::Door(door_data) = &block.block_state else {
                    panic!("Door is missing its state");
                };

                match block.direction {
                    Direction::North | Direction::South => {
                        if door_data.is_open {
                            &BlockShape::DOOR_OPEN_Y_SHAPE_ARRAY
                        } else {
                            &BlockShape::DOOR_CLOSED_Y_SHAPE_ARRAY
                        }
                    }
                    Direction::West | Direction::East => {
                        if door_data.is_open {
                            &BlockShape::DOOR_OPEN_X_SHAPE_ARRAY
                        } else {
                            &BlockShape::DOOR_CLOSED_X_SHAPE_ARRAY
                        }
                    }
                    _ => unreachable!("Doors should never have Up or Down direction"),
                }
            }

            BlockShape::Ladder => match block.direction {
                Direction::North => &BlockShape::LADDER_NORTH_SHAPE_ARRAY,
                Direction::West => &BlockShape::LADDER_WEST_SHAPE_ARRAY,
                Direction::South => &BlockShape::LADDER_SOUTH_SHAPE_ARRAY,
                Direction::East => &BlockShape::LADDER_EAST_SHAPE_ARRAY,
                _ => unreachable!("Ladders should never have Up or Down direction"),
            },

            BlockShape::Stairs => match block.direction {
                Direction::North => &BlockShape::STAIRS_NORTH_SHAPE_ARRAY,
                Direction::West => &BlockShape::STAIRS_WEST_SHAPE_ARRAY,
                Direction::South => &BlockShape::STAIRS_SOUTH_SHAPE_ARRAY,
                Direction::East => &BlockShape::STAIRS_EAST_SHAPE_ARRAY,
                _ => unreachable!("Stairs should never have Up or Down direction"),
            },
        }
    }
}
