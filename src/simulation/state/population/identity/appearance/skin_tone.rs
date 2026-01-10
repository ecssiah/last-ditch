use strum_macros::{Display, EnumString};

#[derive(Clone, Display, Hash, PartialEq, Eq, EnumString)]
pub enum SkinTone {
    Person1,
    Person2,
    Person3,
    Person4,
    Person5,
    Person6,
    Person7,
    Person8,
}
