#[repr(u16)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    None,
    Engraved1,
    Engraved2,
    Stone1,
    Stone2,
    Polished1,
    Polished2,
    LionStone,
    EagleStone,
    WolfStone,
    HorseStone,
    Lion,
    Eagle,
    Wolf,
    Horse,
    NorthBlock,
    WestBlock,
    SouthBlock,
    EastBlock,
    ServerBlock1,
    ServerBlock2,
}

impl Kind {
    pub const CYCLE: &'static [Self] = &[
        Self::Engraved1,
        Self::Engraved2,
        Self::Stone1,
        Self::Stone2,
        Self::Polished1,
        Self::Polished2,
        Self::LionStone,
        Self::EagleStone,
        Self::WolfStone,
        Self::HorseStone,
        Self::Lion,
        Self::Eagle,
        Self::Wolf,
        Self::Horse,
        Self::NorthBlock,
        Self::WestBlock,
        Self::SouthBlock,
        Self::EastBlock,
        Self::ServerBlock1,
        Self::ServerBlock2,
    ];

    pub fn next_block_kind(current_block_kind: &Self) -> Self {
        let current_block_kind_index = Self::CYCLE
            .iter()
            .position(|block_kind| block_kind == current_block_kind);

        match current_block_kind_index {
            Some(index) => {
                if index + 1 < Self::CYCLE.len() {
                    Self::CYCLE[index + 1]
                } else {
                    Self::CYCLE[0]
                }
            }
            None => Self::CYCLE[0],
        }
    }

    pub fn previous_block_kind(current_block_kind: &Self) -> Self {
        let current_block_kind_index = Self::CYCLE
            .iter()
            .position(|block_kind| block_kind == current_block_kind);

        match current_block_kind_index {
            Some(0) => Self::CYCLE[Self::CYCLE.len() - 1],
            Some(index) => Self::CYCLE[index - 1],
            None => Self::CYCLE[Self::CYCLE.len() - 1],
        }
    }
}
