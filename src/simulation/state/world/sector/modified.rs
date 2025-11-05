pub struct Modified {
    pub cell: bool,
    pub boundary: bool,
}

impl Modified {
    pub fn new() -> Self {
        Self {
            cell: false,
            boundary: false,
        }
    }
}
