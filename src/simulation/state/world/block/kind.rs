#[repr(u16)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    Boundary,
    Engraved1,
    Engraved2,
    Engraved3,
    Engraved4,
    Ornate1,
    Ornate2,
    Ornate3,
    Ornate4,
    CarvedStone1,
    CarvedStone2,
    CarvedStone3,
    CarvedStone4,
    Stone1,
    Stone2,
    Stone3,
    Stone4,
    Lion,
    Eagle,
    Wolf,
    Horse,
    LionStone,
    EagleStone,
    WolfStone,
    HorseStone,
    EastBlock,
    WestBlock,
    NorthBlock,
    SouthBlock,
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
    Vent1,
    Vent2,
    Vent3,
    Vent4,
    Caution,
}

impl Kind {
    pub const SELECTABLE: &'static [Self] = &[
        Self::Engraved1,
        Self::Engraved2,
        Self::Engraved3,
        Self::Engraved4,
        Self::Ornate1,
        Self::Ornate2,
        Self::Ornate3,
        Self::Ornate4,
        Self::CarvedStone1,
        Self::CarvedStone2,
        Self::CarvedStone3,
        Self::CarvedStone4,
        Self::Stone1,
        Self::Stone2,
        Self::Stone3,
        Self::Stone4,
        Self::Lion,
        Self::Eagle,
        Self::Wolf,
        Self::Horse,
        Self::LionStone,
        Self::EagleStone,
        Self::WolfStone,
        Self::HorseStone,
        Self::EastBlock,
        Self::WestBlock,
        Self::NorthBlock,
        Self::SouthBlock,
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
        Self::Caution,
    ];

    pub fn next_block_kind(current_block_kind: &Self) -> Self {
        let current_block_kind_index = Self::SELECTABLE
            .iter()
            .position(|block_kind| block_kind == current_block_kind);

        match current_block_kind_index {
            Some(index) => {
                if index + 1 < Self::SELECTABLE.len() {
                    Self::SELECTABLE[index + 1]
                } else {
                    Self::SELECTABLE[0]
                }
            }
            None => Self::SELECTABLE[0],
        }
    }

    pub fn previous_block_kind(current_block_kind: &Self) -> Self {
        let current_block_kind_index = Self::SELECTABLE
            .iter()
            .position(|block_kind| block_kind == current_block_kind);

        match current_block_kind_index {
            Some(0) => Self::SELECTABLE[Self::SELECTABLE.len() - 1],
            Some(index) => Self::SELECTABLE[index - 1],
            None => Self::SELECTABLE[Self::SELECTABLE.len() - 1],
        }
    }
}
