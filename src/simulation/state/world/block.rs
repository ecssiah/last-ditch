pub mod info;
pub mod kind;

pub use info::Info;
pub use kind::Kind;

use crate::simulation::state::world::block;

pub fn get_info(block_kind: block::Kind) -> block::Info {
    match block_kind {
        Kind::None => Info { solid: false },
        Kind::Engraved1 => Info { solid: true },
        Kind::Engraved2 => Info { solid: true },
        Kind::Engraved3 => Info { solid: true },
        Kind::Engraved4 => Info { solid: true },
        Kind::Ornate1 => Info { solid: true },
        Kind::Ornate2 => Info { solid: true },
        Kind::CarvedStone1 => Info { solid: true },
        Kind::CarvedStone2 => Info { solid: true },
        Kind::CarvedStone3 => Info { solid: true },
        Kind::CarvedStone4 => Info { solid: true },
        Kind::Stone1 => Info { solid: true },
        Kind::Stone2 => Info { solid: true },
        Kind::Stone3 => Info { solid: true },
        Kind::Stone4 => Info { solid: true },
        Kind::Lion => Info { solid: true },
        Kind::Eagle => Info { solid: true },
        Kind::Wolf => Info { solid: true },
        Kind::Horse => Info { solid: true },
        Kind::LionStone => Info { solid: true },
        Kind::EagleStone => Info { solid: true },
        Kind::WolfStone => Info { solid: true },
        Kind::HorseStone => Info { solid: true },
        Kind::EastBlock => Info { solid: true },
        Kind::WestBlock => Info { solid: true },
        Kind::NorthBlock => Info { solid: true },
        Kind::SouthBlock => Info { solid: true },
        Kind::Server1 => Info { solid: true },
        Kind::Server2 => Info { solid: true },
        Kind::Server3 => Info { solid: true },
        Kind::Server4 => Info { solid: true },
        Kind::Metal1 => Info { solid: true },
        Kind::Metal2 => Info { solid: true },
        Kind::Metal3 => Info { solid: true },
        Kind::Metal4 => Info { solid: true },
        Kind::Panel1 => Info { solid: true },
        Kind::Panel2 => Info { solid: true },
        Kind::Panel3 => Info { solid: true },
        Kind::Vent1 => Info { solid: true },
        Kind::Vent2 => Info { solid: true },
        Kind::Vent3 => Info { solid: true },
        Kind::Vent4 => Info { solid: true },
        Kind::Caution => Info { solid: true },
    }
}
