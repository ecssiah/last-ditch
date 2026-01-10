use crate::simulation::state::world::block::block_shape::BlockShape;
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
    CardinalEast,
    CardinalNorth,
    CardinalSouth,
    CardinalWest,
    Carved1,
    Carved2,
    Carved3,
    Carved4,
    Caution1,
    Caution2,
    Caution3,
    Caution4,
    DoorLower1,
    DoorUpper1,
    EagleStone,
    EagleSymbol,
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
    Metal5,
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
    Smooth1,
    Smooth2,
    Smooth3,
    Smooth4,
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

    pub fn get_block_shape(block_kind: &Self) -> BlockShape {
        match block_kind {
            BlockKind::CardinalEast
            | BlockKind::CardinalNorth
            | BlockKind::CardinalSouth
            | BlockKind::CardinalWest
            | BlockKind::Carved1
            | BlockKind::Carved2
            | BlockKind::Carved3
            | BlockKind::Carved4
            | BlockKind::Caution1
            | BlockKind::Caution2
            | BlockKind::Caution3
            | BlockKind::Caution4
            | BlockKind::EagleStone
            | BlockKind::EagleSymbol
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
            | BlockKind::Metal5
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
            | BlockKind::Smooth1
            | BlockKind::Smooth2
            | BlockKind::Smooth3
            | BlockKind::Smooth4
            | BlockKind::Vent1
            | BlockKind::Vent2
            | BlockKind::Vent3
            | BlockKind::Vent4
            | BlockKind::WolfStone
            | BlockKind::WolfSymbol => BlockShape::Block,

            BlockKind::DoorUpper1 => BlockShape::DoorUpper,

            BlockKind::DoorLower1 => BlockShape::DoorLower,

            BlockKind::Ladder1 => BlockShape::Ladder,

            BlockKind::Stairs1 => BlockShape::Stairs,
        }
    }
}
