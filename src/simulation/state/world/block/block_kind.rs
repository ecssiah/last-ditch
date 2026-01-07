use crate::{
    simulation::state::world::{
        block::{
            block_shape::*,
            block_state::{
                block_type::BlockType, door_data::DoorData, door_part::DoorPart, BlockState,
            },
        },
        grid::{direction_set::DirectionSet, Direction},
    },
    utils::ldmath::FloatBox,
};
use strum::{EnumCount, VariantArray};
use strum_macros::{Display, EnumCount, EnumIter, EnumString, VariantArray};

#[repr(u16)]
#[derive(
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIter,
    EnumString,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    VariantArray,
)]
pub enum BlockKind {
    Carved1,
    Carved2,
    Carved3,
    Carved4,
    Caution1,
    Caution2,
    Caution3,
    Caution4,
    Door1,
    EagleStone,
    EagleSymbol,
    CardinalEast,
    CardinalNorth,
    CardinalSouth,
    CardinalWest,
    Engraved1,
    Engraved2,
    Engraved3,
    Engraved4,
    HorseStone,
    HorseSymbol,
    Ladder1,
    LionStone,
    LionSymbol,
    Metal1,
    Metal2,
    Metal3,
    Metal4,
    Ornate1,
    Ornate2,
    Ornate3,
    Ornate4,
    Panel1,
    Panel2,
    Panel3,
    Panel4,
    Server1,
    Server2,
    Server3,
    Server4,
    Stairs1,
    Stone1,
    Stone2,
    Stone3,
    Stone4,
    Vent1,
    Vent2,
    Vent3,
    Vent4,
    WolfStone,
    WolfSymbol,
}

impl BlockKind {
    pub fn to_discriminant(block_kind: &Self) -> u16 {
        (block_kind.clone()) as u16
    }

    pub fn get_next_block_kind(block_kind: &Self) -> Self {
        let discriminant = Self::to_discriminant(block_kind) as usize;

        Self::VARIANTS[(discriminant + 1) % Self::VARIANTS.len()].clone()
    }

    pub fn get_previous_block_kind(block_kind: &Self) -> Self {
        let discriminant = Self::to_discriminant(block_kind) as usize;

        Self::VARIANTS[(discriminant + Self::COUNT - 1) % BlockKind::COUNT].clone()
    }

    pub fn get_default_block_state(block_kind: &Self) -> BlockState {
        match block_kind {
            BlockKind::Carved1
            | BlockKind::Carved2
            | BlockKind::Carved3
            | BlockKind::Carved4
            | BlockKind::Caution1
            | BlockKind::Caution2
            | BlockKind::Caution3
            | BlockKind::Caution4
            | BlockKind::EagleStone
            | BlockKind::EagleSymbol
            | BlockKind::CardinalEast
            | BlockKind::CardinalNorth
            | BlockKind::CardinalSouth
            | BlockKind::CardinalWest
            | BlockKind::Engraved1
            | BlockKind::Engraved2
            | BlockKind::Engraved3
            | BlockKind::Engraved4
            | BlockKind::HorseStone
            | BlockKind::HorseSymbol
            | BlockKind::LionStone
            | BlockKind::LionSymbol
            | BlockKind::Metal1
            | BlockKind::Metal2
            | BlockKind::Metal3
            | BlockKind::Metal4
            | BlockKind::Ornate1
            | BlockKind::Ornate2
            | BlockKind::Ornate3
            | BlockKind::Ornate4
            | BlockKind::Panel1
            | BlockKind::Panel2
            | BlockKind::Panel3
            | BlockKind::Panel4
            | BlockKind::Server1
            | BlockKind::Server2
            | BlockKind::Server3
            | BlockKind::Server4
            | BlockKind::Stone1
            | BlockKind::Stone2
            | BlockKind::Stone3
            | BlockKind::Stone4
            | BlockKind::Vent1
            | BlockKind::Vent2
            | BlockKind::Vent3
            | BlockKind::Vent4
            | BlockKind::WolfStone
            | BlockKind::WolfSymbol => BlockState {
                direction: Direction::North,
                exposure_set: DirectionSet::ALL,
                block_type: BlockType::Block,
            },

            BlockKind::Door1 => BlockState {
                direction: Direction::North,
                exposure_set: DirectionSet::ALL,
                block_type: BlockType::Door(DoorData {
                    is_open: false,
                    door_part: DoorPart::Lower,
                }),
            },

            BlockKind::Ladder1 => BlockState {
                direction: Direction::North,
                exposure_set: DirectionSet::ALL,
                block_type: BlockType::Ladder,
            },

            BlockKind::Stairs1 => BlockState {
                direction: Direction::North,
                exposure_set: DirectionSet::ALL,
                block_type: BlockType::Stairs,
            },
        }
    }

