#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerIndex(u32);

impl LayerIndex {
    pub fn new(layer_index: u32) -> Self {
        Self(layer_index)
    }
}

impl From<LayerIndex> for u32 {
    fn from(layer_index: LayerIndex) -> Self {
        layer_index.0
    }
}
