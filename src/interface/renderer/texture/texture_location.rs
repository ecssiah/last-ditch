#[derive(Clone, PartialEq)]
pub struct TextureLocation {
    pub atlas_index: u8,
    pub layer_index: u8,
}

impl TextureLocation {
    pub fn new(atlas_index: u8, layer_index: u8) -> Self {
        Self {
            atlas_index,
            layer_index,
        }
    }
}
