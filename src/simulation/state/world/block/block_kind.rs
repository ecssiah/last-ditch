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

    pub fn next_block_kind(block_kind: &Self) -> Self {
        let discriminant = Self::to_discriminant(block_kind) as usize;

        Self::VARIANTS[(discriminant + 1) % Self::VARIANTS.len()].clone()
    }

    pub fn previous_block_kind(block_kind: &Self) -> Self {
        let discriminant = Self::to_discriminant(block_kind) as usize;

        Self::VARIANTS[(discriminant + Self::COUNT - 1) % BlockKind::COUNT].clone()
    }
}
