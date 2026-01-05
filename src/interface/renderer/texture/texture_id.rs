#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextureID(u32);

impl TextureID {
    pub fn new(id_value: u32) -> Self {
        Self(id_value)
    }
}
