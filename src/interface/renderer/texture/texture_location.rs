#[derive(Clone, PartialEq)]
pub struct TextureLocation {
    pub atlas_index: usize,
    pub layer_index: usize,
}

impl TextureLocation {
    pub fn new(atlas_index: usize, layer_index: usize) -> Self {
        Self {
            atlas_index,
            layer_index,
        }
    }
}
