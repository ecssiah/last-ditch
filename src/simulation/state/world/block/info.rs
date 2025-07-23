use crate::simulation::state::world::block;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Info {
    pub solid: bool,
}

impl Info {
    #[rustfmt::skip]
    pub fn setup() -> HashMap<block::Kind, Info> {
        HashMap::from([
            (
                block::Kind::Empty,
                Info {
                    solid: false
                },
            ),
            (
                block::Kind::Engraved1,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::Engraved2,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::Stone1,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::Stone2,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::Polished1,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::Polished2,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::MagentaStone,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::PurpleStone,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::TealStone,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::CrimsonStone,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::Icon1,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::Icon2,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::Icon3,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::Icon4,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::North,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::West,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::South,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::East,
                Info {
                    solid: true,
                },
            ),
            (
                block::Kind::EsayaBlock,
                Info {
                    solid: true,
                },
            ),
        ])
    }
}
