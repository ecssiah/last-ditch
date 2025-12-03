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
    EsayaBlock,
}

impl Kind {
    pub const CYCLE: &'static [Kind] = &[
        Kind::Engraved1,
        Kind::Engraved2,
        Kind::Stone1,
        Kind::Stone2,
        Kind::Polished1,
        Kind::Polished2,
        Kind::LionStone,
        Kind::EagleStone,
        Kind::WolfStone,
        Kind::HorseStone,
        Kind::Lion,
        Kind::Eagle,
        Kind::Wolf,
        Kind::Horse,
        Kind::NorthBlock,
        Kind::WestBlock,
        Kind::SouthBlock,
        Kind::EastBlock,
        Kind::EsayaBlock,
    ];

    pub fn next_block_kind(current_block_kind: &Kind) -> Kind {
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

    pub fn previous_block_kind(current_block_kind: &Kind) -> Kind {
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
