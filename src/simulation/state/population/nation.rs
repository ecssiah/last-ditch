pub mod kind;
pub mod leadership;

pub use kind::Kind;
pub use leadership::Leadership;

use ultraviolet::IVec3;

use crate::simulation::state::world::block;

#[derive(Clone)]
pub struct Nation {
    pub nation_kind: self::Kind,
    pub home_grid_position: IVec3,
    pub leadership: Leadership,
}

impl Nation {
    pub fn new(nation_kind: self::Kind) -> Self {
        Self {
            nation_kind,
            home_grid_position: IVec3::zero(),
            leadership: Leadership::default(),
        }
    }

    pub fn get_symbol_block_kind(nation_kind: &Kind) -> block::Kind {
        match nation_kind {
            Kind::Lion => block::Kind::Lion,
            Kind::Eagle => block::Kind::Eagle,
            Kind::Horse => block::Kind::Horse,
            Kind::Wolf => block::Kind::Wolf,
        }
    }

    pub fn get_material_block_kind(nation_kind: &Kind) -> block::Kind {
        match nation_kind {
            Kind::Lion => block::Kind::LionStone,
            Kind::Eagle => block::Kind::EagleStone,
            Kind::Horse => block::Kind::HorseStone,
            Kind::Wolf => block::Kind::WolfStone,
        }
    }

    pub fn get_color(nation_kind: &Kind) -> [f32; 4] {
        match nation_kind {
            Kind::Lion => [0.70, 0.55, 0.85, 1.0],
            Kind::Eagle => [0.65, 0.70, 0.80, 1.0],
            Kind::Horse => [0.988, 0.863, 0.592, 1.0],
            Kind::Wolf => [0.85, 0.35, 0.35, 1.0],
        }
    }
}
