use std::fmt;

#[repr(u16)]
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
pub enum Kind {
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
    Horse,
    Wolf,
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
        Self::Horse,
        Self::Wolf,
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
        Self::Caution,
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
            self::Kind::Engraved1 => "engraved 1",
            self::Kind::Engraved2 => "engraved 2",
            self::Kind::Engraved3 => "engraved 3",
            self::Kind::Engraved4 => "engraved 4",
            self::Kind::Ornate1 => "ornate 1",
            self::Kind::Ornate2 => "ornate 2",
            self::Kind::Ornate3 => "ornate 3",
            self::Kind::Ornate4 => "ornate 4",
            self::Kind::CarvedStone1 => "carved stone 1",
            self::Kind::CarvedStone2 => "carved stone 2",
            self::Kind::CarvedStone3 => "carved stone 3",
            self::Kind::CarvedStone4 => "carved stone 4",
            self::Kind::Stone1 => "stone 1",
            self::Kind::Stone2 => "stone 2",
            self::Kind::Stone3 => "stone 3",
            self::Kind::Stone4 => "stone 4",
            self::Kind::Lion => "lion",
            self::Kind::Eagle => "eagle",
            self::Kind::Horse => "horse",
            self::Kind::Wolf => "wolf",
            self::Kind::LionStone => "lion stone",
            self::Kind::EagleStone => "eagle stone",
            self::Kind::HorseStone => "horse stone",
            self::Kind::WolfStone => "wolf stone",
            self::Kind::NorthBlock => "north block",
            self::Kind::WestBlock => "west block",
            self::Kind::SouthBlock => "south block",
            self::Kind::EastBlock => "east block",
            self::Kind::Server1 => "server 1",
            self::Kind::Server2 => "server 2",
            self::Kind::Server3 => "server 3",
            self::Kind::Server4 => "server 4",
            self::Kind::Metal1 => "metal 1",
            self::Kind::Metal2 => "metal 2",
            self::Kind::Metal3 => "metal 3",
            self::Kind::Metal4 => "metal 4",
            self::Kind::Panel1 => "panel 1",
            self::Kind::Panel2 => "panel 2",
            self::Kind::Panel3 => "panel 3",
            self::Kind::Vent1 => "vent 1",
            self::Kind::Vent2 => "vent 2",
            self::Kind::Vent3 => "vent 3",
            self::Kind::Vent4 => "vent 4",
            self::Kind::Caution => "caution",
        }
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
