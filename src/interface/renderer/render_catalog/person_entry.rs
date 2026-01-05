#[derive(Clone)]
pub struct PersonEntry {
    pub model: &'static str,
}

impl PersonEntry {
    pub fn new(model: &'static str) -> Self {
        Self { model }
    }
}
