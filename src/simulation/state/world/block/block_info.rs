use crate::simulation::state::world::block::BlockKind;
use strum::EnumCount;

#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub block_kind: BlockKind,
    pub solid: bool,
}

pub static BLOCK_INFO_ARRAY: [BlockInfo; BlockKind::COUNT] = [
    BlockInfo {
        block_kind: BlockKind::Carved1,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Carved2,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Carved3,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Carved4,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Caution1,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Caution2,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Caution3,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Caution4,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::CardinalEast,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::CardinalNorth,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::CardinalSouth,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::CardinalWest,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::EagleStone,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::EagleSymbol,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Engraved1,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Engraved2,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Engraved3,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Engraved4,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::HorseStone,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::HorseSymbol,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::LionStone,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::LionSymbol,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Metal1,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Metal2,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Metal3,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Metal4,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Ornate1,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Ornate2,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Ornate3,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Ornate4,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Panel1,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Panel2,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Panel3,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Panel4,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Server1,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Server2,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Server3,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Server4,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Stone1,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Stone2,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Stone3,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Stone4,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Vent1,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Vent2,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Vent3,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::Vent4,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::WolfStone,
        solid: true,
    },
    BlockInfo {
        block_kind: BlockKind::WolfSymbol,
        solid: true,
    },
];
