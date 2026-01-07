#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextureLayerIndex(u32);

impl TextureLayerIndex {
    pub fn new(layer_index: u32) -> Self {
        Self(layer_index)
    }
}

impl From<TextureLayerIndex> for u32 {
    fn from(index: TextureLayerIndex) -> Self {
        index.0
    }
}
