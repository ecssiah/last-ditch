use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    Air,
    Engraved1,
    Engraved2,
    Stone1,
    Stone2,
}
