use wgpu::Color;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Kind {
    Air,
    Metal,
    Concrete,
}

#[derive(Debug)]
pub struct Block {
    pub kind: Kind,
    pub opacity: f32,
    pub color: Color,
}
