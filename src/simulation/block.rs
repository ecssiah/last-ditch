use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
pub enum Kind {
    Air,
    Metal,
    Concrete,
    Wood,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub color: (f32, f32, f32, f32),
}
