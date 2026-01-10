use strum_macros::{Display, EnumString};

#[derive(Clone, Display, Debug, EnumString, Hash, PartialEq, Eq)]
pub enum Sex {
    Male,
    Female,
}
