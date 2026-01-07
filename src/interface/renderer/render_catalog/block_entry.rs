#[derive(Clone)]
pub struct BlockEntry {
    pub texture_name: &'static str,
}

impl BlockEntry {
    pub fn new(texture_name: &'static str) -> Self {
        Self { texture_name }
    }
}
