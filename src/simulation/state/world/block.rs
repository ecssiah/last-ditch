pub mod info;
pub mod kind;

pub use info::Info;
pub use kind::Kind;

#[derive(Clone, Debug)]
pub struct Block {
    pub block_kind: self::Kind,
    pub solid: bool,
}

impl Block {
    pub fn new(block_kind: self::Kind) -> Self {
        let block_info = Self::get_block_info(block_kind);

        Self {
            block_kind,
            solid: block_info.solid,
        }
    }

    pub const fn get_boundary_block() -> Self {
        Self {
            block_kind: self::Kind::Boundary,
            solid: true,
        }
    }

    pub const fn get_block_info(block_kind: self::Kind) -> self::Info {
        match block_kind {
            self::Kind::Boundary => self::Info { solid: true },
            self::Kind::Engraved1 => self::Info { solid: true },
            self::Kind::Engraved2 => self::Info { solid: true },
            self::Kind::Engraved3 => self::Info { solid: true },
            self::Kind::Engraved4 => self::Info { solid: true },
            self::Kind::Ornate1 => self::Info { solid: true },
            self::Kind::Ornate2 => self::Info { solid: true },
            self::Kind::Ornate3 => self::Info { solid: true },
            self::Kind::Ornate4 => self::Info { solid: true },
            self::Kind::CarvedStone1 => self::Info { solid: true },
            self::Kind::CarvedStone2 => self::Info { solid: true },
            self::Kind::CarvedStone3 => self::Info { solid: true },
            self::Kind::CarvedStone4 => self::Info { solid: true },
            self::Kind::Stone1 => self::Info { solid: true },
            self::Kind::Stone2 => self::Info { solid: true },
            self::Kind::Stone3 => self::Info { solid: true },
            self::Kind::Stone4 => self::Info { solid: true },
            self::Kind::Lion => self::Info { solid: true },
            self::Kind::Eagle => self::Info { solid: true },
            self::Kind::Wolf => self::Info { solid: true },
            self::Kind::Horse => self::Info { solid: true },
            self::Kind::LionStone => self::Info { solid: true },
            self::Kind::EagleStone => self::Info { solid: true },
            self::Kind::WolfStone => self::Info { solid: true },
            self::Kind::HorseStone => self::Info { solid: true },
            self::Kind::EastBlock => self::Info { solid: true },
            self::Kind::WestBlock => self::Info { solid: true },
            self::Kind::NorthBlock => self::Info { solid: true },
            self::Kind::SouthBlock => self::Info { solid: true },
            self::Kind::Server1 => self::Info { solid: true },
            self::Kind::Server2 => self::Info { solid: true },
            self::Kind::Server3 => self::Info { solid: true },
            self::Kind::Server4 => self::Info { solid: true },
            self::Kind::Metal1 => self::Info { solid: true },
            self::Kind::Metal2 => self::Info { solid: true },
            self::Kind::Metal3 => self::Info { solid: true },
            self::Kind::Metal4 => self::Info { solid: true },
            self::Kind::Panel1 => self::Info { solid: true },
            self::Kind::Panel2 => self::Info { solid: true },
            self::Kind::Panel3 => self::Info { solid: true },
            self::Kind::Vent1 => self::Info { solid: true },
            self::Kind::Vent2 => self::Info { solid: true },
            self::Kind::Vent3 => self::Info { solid: true },
            self::Kind::Vent4 => self::Info { solid: true },
            self::Kind::Caution => self::Info { solid: true },
        }
    }
}
