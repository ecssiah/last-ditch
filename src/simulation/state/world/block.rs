pub mod info;
pub mod kind;

pub use info::Info;
pub use kind::Kind;

use crate::simulation::state::world::block;

pub fn get_info(block_kind: block::Kind) -> block::Info {
    match block_kind {
        Kind::None => Info { solid: false },
        Kind::EngravedStone1 => Info { solid: true },
        Kind::EngravedStone2 => Info { solid: true },
        Kind::Stone1 => Info { solid: true },
        Kind::Stone2 => Info { solid: true },
        Kind::PolishedStone1 => Info { solid: true },
        Kind::PolishedStone2 => Info { solid: true },
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
        Kind::ServerBlock1 => Info { solid: true },
        Kind::ServerBlock2 => Info { solid: true },
        Kind::ServerBlock3 => Info { solid: true },
        Kind::EngravedMetal1 => Info { solid: true },
        Kind::Metal1 => Info { solid: true },
        Kind::Metal2 => Info { solid: true },
        Kind::Metal3 => Info { solid: true },
        Kind::SupportBeam1 => Info { solid: true },
        Kind::Crate1 => Info { solid: true },
        Kind::Fan1 => Info { solid: true },
        Kind::Vent1 => Info { solid: true },
        Kind::Vent2 => Info { solid: true },
    }
}
