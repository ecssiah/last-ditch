use crate::simulation::state::world::cell;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Info {
    pub solid: bool,
}

impl Info {
    #[rustfmt::skip]
    pub fn setup() -> HashMap<cell::Kind, Info> {
        HashMap::from([
            (
                cell::Kind::Empty,
                Info {
                    solid: false
                },
            ),
            (
                cell::Kind::Engraved1,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::Engraved2,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::Stone1,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::Stone2,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::Polished1,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::Polished2,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::MagentaStone,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::PurpleStone,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::TealStone,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::CrimsonStone,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::Icon1,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::Icon2,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::Icon3,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::Icon4,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::North,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::West,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::South,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::East,
                Info {
                    solid: true,
                },
            ),
            (
                cell::Kind::EsayaBlock,
                Info {
                    solid: true,
                },
            ),
        ])
    }
}
