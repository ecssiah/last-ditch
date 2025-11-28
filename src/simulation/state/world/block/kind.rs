#[repr(u16)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    None,
    Engraved1,
    Engraved2,
    Stone1,
    Stone2,
    Polished1,
    Polished2,
    MagentaStone,
    PurpleStone,
    TealStone,
    CrimsonStone,
    Icon1,
    Icon2,
    Icon3,
    Icon4,
    NorthBlock,
    WestBlock,
    SouthBlock,
    EastBlock,
    EsayaBlock,
}

impl Kind {
    pub const CYCLE: &'static [Kind] = &[
        Kind::Engraved1,
        Kind::Engraved2,
        Kind::Stone1,
        Kind::Stone2,
        Kind::Polished1,
        Kind::Polished2,
        Kind::MagentaStone,
        Kind::PurpleStone,
        Kind::TealStone,
        Kind::CrimsonStone,
        Kind::Icon1,
        Kind::Icon2,
        Kind::Icon3,
        Kind::Icon4,
        Kind::NorthBlock,
        Kind::WestBlock,
        Kind::SouthBlock,
        Kind::EastBlock,
        Kind::EsayaBlock,
    ];

    pub fn next(self) -> Kind {
        let idx = Self::CYCLE.iter().position(|&k| k == self);

        match idx {
            Some(i) => {
                if i + 1 < Self::CYCLE.len() {
                    Self::CYCLE[i + 1]
                } else {
                    Self::CYCLE[0] // wrap
                }
            }
            None => Self::CYCLE[0], // if starting from None or anything weird
        }
    }

    pub fn prev(self) -> Kind {
        let idx = Self::CYCLE.iter().position(|&k| k == self);

        match idx {
            Some(0) => Self::CYCLE[Self::CYCLE.len() - 1], // wrap
            Some(i) => Self::CYCLE[i - 1],
            None => Self::CYCLE[Self::CYCLE.len() - 1],
        }
    }
}
