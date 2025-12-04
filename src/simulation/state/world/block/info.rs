use crate::simulation::state::world::block;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Info {
    pub solid: bool,
}

impl Info {
    #[rustfmt::skip]
    pub fn setup() -> HashMap<block::Kind, Self> {
        HashMap::from([
            (
                block::Kind::None,
                Self {
                    solid: false
                },
            ),
            (
                block::Kind::Engraved1,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::Engraved2,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::Stone1,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::Stone2,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::Polished1,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::Polished2,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::LionStone,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::EagleStone,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::WolfStone,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::HorseStone,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::Lion,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::Eagle,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::Wolf,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::Horse,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::NorthBlock,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::WestBlock,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::SouthBlock,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::EastBlock,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::ServerBlock1,
                Self {
                    solid: true,
                },
            ),
            (
                block::Kind::ServerBlock2,
                Self {
                    solid: true,
                },
            ),
        ])
    }
}
