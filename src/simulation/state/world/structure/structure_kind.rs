use strum_macros::{Display, EnumCount, EnumIter, EnumString, VariantArray};

#[repr(u16)]
#[derive(
    Clone,
    Debug,
    Display,
    EnumCount,
    EnumIter,
    EnumString,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    VariantArray,
)]
pub enum StructureKind {
    Door1,
    Ladder1,
    Stairs1,
}
