use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
pub enum Kind {
    Air,
    Wood,
    Metal,
    Concrete,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub color: (f32, f32, f32, f32),
}
