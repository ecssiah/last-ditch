use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
pub enum BlockKind {
    Air,
    Wood,
    Metal,
    Concrete,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: BlockKind,
    pub opacity: f32,
    pub color: (f32, f32, f32, f32),
}
