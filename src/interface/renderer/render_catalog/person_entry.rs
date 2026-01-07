#[derive(Clone)]
pub struct PersonEntry {
    pub texture_name: &'static str,
}

impl PersonEntry {
    pub fn new(texture_name: &'static str) -> Self {
        Self { texture_name }
    }
}
