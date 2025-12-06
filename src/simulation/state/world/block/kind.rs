#[repr(u16)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    None,
    EngravedStone1,
    EngravedStone2,
    Stone1,
    Stone2,
    PolishedStone1,
    PolishedStone2,
    Lion,
    Eagle,
    Wolf,
    Horse,
    LionStone,
    EagleStone,
    WolfStone,
    HorseStone,
    NorthBlock,
    WestBlock,
    SouthBlock,
    EastBlock,
    ServerBlock1,
    ServerBlock2,
    VentBlock,
    EngravedMetal1,
    Metal1,
    Metal2,
    Metal3,
    Crate,
    Fan1,
}

impl Kind {
    pub const CYCLE: &'static [Self] = &[
        Self::EngravedStone1,
        Self::EngravedStone2,
        Self::Stone1,
        Self::Stone2,
        Self::PolishedStone1,
        Self::PolishedStone2,
        Self::Lion,
        Self::Eagle,
        Self::Wolf,
        Self::Horse,
        Self::LionStone,
        Self::EagleStone,
        Self::WolfStone,
        Self::HorseStone,
        Self::NorthBlock,
        Self::WestBlock,
        Self::SouthBlock,
        Self::EastBlock,
        Self::ServerBlock1,
        Self::ServerBlock2,
        Self::VentBlock,
        Self::EngravedMetal1,
        Self::Metal1,
        Self::Metal2,
        Self::Metal3,
        Self::Crate,
        Self::Fan1,
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
