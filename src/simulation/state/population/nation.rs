pub mod leadership;
pub mod nation_kind;

use crate::simulation::state::{
    population::nation::{leadership::Leadership, nation_kind::NationKind},
    world::block::block_kind::BlockKind,
};
use ultraviolet::IVec3;

#[derive(Clone)]
pub struct Nation {
    pub nation_kind: NationKind,
    pub home_grid_position: IVec3,
    pub leadership: Leadership,
}

impl Nation {
    pub fn new(nation_kind: NationKind) -> Self {
        Self {
            nation_kind,
            home_grid_position: IVec3::zero(),
            leadership: Leadership::default(),
        }
    }

    pub fn get_nation_symbol(nation_kind: &NationKind) -> &BlockKind {
        match nation_kind {
            NationKind::Lion => &BlockKind::LionSymbol,
            NationKind::Eagle => &BlockKind::EagleSymbol,
            NationKind::Horse => &BlockKind::HorseSymbol,
            NationKind::Wolf => &BlockKind::WolfSymbol,
        }
    }

    pub fn get_nation_stone(nation_kind: &NationKind) -> &BlockKind {
        match nation_kind {
            NationKind::Lion => &BlockKind::LionStone,
            NationKind::Eagle => &BlockKind::EagleStone,
            NationKind::Horse => &BlockKind::HorseStone,
            NationKind::Wolf => &BlockKind::WolfStone,
        }
    }

    pub fn get_color(nation_kind: &NationKind) -> [f32; 4] {
        match nation_kind {
            NationKind::Lion => [0.70, 0.55, 0.85, 1.0],
            NationKind::Eagle => [0.65, 0.70, 0.80, 1.0],
            NationKind::Horse => [0.988, 0.863, 0.592, 1.0],
            NationKind::Wolf => [0.85, 0.35, 0.35, 1.0],
        }
    }
}
