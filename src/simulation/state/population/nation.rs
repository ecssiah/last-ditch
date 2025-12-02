pub mod kind;

pub use kind::Kind;

use crate::simulation::state::world::block;
use ultraviolet::IVec3;

#[derive(Clone)]
pub struct Nation {
    pub home_position: IVec3,
}

impl Nation {
    pub fn block(nation_kind: &Kind) -> block::Kind {
        match nation_kind {
            Kind::Eagle => block::Kind::Eagle,
            Kind::Lion => block::Kind::Lion,
            Kind::Horse => block::Kind::Horse,
            Kind::Wolf => block::Kind::Wolf,
        }
    }

    pub fn color(nation_kind: &Kind) -> [f32; 4] {
        match nation_kind {
            Kind::Eagle => [0.65, 0.70, 0.80, 1.0],
            Kind::Lion => [0.70, 0.55, 0.85, 1.0],
            Kind::Horse => [0.988, 0.863, 0.592, 1.0],
            Kind::Wolf => [0.85, 0.35, 0.35, 1.0],
        }
    }
}
