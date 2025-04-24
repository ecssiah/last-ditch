use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    Air,
    Engraved1,
    Engraved2,
    Stone1,
    Stone2,
    Polished1,
    Polished2,
    Origin,
    Icon1,
    Icon2,
    Icon3,
    Icon4,
}
