pub struct Modified {
    pub block: bool,
    pub boundary: bool,
}

impl Modified {
    pub fn new() -> Self {
        Self {
            block: false,
            boundary: false,
        }
    }
}
