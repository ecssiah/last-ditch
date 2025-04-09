use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Kind {
    Air,
    Metal1,
    Metal2,
    Metal3,
    Metal4,
    Metal5,
    Metal6,
}
