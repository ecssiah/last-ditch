use crate::simulation::state::world::block;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Meta {
    pub solid: bool,
}

impl Meta {
    #[rustfmt::skip]
    pub fn setup() -> HashMap<block::Kind, Meta> {
        HashMap::from([
            (
                block::Kind::Empty,
                Meta {
                    solid: false
                },
            ),
            (
                block::Kind::Engraved1,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::Engraved2,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::Stone1,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::Stone2,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::Polished1,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::Polished2,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::MagentaStone,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::PurpleStone,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::TealStone,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::CrimsonStone,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::Icon1,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::Icon2,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::Icon3,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::Icon4,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::North,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::West,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::South,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::East,
                Meta {
                    solid: true,
                },
            ),
            (
                block::Kind::EsayaBlock,
                Meta {
                    solid: true,
                },
            ),
        ])
    }
}
