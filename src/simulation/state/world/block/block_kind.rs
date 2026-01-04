use std::fmt;
use strum_macros::{EnumCount, EnumIter};

#[repr(u16)]
#[derive(Clone, Debug, EnumCount, EnumIter, Hash, PartialEq, Eq, PartialOrd)]
pub enum BlockKind {
    Engraved1,
    Engraved2,
    Engraved3,
    Engraved4,
    Ornate1,
    Ornate2,
    Ornate3,
    Ornate4,
    Carved1,
    Carved2,
    Carved3,
    Carved4,
    Stone1,
    Stone2,
    Stone3,
    Stone4,
    LionSymbol,
    EagleSymbol,
    HorseSymbol,
    WolfSymbol,
    LionStone,
    EagleStone,
    HorseStone,
    WolfStone,
    NorthBlock,
    WestBlock,
    SouthBlock,
    EastBlock,
    Server1,
    Server2,
    Server3,
    Server4,
    Metal1,
    Metal2,
    Metal3,
    Metal4,
    Panel1,
    Panel2,
    Panel3,
    Panel4,
    Vent1,
    Vent2,
    Vent3,
    Vent4,
    Caution1,
    Caution2,
    Caution3,
    Caution4,
    Platform1,
    Stairs1,
    Ladder1,
    Door1,
}

impl BlockKind {
    pub fn to_index(block_kind: &BlockKind) -> usize {
        block_kind.clone() as usize
    }

    pub const SELECTABLE: &'static [Self] = &[
        Self::Engraved1,
        Self::Engraved2,
        Self::Engraved3,
        Self::Engraved4,
        Self::Ornate1,
        Self::Ornate2,
        Self::Ornate3,
        Self::Ornate4,
        Self::Carved1,
        Self::Carved2,
        Self::Carved3,
        Self::Carved4,
        Self::Stone1,
        Self::Stone2,
        Self::Stone3,
        Self::Stone4,
        Self::LionSymbol,
        Self::EagleSymbol,
        Self::HorseSymbol,
        Self::WolfSymbol,
        Self::LionStone,
        Self::EagleStone,
        Self::HorseStone,
        Self::WolfStone,
        Self::NorthBlock,
        Self::WestBlock,
        Self::SouthBlock,
        Self::EastBlock,
        Self::Server1,
        Self::Server2,
        Self::Server3,
        Self::Server4,
        Self::Metal1,
        Self::Metal2,
        Self::Metal3,
        Self::Metal4,
        Self::Panel1,
        Self::Panel2,
        Self::Panel3,
        Self::Vent1,
        Self::Vent2,
        Self::Vent3,
        Self::Vent4,
        Self::Caution1,
        Self::Caution2,
        Self::Caution3,
        Self::Caution4,
        Self::Platform1,
        Self::Stairs1,
        Self::Ladder1,
        Self::Door1,
    ];

    pub fn next_block_kind(current_block_kind: &Self) -> Self {
        let current_block_kind_index = Self::SELECTABLE
            .iter()
            .position(|block_kind| block_kind == current_block_kind);

        match current_block_kind_index {
            Some(index) => {
                if index + 1 < Self::SELECTABLE.len() {
                    Self::SELECTABLE[index + 1].clone()
                } else {
                    Self::SELECTABLE[0].clone()
                }
            }
            None => Self::SELECTABLE[0].clone(),
        }
    }

    pub fn previous_block_kind(current_block_kind: &Self) -> Self {
        let current_block_kind_index = Self::SELECTABLE
            .iter()
            .position(|block_kind| block_kind == current_block_kind);

        match current_block_kind_index {
            Some(0) => Self::SELECTABLE[Self::SELECTABLE.len() - 1].clone(),
            Some(index) => Self::SELECTABLE[index - 1].clone(),
            None => Self::SELECTABLE[Self::SELECTABLE.len() - 1].clone(),
        }
    }

    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Engraved1 => "engraved 1",
            Self::Engraved2 => "engraved 2",
            Self::Engraved3 => "engraved 3",
            Self::Engraved4 => "engraved 4",
            Self::Ornate1 => "ornate 1",
            Self::Ornate2 => "ornate 2",
            Self::Ornate3 => "ornate 3",
            Self::Ornate4 => "ornate 4",
            Self::Carved1 => "carved 1",
            Self::Carved2 => "carved 2",
            Self::Carved3 => "carved 3",
            Self::Carved4 => "carved 4",
            Self::Stone1 => "stone 1",
            Self::Stone2 => "stone 2",
            Self::Stone3 => "stone 3",
            Self::Stone4 => "stone 4",
            Self::LionSymbol => "lion symbol",
            Self::EagleSymbol => "eagle symbol",
            Self::HorseSymbol => "horse symbol",
            Self::WolfSymbol => "wolf symbol",
            Self::LionStone => "lion stone",
            Self::EagleStone => "eagle stone",
            Self::HorseStone => "horse stone",
            Self::WolfStone => "wolf stone",
            Self::NorthBlock => "north block",
            Self::WestBlock => "west block",
            Self::SouthBlock => "south block",
            Self::EastBlock => "east block",
            Self::Server1 => "server 1",
            Self::Server2 => "server 2",
            Self::Server3 => "server 3",
            Self::Server4 => "server 4",
            Self::Metal1 => "metal 1",
            Self::Metal2 => "metal 2",
            Self::Metal3 => "metal 3",
            Self::Metal4 => "metal 4",
            Self::Panel1 => "panel 1",
            Self::Panel2 => "panel 2",
            Self::Panel3 => "panel 3",
            Self::Panel4 => "panel 4",
            Self::Vent1 => "vent 1",
            Self::Vent2 => "vent 2",
            Self::Vent3 => "vent 3",
            Self::Vent4 => "vent 4",
            Self::Caution1 => "caution 1",
            Self::Caution2 => "caution 2",
            Self::Caution3 => "caution 3",
            Self::Caution4 => "caution 4",
            Self::Platform1 => "platform1",
            Self::Stairs1 => "stairs1",
            Self::Ladder1 => "ladder1",
            Self::Door1 => "door1",
        }
    }
}

impl fmt::Display for BlockKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
