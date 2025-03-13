use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, PartialOrd, Ord, Eq, PartialEq)]
pub enum BlockKind {
    Air,
    Wood,
    Metal,
    Concrete,
    Black,
    White,
    Red,
    Blue,
    Gold,
    Skin,
    Green,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: BlockKind,
    pub opacity: f32,
    pub color: (f32, f32, f32, f32),
}