    pub fn is_direction_occluded(
        direction: &Direction,
        block_state: &BlockState,
        block_kind: &Self,
    ) -> bool {
        match block_kind {
            BlockKind::Carved1
            | BlockKind::Carved2
            | BlockKind::Carved3
            | BlockKind::Carved4
            | BlockKind::Caution1
            | BlockKind::Caution2
            | BlockKind::Caution3
            | BlockKind::Caution4
            | BlockKind::EagleStone
            | BlockKind::EagleSymbol
            | BlockKind::CardinalEast
            | BlockKind::CardinalNorth
            | BlockKind::CardinalSouth
            | BlockKind::CardinalWest
            | BlockKind::Engraved1
            | BlockKind::Engraved2
            | BlockKind::Engraved3
            | BlockKind::Engraved4
            | BlockKind::HorseStone
            | BlockKind::HorseSymbol
            | BlockKind::LionStone
            | BlockKind::LionSymbol
            | BlockKind::Metal1
            | BlockKind::Metal2
            | BlockKind::Metal3
            | BlockKind::Metal4
            | BlockKind::Ornate1
            | BlockKind::Ornate2
            | BlockKind::Ornate3
            | BlockKind::Ornate4
            | BlockKind::Panel1
            | BlockKind::Panel2
            | BlockKind::Panel3
            | BlockKind::Panel4
            | BlockKind::Server1
            | BlockKind::Server2
            | BlockKind::Server3
            | BlockKind::Server4
            | BlockKind::Stone1
            | BlockKind::Stone2
            | BlockKind::Stone3
            | BlockKind::Stone4
            | BlockKind::Vent1
            | BlockKind::Vent2
            | BlockKind::Vent3
            | BlockKind::Vent4
            | BlockKind::WolfStone
            | BlockKind::WolfSymbol => true,

            BlockKind::Door1 => false,

            BlockKind::Ladder1 => false,

            BlockKind::Stairs1 => {
                if direction == &Direction::Down {
                    true
                } else if direction == &block_state.direction {
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn get_collider_shape_array(
        block_state: &BlockState,
        block_kind: &Self,
    ) -> &'static [FloatBox] {
        match block_kind {
            BlockKind::Carved1
            | BlockKind::Carved2
            | BlockKind::Carved3
            | BlockKind::Carved4
            | BlockKind::Caution1
            | BlockKind::Caution2
            | BlockKind::Caution3
            | BlockKind::Caution4
            | BlockKind::EagleStone
            | BlockKind::EagleSymbol
            | BlockKind::CardinalEast
            | BlockKind::CardinalNorth
            | BlockKind::CardinalSouth
            | BlockKind::CardinalWest
            | BlockKind::Engraved1
            | BlockKind::Engraved2
            | BlockKind::Engraved3
            | BlockKind::Engraved4
            | BlockKind::HorseStone
            | BlockKind::HorseSymbol
            | BlockKind::LionStone
            | BlockKind::LionSymbol
            | BlockKind::Metal1
            | BlockKind::Metal2
            | BlockKind::Metal3
            | BlockKind::Metal4
            | BlockKind::Ornate1
            | BlockKind::Ornate2
            | BlockKind::Ornate3
            | BlockKind::Ornate4
            | BlockKind::Panel1
            | BlockKind::Panel2
            | BlockKind::Panel3
            | BlockKind::Panel4
            | BlockKind::Server1
            | BlockKind::Server2
            | BlockKind::Server3
            | BlockKind::Server4
            | BlockKind::Stone1
            | BlockKind::Stone2
            | BlockKind::Stone3
            | BlockKind::Stone4
            | BlockKind::Vent1
            | BlockKind::Vent2
            | BlockKind::Vent3
            | BlockKind::Vent4
            | BlockKind::WolfStone
            | BlockKind::WolfSymbol => &BLOCK_SHAPE_ARRAY,

            BlockKind::Door1 => {
                let BlockType::Door(door_data) = &block_state.block_type else {
                    panic!("Door is missing its data");
                };

                match block_state.direction {
                    Direction::North | Direction::South => {
                        if door_data.is_open {
                            &DOOR_OPEN_Y_SHAPE_ARRAY
                        } else {
                            &DOOR_CLOSED_Y_SHAPE_ARRAY
                        }
                    }
                    Direction::West | Direction::East => {
                        if door_data.is_open {
                            &DOOR_OPEN_X_SHAPE_ARRAY
                        } else {
                            &DOOR_CLOSED_X_SHAPE_ARRAY
                        }
                    }
                    _ => unreachable!("Doors should never have Up or Down direction"),
                }
            }

            BlockKind::Ladder1 => match block_state.direction {
                Direction::North => &LADDER_NORTH_SHAPE_ARRAY,
                Direction::West => &LADDER_WEST_SHAPE_ARRAY,
                Direction::South => &LADDER_SOUTH_SHAPE_ARRAY,
                Direction::East => &LADDER_EAST_SHAPE_ARRAY,
                _ => unreachable!("Ladders should never have Up or Down direction"),
            },

            BlockKind::Stairs1 => match block_state.direction {
                Direction::North => &STAIRS_NORTH_SHAPE_ARRAY,
                Direction::West => &STAIRS_WEST_SHAPE_ARRAY,
                Direction::South => &STAIRS_SOUTH_SHAPE_ARRAY,
                Direction::East => &STAIRS_EAST_SHAPE_ARRAY,
                _ => unreachable!("Stairs should never have Up or Down direction"),
            },
        }
    }
}
